use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, CssProvider, Label, Orientation, StyleContext, Window, WindowType, Box};
use std::fs;
use std::process::Command;
use gtk::glib::Bytes;
use gtk::gdk::Display;

// Path to the icon asset
const ICON_PATH: &str = "assets/icon.png"; // Ensure the icon exists in the assets folder
const CSS_PATH: &str = "assets/style.css"; // Linux theme CSS

pub fn create_ui(app: &Application) {
    let provider = CssProvider::new();
    if let Ok(css_data) = fs::read(CSS_PATH) {
        provider.load_from_data(&Bytes::from(&css_data)).expect("Failed to load CSS");
        StyleContext::add_provider_for_display(&Display::default().unwrap(), &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }

    let window = ApplicationWindow::new(app);
    window.set_title("SimpyManager");
    window.set_default_size(500, 300);
    window.set_icon_from_file(ICON_PATH).expect("Failed to set app icon");

    let vbox = Box::new(Orientation::Vertical, 15);
    vbox.set_margin_top(20);
    vbox.set_margin_bottom(20);
    vbox.set_margin_start(30);
    vbox.set_margin_end(30);

    let title_label = Label::new(Some(":desktop: SimpyManager"));
    title_label.set_justify(gtk::Justification::Center);
    title_label.get_style_context().add_class("title-label");
    vbox.append(&title_label);

    let description = Label::new(Some("A stylish Linux-themed system utility manager."));
    description.set_justify(gtk::Justification::Center);
    description.get_style_context().add_class("desc-label");
    vbox.append(&description);

    let beta_button = Button::with_label(":globe_with_meridians: Join Beta Program");
    beta_button.get_style_context().add_class("custom-button");
    beta_button.connect_clicked(|_| {
        crate::join_beta_program();
    });
    vbox.append(&beta_button);

    let update_button = Button::with_label(":arrows_counterclockwise: Update System & Flatpak");
    update_button.get_style_context().add_class("custom-button");
    update_button.connect_clicked(|_| {
        crate::run_updates();
    });
    vbox.append(&update_button);

    let footer_label = Label::new(Some(":penguin: Linux Edition | Version 1.0.0"));
    footer_label.set_justify(gtk::Justification::Center);
    footer_label.get_style_context().add_class("footer-label");
    vbox.append(&footer_label);

    window.set_child(Some(&vbox));
    window.show();
}

pub fn ask_to_join_beta() {
    const BETA_ANSWER_FILE: &str = "/usr/share/simpy_manager/beta_answer.txt";

    if fs::metadata(BETA_ANSWER_FILE).is_err() {
        let user_response = Command::new("zenity")
            .arg("--question")
            .arg("--text=Do you want to join the Beta program?")
            .output()
            .expect("Failed to display question");

        if user_response.status.success() {
            crate::join_beta_program();
            fs::write(BETA_ANSWER_FILE, "yes").expect("Failed to write answer");
        } else {
            fs::write(BETA_ANSWER_FILE, "no").expect("Failed to write answer");
        }
    }
}