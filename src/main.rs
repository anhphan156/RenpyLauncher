use std::rc::Rc;

use gtk4::{
    Application, ApplicationWindow, CssProvider, HeaderBar,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    glib,
    prelude::GtkWindowExt,
};

use crate::{
    constants::*,
    ui::{AppController, create_game::UiAddGameForm, launcher::UiLauncher},
};

mod constants;
mod db;
mod ui;

fn main() -> glib::ExitCode {
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

        let app_controller = Rc::new(AppController::new());
        let _ = UiLauncher::new(app_controller.clone()).get_box();
        let _ = UiAddGameForm::new(app_controller.clone()).get_box();

        app_controller.stack.set_visible_child_name(LAUNCHER_STACK);

        window.set_child(Some(&app_controller.stack));
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
