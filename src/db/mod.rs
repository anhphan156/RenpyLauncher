use rusqlite::Connection;

pub mod game;

pub fn sqlite_init() -> Result<Connection, rusqlite::Error> {
    let path = dirs::data_local_dir()
        .unwrap()
        .join("vn_launcher")
        .join("vn_launcher.db");
    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS games (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            path TEXT NOT NULL
        )",
        (),
    )?;

    return Ok(conn);
}
