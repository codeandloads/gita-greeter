use sqlite::Connection;
use sqlite::Error;
use std::env;
use std::path::PathBuf;

pub fn connect() -> Result<Connection, Error> {
    let sqlite_path_dir: String;
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let db_path = PathBuf::from(manifest_dir).join("bhagavad.sqlite");
        sqlite_path_dir = db_path.to_string_lossy().to_string();
    } else {
        let exe_path = env::current_exe().expect("Failed to get current executable path");
        let db_dir = exe_path.parent().expect("Failed to get parent directory");
        let file_path_dir = db_dir.join("bhagavad.sqlite");
        sqlite_path_dir = file_path_dir.to_string_lossy().to_string();
    }
    match sqlite::open(&sqlite_path_dir) {
        Ok(conn) => return Ok(conn),
        Err(err) => Err(err),
    }
}
