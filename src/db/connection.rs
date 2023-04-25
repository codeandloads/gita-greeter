use sqlite::Connection;
use sqlite::Error;
use std::env;
use std::path::PathBuf;

pub fn connect() -> Result<Connection, Error> {
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let db_dir = exe_path.parent().expect("Failed to get parent directory");
    let file_path_dir = db_dir.join("bhagavad.sqlite");
    let db_path_str = file_path_dir.to_string_lossy().to_string();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to read CARGO_MANIFEST_DIR");
    let db_path = PathBuf::from(manifest_dir).join("bhagavad.sqlite");
    let db_path_str = db_path.to_string_lossy().to_string();
    println!("{:?}", db_path_str);
    /* @TOOD if CARGO_MANIFEST_DIR exists then switch path for local development, else symbolic
       link
       Check if CARGO_MANIFEST_DIR environment variable exists
       if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
           // Construct the full path to the db.sqlite file using CARGO_MANIFEST_DIR
           let db_path = PathBuf::from(manifest_dir).join("db.sqlite");
           let db_path_str = db_path.to_string_lossy().to_string();

           println!("DB path: {}", db_path_str);

           // Use the db_path_str to open and access the db.sqlite file using rusqlite crate
           let conn = Connection::open(&db_path_str).expect("Failed to open db.sqlite");

           // ... your code that uses the conn connection ...
       } else {
           // Fallback path if CARGO_MANIFEST_DIR does not exist
           let db_path_str = "/path/to/your/fallback/db.sqlite";

           println!("DB path: {}", db_path_str);

           // Use the db_path_str to open and access the db.sqlite file using rusqlite crate
           let conn = Connection::open(&db_path_str).expect("Failed to open db.sqlite");

           // ... your code that uses the conn connection ...
       }
       */
    match sqlite::open(db_path_str) {
        Ok(conn) => return Ok(conn),
        Err(err) => Err(err),
    }
}
