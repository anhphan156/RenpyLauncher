use gtk4::{
    Box, Button, Label, ListView, SignalListItemFactory, SingleSelection,
    gio::prelude::ListModelExt,
    glib::{BoxedAnyObject, object::Cast},
    prelude::{BoxExt, ButtonExt, ListItemExt},
};
use std::{cell::RefCell, process::Command, rc::Rc};

use crate::{
    constants::{ADD_GAME_FORM_STACK, LAUNCHER_STACK, WAITING_STACK},
    db::game::Game,
    ui::AppController,
};

pub struct UiLauncher {
    current_title: Rc<Label>,
    current_exe: Rc<RefCell<String>>,
    app_controller: Rc<AppController>,
}

impl UiLauncher {
    pub fn new(app_controller: Rc<AppController>) -> UiLauncher {
        UiLauncher {
            current_title: Rc::new(
                Label::builder()
                    .label("Game Title")
                    .css_classes(["game-title"])
                    .ellipsize(gtk4::pango::EllipsizeMode::End)
                    .max_width_chars(20)
                    .vexpand(true)
                    .build(),
            ),
            app_controller,
            current_exe: Rc::new(RefCell::new(String::from(""))),
        }
    }

    pub fn get_box(&self) -> Box {
        let main_box = Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .build();
        let left_box = self.left_box();
        let right_box = self.right_box();
        let waiting_box = self.configure_waiting_box();

        main_box.append(&left_box);
        main_box.append(&right_box);

        let ac_cloned = self.app_controller.clone();
        let rx = ac_cloned.rx.clone();
        gtk4::glib::MainContext::default().spawn_local(async move {
            while let Ok(msg) = rx.recv().await {
                println!("msg received: {}", msg);
                ac_cloned.stack.set_visible_child_name(LAUNCHER_STACK);
            }
        });

        self.app_controller
            .stack
            .add_named(&main_box, Some(LAUNCHER_STACK));
        self.app_controller
            .stack
            .add_named(&waiting_box, Some(WAITING_STACK));
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
                self.current_title.set_text(game.name());
                self.current_exe.borrow_mut().push_str(game.path());
            }
        }

        let selection = SingleSelection::new(Some(store.clone()));
        let title = self.current_title.clone();
        let exe = self.current_exe.clone();
        selection.connect_selected_notify(move |sel| {
            if let Some(obj) = sel.selected_item() {
                let obj = obj.downcast::<BoxedAnyObject>().unwrap();
                let game = obj.borrow::<Game>();

                title.set_text(game.name());
                let mut exe = exe.borrow_mut();
                exe.clear();
                exe.push_str(game.path());
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

        let exe = self.current_exe.clone();
        let ac_cloned = self.app_controller.clone();
        let tx = ac_cloned.tx.clone();
        launch_btn.connect_clicked(move |_| {
            let path = exe.borrow().clone();
            ac_cloned.stack.set_visible_child_name(WAITING_STACK);
            let tx = tx.clone();
            std::thread::spawn(move || {
                let mut child = Command::new("sh")
                    .arg(path)
                    .spawn()
                    .expect("Failed to spawn game");

                let status = child.wait().unwrap();
                let _ = tx.send_blocking(status.to_string());
            });
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

        right_box.append(self.current_title.as_ref());
        right_box.append(&button_box);

        right_box
    }

    fn configure_waiting_box(&self) -> Box {
        let lbl = Label::builder()
            .label("Game is running!")
            .css_classes(["primary-font-size"])
            .build();
        let main_box = Box::builder()
            .hexpand(true)
            .vexpand(true)
            .halign(gtk4::Align::Center)
            .valign(gtk4::Align::Center)
            .build();
        main_box.append(&lbl);
        main_box
    }
}
