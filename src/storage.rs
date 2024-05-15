use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::{self, Read};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct AppData {
    team: Option<TeamInfo>,
    user: Option<UserInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TeamInfo {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
}

static APP_INFO: Lazy<Mutex<AppData>> = Lazy::new(|| {
    Mutex::new(load_app_info().unwrap_or_default())
});

pub fn set_team_info(id: String, name: String) -> io::Result<()> {
    let mut app_data = APP_INFO.lock().unwrap();
    app_data.team = Some(TeamInfo { id, name });
    save_app_info(&app_data)
}

pub fn get_team_info() -> Option<TeamInfo> {
    APP_INFO.lock().unwrap().team.clone()
}

pub fn set_user_info(id: String, name: String) -> io::Result<()> {
    let mut app_data = APP_INFO.lock().unwrap();
    app_data.user = Some(UserInfo { id, name });
    save_app_info(&app_data)
}

pub fn get_user_info() -> Option<UserInfo> {
    APP_INFO.lock().unwrap().user.clone()
}

pub fn clear_app_info() -> io::Result<()> {
    let mut app_data = APP_INFO.lock().unwrap();
    app_data.team = None;
    app_data.user = None;
    save_app_info(&app_data)
}

fn save_app_info(app_data: &AppData) -> io::Result<()> {
    let file = File::create("app_info.json")?;
    serde_json::to_writer(file, &app_data)?;
    Ok(())
}

fn load_app_info() -> io::Result<AppData> {
    let file_path = "app_info.json";
    match File::open(file_path) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(serde_json::from_str(&contents).unwrap_or_else(|_| AppData::default()))
        },
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            Ok(AppData::default())
        },
        Err(e) => Err(e),
    }
}
