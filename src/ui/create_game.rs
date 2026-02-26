use crate::db::game::Game;
use crate::{constants::*, ui::AppController};
use gtk4::{
    Align, Box, Button, Entry, Grid, Label,
    prelude::{BoxExt, ButtonExt, EditableExt, GridExt},
};
use std::rc::Rc;

pub struct UiAddGameForm {
    app_controller: Rc<AppController>,
}

impl UiAddGameForm {
    pub fn new(app_controller: Rc<AppController>) -> UiAddGameForm {
        UiAddGameForm { app_controller }
    }

    pub fn get_box(&self) -> Box {
        let main_box = Box::builder()
            .vexpand(true)
            .hexpand(true)
            .halign(Align::Center)
            .valign(Align::Center)
            .build();

        let form_grid = Grid::builder()
            .row_spacing(8)
            .column_spacing(8)
            .css_classes(["settings"])
            .build();

        let lb_form = Label::builder()
            .label("Add a New Entry")
            .halign(Align::Center)
            .css_classes(["primary-font-size"])
            .build();

        let lb_name = Label::builder().label("Name").halign(Align::End).build();
        let lb_path = Label::builder().label("Path").halign(Align::End).build();

        let txt_name = Entry::builder().halign(Align::Start).build();
        let txt_path = Entry::builder().halign(Align::Start).build();

        let btn_back = Button::builder().label("Back").build();
        let btn_add = Button::builder()
            .label("Add")
            .css_classes(["bg-blue"])
            .build();

        form_grid.attach(&lb_form, 0, 0, 3, 1);
        form_grid.attach(&lb_name, 0, 1, 1, 1);
        form_grid.attach(&lb_path, 0, 2, 1, 1);
        form_grid.attach(&txt_name, 1, 1, 3, 1);
        form_grid.attach(&txt_path, 1, 2, 3, 1);
        form_grid.attach(&btn_back, 1, 3, 1, 1);
        form_grid.attach(&btn_add, 2, 3, 1, 1);

        let ac_cloned = self.app_controller.clone();
        btn_back.connect_clicked(move |_| {
            ac_cloned.stack.set_visible_child_name(LAUNCHER_STACK);
        });

        let ac_cloned = self.app_controller.clone();
        btn_add.connect_clicked(move |_| {
            let name = txt_name.text().to_string();
            let path = txt_path.text().to_string();
            let game = Game::new(name, path);
            match game.save(&ac_cloned.conn) {
                Ok(_) => {
                    ac_cloned.stack.set_visible_child_name(LAUNCHER_STACK);
                    ac_cloned.build_list_view_store();
                }
                Err(e) => println!("{}", e),
            }
        });

        main_box.append(&form_grid);
        self.app_controller
            .stack
            .add_named(&main_box, Some(ADD_GAME_FORM_STACK));
        main_box
    }
}
