use sqlite::Connection;
use sqlite::Error;
use std::env;
use std::path::PathBuf;

pub fn connect() -> Result<Connection, Error> {
    let home_dir = env::var_os("HOME").expect("Failed to get home directory");
    let mut db_path = PathBuf::from(home_dir);
    db_path.push("bhagavad.sqlite");
    match sqlite::open(db_path) {
        Ok(conn) => return Ok(conn),
        Err(err) => Err(err),
    }
}
