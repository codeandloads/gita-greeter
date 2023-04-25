use crate::db::connection;

#[allow(dead_code)]
pub fn destroy() {
    let connection = match connection::connect() {
        Ok(conn) => conn,
        Err(err) => {
            println!("{:?}", err.message);
            panic!("Failed to connect to the database");
        }
    };
    let query = "DROP TABLE IF EXISTS verses;";
    connection.execute(query).unwrap();
}
