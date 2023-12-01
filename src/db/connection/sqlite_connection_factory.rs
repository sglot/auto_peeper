use rusqlite::Connection;

pub fn get() -> Connection {
    let mut path = std::env::var("DATABASE_PATH").expect("DATABASE_PATH must be set");
    path.push_str(&std::env::var("DATABASE_NAME").expect("DATABASE_PATH must be set"));
    
    Connection::open(path).unwrap()
}