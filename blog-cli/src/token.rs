use std::fs;
use std::path::PathBuf;

const TOKEN_FILE: &str = ".blog_token";

pub fn load_token() -> Option<String> {
    let path = PathBuf::from(TOKEN_FILE);
    if path.exists() {
        fs::read_to_string(path).ok().map(|s| s.trim().to_string())
    } else {
        None
    }
}

pub fn save_token(token: &str) -> Result<(), String> {
    fs::write(TOKEN_FILE, token).map_err(|e| format!("Не удалось сохранить токен: {}", e))
}
