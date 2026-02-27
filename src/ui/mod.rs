use crate::db::{game::Game, sqlite_init};
use gtk4::{Stack, gio::ListStore, glib::BoxedAnyObject};
use rusqlite::{Connection, Error};

pub mod create_game;
pub mod launcher;

pub struct AppController {
    pub stack: Stack,
    pub conn: Connection,
    pub tx: async_channel::Sender<String>,
    pub rx: async_channel::Receiver<String>,
    store: ListStore,
}

impl AppController {
    pub fn new() -> AppController {
        let conn = match sqlite_init() {
            Ok(c) => c,
            Err(e) => {
                panic!("Error connecting to game database: {}", e.to_string());
            }
        };

        let (tx, rx) = async_channel::unbounded::<String>();

        AppController {
            stack: Stack::new(),
            conn,
            store: ListStore::new::<BoxedAnyObject>(),
            tx,
            rx,
        }
    }

    pub fn build_list_view_store(&self) -> &ListStore {
        self.store.remove_all();
        if let Ok(games) = self.load_games() {
            for g in games {
                self.store.append(&BoxedAnyObject::new(g));
            }
        }

        &self.store
    }

    fn load_games(&self) -> Result<Vec<Game>, Error> {
        let mut stmt = self.conn.prepare("SELECT * FROM games;")?;
        let game_iter = stmt.query_map([], |row| Ok(Game::new(row.get(1)?, row.get(2)?)))?;

        let mut games = vec![];
        for g in game_iter {
            games.push(g?);
        }

        Ok(games)
    }
}
