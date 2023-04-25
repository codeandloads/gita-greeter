use sqlite::Connection;
use sqlite::Error;

pub fn connect() -> Result<Connection, Error> {
    match sqlite::open("bhagavad.sqlite") {
        Ok(conn) => return Ok(conn),
        Err(err) => Err(err),
    }
}
