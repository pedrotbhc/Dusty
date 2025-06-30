use serde::Deserialize;
use serde_json;

// Portugues brasil
pub const PT_BR_JSON: &str = include_str!("../assets/pt_br.json");
// Ingles estadunidense
pub const EN_US_JSON: &str = include_str!("../assets/en_us.json");
// Frances da franca
pub const FR_FR_JSON: &str = include_str!("../assets/fr_fr.json");
// Espanhol da espanha
pub const ES_ES_JSON: &str = include_str!("../assets/es_es.json");

pub fn load_langs(lang: &str) -> AppText {
    let content = match lang.trim() {
        "portuguese" => PT_BR_JSON,
        "english" => EN_US_JSON,
        "french" => FR_FR_JSON,
        "spanish" => ES_ES_JSON,
        _ => EN_US_JSON,
    };

    serde_json::from_str(content).expect("Error")
}

#[derive(Deserialize, Clone)]
pub struct AppText {
    pub labels: LabelsText,
    pub status: StatusText,
    pub data: DataText,
    pub logs: LogsText,
    pub input: InputText,
    pub messages: MessagesText,
    pub help: HelpText,
    pub about: AboutText,
}

#[derive(Deserialize, Clone)]
pub struct LabelsText {
    pub left_area: LeftArea,
    pub right_area: RightArea,
    pub top_bar: TopBar,
    pub bottom_bar: BottomBar,
}

#[derive(Deserialize, Clone)]
pub struct LeftArea {
    pub title: String,
}

#[derive(Deserialize, Clone)]
pub struct RightArea {
    pub title: String,
}

#[derive(Deserialize, Clone)]
pub struct BottomBar {
    pub fast_help: String,
    pub delete_text: DeleteText,
}

#[derive(Deserialize, Clone)]
pub struct DeleteText {
    pub press_yn_to_delete: String,
    pub press_enter_to_confirm: String,
}

#[derive(Deserialize, Clone)]
pub struct TopBar;

#[derive(Deserialize, Clone)]
pub struct StatusText {
    pub previous_folder: String,
    pub unknown_info: String,
    pub coming_soon: String,
}

#[derive(Deserialize, Clone)]
pub struct DataText {
    pub size_human_format: String,
    pub size_byte: String,
    pub owner_uid: String,
    pub owner_gid: String,
    pub inodes: String,
    pub permissions: String,
    pub nlinks: String,
}

#[derive(Deserialize, Clone)]
pub struct LogsText;

#[derive(Deserialize, Clone)]
pub struct InputText;

#[derive(Deserialize, Clone)]
pub struct MessagesText;

#[derive(Deserialize, Clone)]
pub struct HelpText;

#[derive(Deserialize, Clone)]
pub struct AboutText;
