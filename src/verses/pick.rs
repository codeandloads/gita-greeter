use crate::db::connection;
use sqlite::State;

pub fn pick() {
    let connection = match connection::connect() {
        Ok(conn) => conn,
        Err(_err) => {
            panic!("Failed to connect to the database");
        }
    };

    let query = "
SELECT * FROM verses WHERE id = (SELECT id FROM verses WHERE id >= (SELECT abs(random() % (SELECT count(id) FROM verses))) LIMIT 1);";

    let mut statement = match connection.prepare(query) {
        Ok(stmt) => stmt,
        Err(_err) => {
            println!("Sorry, i could not find any verses.");
            return;
        }
    };

    while let Ok(State::Row) = statement.next() {
        let chapter = statement.read::<i64, _>("chapter").unwrap();
        let verse = statement.read::<i64, _>("position").unwrap();
        let transliteration = statement.read::<String, _>("transliteration").unwrap();
        let translation = statement.read::<String, _>("translation").unwrap();
        println!("{:-^30}", chapter);
        println!();
        println!("{}", transliteration);
        println!();
        println!("{}", translation);
        println!();
        println!("{:-^30}", verse);
    }
}
