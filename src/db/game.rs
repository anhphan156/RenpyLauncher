#[derive(Debug)]
pub struct Game {
    name: String,
    path: String,
}

impl Game {
    pub fn new(name: String, path: String) -> Game {
        Game {
            name: name,
            path: path,
        }
    }

    pub fn save(&self, conn: &rusqlite::Connection) -> Result<usize, rusqlite::Error> {
        conn.execute(
            "INSERT INTO games (name, path) VALUES (?1, ?2)",
            (&self.name, &self.path),
        )
    }
}
