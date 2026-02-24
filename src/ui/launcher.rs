use std::rc::Rc;

use gtk4::{
    Box, Button, Label, Stack,
    prelude::{BoxExt, ButtonExt},
};

pub fn ui_launcher(stack: Rc<Stack>) -> Box {
    let main_box = Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .build();
    let left_box = Box::builder().hexpand(true).build();
    let right_box = Box::builder()
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

    settings_btn.connect_clicked(move |_| {
        stack.set_visible_child_name("add_game_form");
    });

    button_box.append(&launch_btn);
    button_box.append(&settings_btn);

    right_box.append(&game_title);
    right_box.append(&button_box);

    main_box.append(&left_box);
    main_box.append(&right_box);

    main_box
}
