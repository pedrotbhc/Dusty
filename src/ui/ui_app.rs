use std::io::stdout;
use std::time::Duration;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::lang::AppText;
use crate::dusty::read_dir::FileInfo;
use crate::dusty::{file_size, read_dir};

pub fn show_ui(lang: AppText) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<std::io::Stdout>> = Terminal::new(backend)?;

    let mut selected: usize = 0;
    let mut scroll: usize = 0;

    let mut current_path = std::path::PathBuf::from(".");
    let mut files = Arc::new(Mutex::new(read_dir::read_dir(
        current_path.to_str().unwrap_or(""),
    )));
    let calculating = Arc::new(Mutex::new(HashSet::new()));
    let size_cache: Arc<Mutex<HashMap<PathBuf, u128>>> = Arc::new(Mutex::new(HashMap::new()));

    files.lock().unwrap().insert(
        0,
        FileInfo {
            path: "..".to_string(),
            byte_size: 0,
            human_size: "".to_string(),
            owner_uid: 0,
            owner_gid: 0,
            inode: 0,
            permissions: 0,
            nlinks: 0,
            dir_info: None,
        },
    );

    let mut confirm_delete: Option<usize> = None;
    let mut confirm_delete_ready: bool = false;

    loop {
        let (path, needs_calc) = {
            let files_guard = files.lock().unwrap();
            if let Some(file) = files_guard.get(selected) {
                let path = current_path.join(&file.path);
                let is_dir = std::fs::metadata(&path)
                    .map(|m| m.is_dir())
                    .unwrap_or(false);
                let cache_guard = size_cache.lock().unwrap();
                let calculating_guard = calculating.lock().unwrap();
                let needs = is_dir
                    && !cache_guard.contains_key(&path)
                    && !calculating_guard.contains(&path);
                (path, needs)
            } else {
                continue;
            }
        };

        if needs_calc {
            calculating.lock().unwrap().insert(path.clone());

            let size_cache = Arc::clone(&size_cache);
            let calculating = Arc::clone(&calculating);

            std::thread::spawn(move || {
                let size = read_dir::dir_size_byte(&path);
                size_cache.lock().unwrap().insert(path.clone(), size.into());
                calculating.lock().unwrap().remove(&path);
            });
        }

        let mut max_visible: usize = 0; // definir fora do draw para escopo

        terminal.draw(|f| {
            let size = f.size();

            let full_screen = f.size();
            let chunks_main = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Min(0),
                    Constraint::Length(1),
                ])
                .split(full_screen);

            let content_area = chunks_main[1];

            let block = Block::default()
                .style(Style::default().fg(Color::White).bg(Color::Rgb(11, 32, 39)));

            f.render_widget(&block, size);

            let inner = block.inner(content_area);

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(inner);
            let right_block = Block::default()
                .title(lang.labels.right_area.title.clone())
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White));
            let left_block = Block::default()
                .title(lang.labels.left_area.title.clone())
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White));
            f.render_widget(left_block, chunks[0]);
            f.render_widget(right_block, chunks[1]);

            let spans = vec![
                Span::styled("Dusty", Style::default().bg(Color::Rgb(27, 59, 111))),
                Span::styled(
                    " ".repeat((size.width as usize).saturating_sub(0)),
                    Style::default().bg(Color::Rgb(27, 59, 111)),
                ),
            ];

            let line = Line::from(spans);

            let top_bar_area = Rect {
                x: 0,
                y: 0,
                width: size.width,
                height: 1,
            };

            let top_bar_widget = Paragraph::new(line)
                .style(Style::default())
                .block(Block::default());
            f.render_widget(top_bar_widget, top_bar_area);

            let bottons_top_bar = vec![Span::styled(
                format!("[{}]", lang.status.coming_soon),
                Style::default(),
            )];

            let line2 = Line::from(bottons_top_bar);
            let bottons_top_bar_widget = Paragraph::new(line2)
                .style(Style::default())
                .alignment(Alignment::Right);
            f.render_widget(bottons_top_bar_widget, top_bar_area);

            let bottom_bar_area = Rect {
                x: 0,
                y: size.height.saturating_sub(1),
                width: size.width,
                height: 1,
            };

            let bottom_message = if let Some(idx) = confirm_delete {
                let files_guard = files.lock().unwrap();
                if let Some(file) = files_guard.get(idx) {
                    if confirm_delete_ready {
                        format!(
                            "{} {}:",
                            lang.labels.bottom_bar.delete_text.press_enter_to_confirm, file.path
                        )
                    } else {
                        format!(
                            "{} {}:",
                            lang.labels.bottom_bar.delete_text.press_yn_to_delete, file.path
                        )
                    }
                } else {
                    "".to_string()
                }
            } else {
                lang.labels.bottom_bar.fast_help.clone()
            };

            let bottom_bar_widget = Paragraph::new(bottom_message)
                .style(Style::default().bg(Color::Rgb(27, 59, 111)))
                .alignment(Alignment::Left)
                .block(Block::default());
            f.render_widget(bottom_bar_widget, bottom_bar_area);

            let item_height: u16 = 1;
            max_visible = (chunks[1].height.saturating_sub(2)) as usize / item_height as usize;

            let visible_files: Vec<(usize, read_dir::FileInfo)> = {
                let files_guard = files.lock().unwrap();
                files_guard
                    .iter()
                    .enumerate()
                    .skip(scroll)
                    .take(max_visible)
                    .map(|(i, f)| (i, f.clone()))
                    .collect()
            };

            let y_left = chunks[1].y + 1;

            let area_left = Rect {
                x: chunks[1].x + 1,
                y: y_left,
                width: chunks[1].width.saturating_sub(1),
                height: 8,
            };

            for (i, (idx, file)) in visible_files.iter().enumerate() {
                let y_right = chunks[0].y + (i as u16) + 1;
                let is_selected = *idx == selected;

                let file_style = if is_selected {
                    Style::default()
                        .fg(Color::Green)
                        .bg(Color::Rgb(88, 164, 176))
                } else {
                    Style::default().fg(Color::Green)
                };

                let info_style = Style::default();

                let area_right = Rect {
                    x: chunks[0].x + 1,
                    y: y_right,
                    width: chunks[0].width.saturating_sub(1),
                    height: item_height,
                };

                let max_width = (area_left.width as usize) - 1;
                let file_name_trunc = truncate_text(&file.path, max_width);

                let right_side =
                    Paragraph::new(vec![Line::from(Span::styled(file_name_trunc, file_style))])
                        .block(Block::default())
                        .wrap(Wrap { trim: true });
                f.render_widget(right_side, area_right);

                let files_guard = files.lock().unwrap();
                if let Some(file) = files_guard.get(selected) {
                    let path = current_path.join(&file.path);
                    let cache_guard = size_cache.lock().unwrap();
                    let text = if let Some(size) = cache_guard.get(&path) {
                        file_size::convert_bytes(*size as u64)
                    } else if std::fs::metadata(&path)
                        .map(|m| m.is_dir())
                        .unwrap_or(false)
                    {
                        "...".to_string()
                    } else {
                        file.human_size.clone()
                    };

                    if selected != 0 {
                        let is_dir = std::fs::metadata(&path)
                            .map(|m| m.is_dir())
                            .unwrap_or(false);

                        let lines = if is_dir {
                            vec![
                                Line::from(Span::styled(
                                    format!("{}{}", lang.data.size_human_format, text.clone()),
                                    info_style,
                                )),
                                Line::from(Span::styled(
                                    format!("{}{}", lang.data.size_byte, file.byte_size),
                                    info_style,
                                )),
                                Line::from(Span::styled(
                                    format!("{}{}", lang.data.owner_uid, file.owner_uid),
                                    info_style,
                                )),
                                Line::from(Span::styled(
                                    format!("{}{}", lang.data.owner_gid, file.owner_gid),
                                    info_style,
                                )),
                                Line::from(Span::styled(
                                    format!("{}{}", lang.data.inodes, file.inode),
                                    info_style,
                                )),
                                Line::from(Span::styled(
                                    format!("{}{}", lang.data.permissions, file.permissions),
                                    info_style,
                                )),
                                Line::from(Span::styled(
                                    format!("{}{}", lang.data.nlinks, file.nlinks),
                                    info_style,
                                )),
                            ]
                        } else {
                            vec![
                                Line::from(Span::styled(
                                    format!("{}{}", lang.data.size_human_format, text.clone()),
                                    info_style,
                                )),
                                Line::from(Span::styled(
                                    format!("{}{}", lang.data.size_byte, file.byte_size),
                                    info_style,
                                )),
                                Line::from(Span::styled(
                                    format!("{}{}", lang.data.owner_uid, file.owner_uid),
                                    info_style,
                                )),
                                Line::from(Span::styled(
                                    format!("{}{}", lang.data.owner_gid, file.owner_gid),
                                    info_style,
                                )),
                                Line::from(Span::styled(
                                    format!("{}{}", lang.data.inodes, file.inode),
                                    info_style,
                                )),
                                Line::from(Span::styled(
                                    format!("{}{}", lang.data.permissions, file.permissions),
                                    info_style,
                                )),
                                Line::from(Span::styled(
                                    format!("{}{}", lang.data.nlinks, file.nlinks),
                                    info_style,
                                )),
                                Line::from(Span::raw("")),
                            ]
                        };

                        let info = Paragraph::new(lines)
                            .block(Block::default())
                            .wrap(Wrap { trim: true })
                            .style(Style::default());

                        f.render_widget(info, area_left);
                    } else {
                        // falta traduzir essa parte.
                        let anterior = lang.status.previous_folder.clone();
                        f.render_widget(anterior, area_left);
                    }
                }
            }
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if let Some(idx) = confirm_delete {
                    match key.code {
                        KeyCode::Char('y') => {
                            confirm_delete_ready = true;
                        }
                        KeyCode::Enter => {
                            if confirm_delete_ready {
                                let mut guard = files.lock().unwrap();
                                if let Some(file) = guard.get(idx) {
                                    let path = current_path.join(&file.path);

                                    if file.path == ".." {
                                        confirm_delete = None;
                                        confirm_delete_ready = false;
                                        continue;
                                    }

                                    let result = if path.is_file() {
                                        std::fs::remove_file(&path)
                                    } else if path.is_dir() {
                                        std::fs::remove_dir_all(&path)
                                    } else {
                                        Err(std::io::Error::other("Invalid type."))
                                    };

                                    if result.is_ok() {
                                        guard.remove(idx);
                                        selected = selected.saturating_sub(1);
                                    }
                                }
                                confirm_delete = None;
                                confirm_delete_ready = false;
                            }
                        }
                        KeyCode::Char('n') => {
                            confirm_delete = None;
                            confirm_delete_ready = false;
                        }
                        _ => {}
                    }
                    continue;
                }
                match key.code {
                    KeyCode::Char('d') => {
                        if selected == 0 {
                            confirm_delete = None;
                        } else {
                            confirm_delete = Some(selected);
                        }
                    }

                    KeyCode::Char('q') => break,
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                            if selected < scroll {
                                scroll = scroll.saturating_sub(1);
                            }
                        }
                    }
                    KeyCode::Down => {
                        if selected + 1 < files.lock().unwrap().len() {
                            selected += 1;
                            if selected >= scroll + max_visible {
                                scroll += 1;
                            }
                        }
                    }
                    KeyCode::Enter => {
                        let files_guard = files.lock().unwrap();
                        if let Some(file) = files_guard.get(selected) {
                            let new_path = current_path.join(&file.path);
                            if std::fs::metadata(&new_path)
                                .map(|m| m.is_dir())
                                .unwrap_or(false)
                            {
                                current_path = new_path;
                                selected = 0;
                                scroll = 0;
                                drop(files_guard);
                                files = Arc::new(Mutex::new(read_dir::read_dir(
                                    current_path.to_str().unwrap_or(""),
                                )));
                                let new_files =
                                    read_dir::read_dir(current_path.to_str().unwrap_or(""));
                                let mut guard = files.lock().unwrap();
                                *guard = new_files;

                                guard.insert(
                                    0,
                                    FileInfo {
                                        path: "..".to_string(),
                                        byte_size: 0,
                                        human_size: "".to_string(),
                                        owner_uid: 0,
                                        owner_gid: 0,
                                        inode: 0,
                                        permissions: 0,
                                        nlinks: 0,
                                        dir_info: None,
                                    },
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn truncate_text(text: &str, max_width: usize) -> String {
    if text.len() > max_width && max_width > 3 {
        format!("{}...", &text[..max_width - 2])
    } else {
        text.to_string()
    }
}
