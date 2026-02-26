use super::super::constants::*;
use super::super::db::game::Game;
use std::rc::Rc;

use gtk4::{
    Align, Box, Button, Entry, Grid, Label, Stack,
    prelude::{BoxExt, ButtonExt, EditableExt, GridExt},
};

pub fn ui_add_game_form(stack: Rc<Stack>, conn: Rc<rusqlite::Connection>) -> Box {
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

    let stack_cloned = stack.clone();
    btn_back.connect_clicked(move |_| {
        stack_cloned.set_visible_child_name(LAUNCHER_STACK);
    });

    let stack_cloned = stack.clone();
    btn_add.connect_clicked(move |_| {
        let name = txt_name.text().to_string();
        let path = txt_path.text().to_string();
        let game = Game::new(name, path);
        match game.save(conn.as_ref()) {
            Ok(_) => stack_cloned.set_visible_child_name(LAUNCHER_STACK),
            Err(e) => println!("{}", e),
        }
    });

    main_box.append(&form_grid);

    main_box
}
