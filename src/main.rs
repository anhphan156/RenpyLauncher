use gtk4::{
    Application, ApplicationWindow, CssProvider, HeaderBar,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    glib,
    prelude::GtkWindowExt,
};

use crate::{
    db::sqlite_init,
    ui::{create_game::ui_add_game_form, launcher::ui_launcher},
};

mod db;
mod ui;

fn main() -> glib::ExitCode {
    let conn = match sqlite_init() {
        Ok(c) => c,
        Err(e) => {
            println!("Error connecting to game database: {}", e.to_string());
            return glib::ExitCode::FAILURE;
        }
    };
    let conn = std::rc::Rc::new(conn);

    let app = Application::builder()
        .application_id("org.example.VnLauncher")
        .build();

    app.connect_activate(move |app| {
        load_css();

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(1280)
            .default_height(720)
            .title("VN Launcher")
            .css_classes(["launcher-background"])
            .build();

        let header_bar = HeaderBar::builder().show_title_buttons(true).build();

        let stack = gtk4::Stack::new();
        let stack = std::rc::Rc::new(stack);

        let create_game_form = ui_add_game_form(stack.clone(), conn.clone());
        let launcher = ui_launcher(stack.clone());

        stack.add_named(&launcher, Some("launcher"));
        stack.add_named(&create_game_form, Some("add_game_form"));
        stack.set_visible_child_name("launcher");

        window.set_child(Some(stack.as_ref()));
        window.set_titlebar(Some(&header_bar));

        window.present();
    });

    app.run()
}

fn load_css() {
    let _ = gtk4::gio::resources_register_include!("compiled.gresource");
    let provider = CssProvider::new();
    provider.load_from_resource("/vn_launcher/style.css");
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().unwrap(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
