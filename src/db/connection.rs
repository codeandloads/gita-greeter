use sqlite::Connection;
use sqlite::Error;
use std::env;

pub fn connect() -> Result<Connection, Error> {
    let sqlite_path_dir: String;
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let db_dir = exe_path.parent().expect("Failed to get parent directory");
    let file_path_dir = db_dir.join("bhagavad.sqlite");
    sqlite_path_dir = file_path_dir.to_string_lossy().to_string();

    // Embed the example.sqlite file as a string
    const EXAMPLE_SQLITE: &str = include_str!("../../bhagavad.sqlite");
    match sqlite::open(sqlite_path_dir) {
        Ok(conn) => return Ok(conn),
        Err(err) => Err(err),
    }
}
