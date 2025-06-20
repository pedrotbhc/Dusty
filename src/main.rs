use std::io::{stdout};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction, Rect},
    text::{Span, Line},
    style::{Style, Color},
};

mod dusty;
use crate::dusty::read_dir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Modo raw e tela alternativa
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Loop principal
    loop {
    let files = read_dir::read_dir(".");
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());

            let block = Block::default()
                .title("Dusty")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White));

            f.render_widget(block, chunks[0]);
            for file in files {
                let text = Paragraph::new(vec![
                    Line::from(vec![
                        Span::raw("Nome: "),
                        Span::styled(file.path, Style::default().fg(Color::Green)),
                    ]),
                    Line::from(vec![
                        Span::raw("Tamanho: "),
                        Span::styled(format!("{}", file.size), Style::default().fg(Color::Green)),
                    ]),
                ]);
                f.render_widget(text, f.size());
            }
        })?;

        // Sai com a tecla 'q'
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // Restaura o terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
