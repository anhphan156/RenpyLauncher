use gtk4::{
    Box, Button, Label, ListView, SignalListItemFactory, SingleSelection, Stack,
    glib::{BoxedAnyObject, object::Cast},
    prelude::{BoxExt, ButtonExt, ListItemExt, WidgetExt},
};
use rusqlite::{Connection, Error};
use std::rc::Rc;

use crate::{constants::ADD_GAME_FORM_STACK, db::game::Game};

pub struct UiLauncher {
    stack: Rc<Stack>,
    conn: Rc<Connection>,
}

impl UiLauncher {
    pub fn new(stack: Rc<Stack>, conn: Rc<Connection>) -> UiLauncher {
        UiLauncher { stack, conn }
    }

    pub fn get_box(&self) -> Box {
        let main_box = Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .build();
        let left_box = self.left_box();
        let right_box = self.right_box();

        main_box.append(&left_box);
        main_box.append(&right_box);

        main_box
    }

    fn left_box(&self) -> Box {
        let left_box = Box::builder()
            .css_classes(["left-box"])
            .hexpand(true)
            .build();

        let factory = SignalListItemFactory::new();
        factory.connect_setup(|_, item| {
            let list_item = item.downcast_ref::<gtk4::ListItem>().unwrap();
            let label = Label::builder().halign(gtk4::Align::Start).build();
            list_item.set_child(Some(&label));
        });
        factory.connect_bind(|_, item| {
            let list_item = item.downcast_ref::<gtk4::ListItem>().unwrap();

            let obj = list_item.item().unwrap();
            let obj = obj.downcast::<BoxedAnyObject>().unwrap();

            let game = obj.borrow::<Game>();

            let label = list_item.child().unwrap().downcast::<Label>().unwrap();

            label.set_label(game.name());
        });

        let store = self.build_list_view_model();
        let selection = SingleSelection::new(Some(store));

        selection.connect_selected_notify(|sel| {
            if let Some(obj) = sel.selected_item() {
                let obj = obj.downcast::<BoxedAnyObject>().unwrap();
                let game = obj.borrow::<Game>();

                println!("selected {}", game.name());
            }
        });

        let list_view = ListView::builder()
            .factory(&factory)
            .model(&selection)
            .hexpand(true)
            .build();

        left_box.append(&list_view);

        left_box
    }

    fn right_box(&self) -> Box {
        let right_box = Box::builder()
            .hexpand(true)
            .orientation(gtk4::Orientation::Vertical)
            .build();
        let button_box = Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .vexpand(false)
            .hexpand(false)
            .valign(gtk4::Align::End)
            .halign(gtk4::Align::End)
            .build();

        let game_title = Label::builder()
            .label("Game TitleGame TitleGame TitleGame Title")
            .css_classes(["game-title"])
            .ellipsize(gtk4::pango::EllipsizeMode::End)
            .max_width_chars(20)
            .vexpand(true)
            .build();

        let launch_btn = Button::builder()
            .label("Launch")
            .css_classes(["launch-button", "bg-blue", "secondary-font-size"])
            .build();

        launch_btn.connect_clicked(|_| {
            eprintln!("Oh yeah baby click harder!");
        });

        let settings_btn = Button::builder()
            .label("O")
            .css_classes(["settings-button", "bg-blue", "secondary-font-size"])
            .build();

        let stack = self.stack.clone();
        settings_btn.connect_clicked(move |_| {
            stack.set_visible_child_name(ADD_GAME_FORM_STACK);
        });

        button_box.append(&launch_btn);
        button_box.append(&settings_btn);

        right_box.append(&game_title);
        right_box.append(&button_box);

        right_box
    }

    fn build_list_view_model(&self) -> gtk4::gio::ListStore {
        let store = gtk4::gio::ListStore::new::<BoxedAnyObject>();

        if let Ok(games) = self.load_games() {
            for g in games {
                store.append(&BoxedAnyObject::new(g));
            }
        }

        store
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
