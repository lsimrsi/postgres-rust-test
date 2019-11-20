extern crate postgres;

use std::env;
use postgres::{Connection, TlsMode};

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

fn main() {
    let database_url = env::var("TSDB_URL").expect("TSDB_URL must be set");
    let conn = Connection::connect(database_url.to_owned(), TlsMode::None)
            .unwrap();

    conn.execute("CREATE TABLE person (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    data            BYTEA
                  )", &[]).unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_owned(),
        data: None
    };
    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
                 &[&me.name, &me.data]).unwrap();

    for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2)
        };
        println!("Found person {}", person.name);
    }
}