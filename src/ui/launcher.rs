use gtk4::{
    Box, Button, Label, ListView, SignalListItemFactory, SingleSelection,
    gio::prelude::ListModelExt,
    glib::{BoxedAnyObject, object::Cast},
    prelude::{BoxExt, ButtonExt, ListItemExt},
};
use std::rc::Rc;

use crate::{
    constants::{ADD_GAME_FORM_STACK, LAUNCHER_STACK},
    db::game::Game,
    ui::AppController,
};

pub struct UiLauncher {
    title: Rc<Label>,
    app_controller: Rc<AppController>,
}

impl UiLauncher {
    pub fn new(app_controller: Rc<AppController>) -> UiLauncher {
        UiLauncher {
            title: Rc::new(
                Label::builder()
                    .label("Game Title")
                    .css_classes(["game-title"])
                    .ellipsize(gtk4::pango::EllipsizeMode::End)
                    .max_width_chars(20)
                    .vexpand(true)
                    .build(),
            ),
            app_controller,
        }
    }

    pub fn get_box(&self) -> Box {
        let main_box = Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .build();
        let left_box = self.left_box();
        let right_box = self.right_box();

        main_box.append(&left_box);
        main_box.append(&right_box);

        self.app_controller
            .stack
            .add_named(&main_box, Some(LAUNCHER_STACK));
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

        let store = self.app_controller.build_list_view_store();
        if store.n_items() > 0 {
            if let Some(obj) = store.item(0) {
                let obj = obj.downcast::<BoxedAnyObject>().unwrap();
                let game = obj.borrow::<Game>();
                self.title.set_text(game.name());
            }
        }

        let selection = SingleSelection::new(Some(store.clone()));
        let title = self.title.clone();
        selection.connect_selected_notify(move |sel| {
            if let Some(obj) = sel.selected_item() {
                let obj = obj.downcast::<BoxedAnyObject>().unwrap();
                let game = obj.borrow::<Game>();

                title.set_text(game.name());
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

        let ac_cloned = self.app_controller.clone();
        settings_btn.connect_clicked(move |_| {
            ac_cloned.stack.set_visible_child_name(ADD_GAME_FORM_STACK);
        });

        button_box.append(&launch_btn);
        button_box.append(&settings_btn);

        right_box.append(self.title.as_ref());
        right_box.append(&button_box);

        right_box
    }
}
