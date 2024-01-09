use gtk4 as gtk;

use gtk::{
    gdk,
    glib::{self, clone},
    prelude::*,
    Align, Application, ApplicationWindow, Button, Label, Orientation,
};

use super::Props;

const WINDOW_NAME: &str = "passmumbler";

pub fn build_show_ui(props: Props, application: &Application) {
    let Props { title, secrets } = props;

    let window = ApplicationWindow::builder()
        .application(application)
        .title(WINDOW_NAME)
        .modal(true)
        .build();

    let display = gdk::Display::default().unwrap();
    let clipboard = display.clipboard();

    const SPACING: i32 = 12;

    let container = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(SPACING)
        .margin_start(SPACING)
        .margin_end(SPACING)
        .margin_bottom(SPACING)
        .spacing(SPACING)
        .halign(Align::Center)
        .valign(Align::Center)
        .build();

    // Add a title if specified
    if let Some(title) = title {
        let label = Label::builder()
            .label(title)
            .halign(Align::Center)
            .valign(Align::Center)
            .build();
        container.append(&label);
    }

    // Add all the secrets as buttons, which when clicked will copy the secret to the clipboard
    for (label, data) in secrets.iter() {
        let btn = Button::with_label(label);
        btn.set_tooltip_text(Some("Copy to clipboard"));
        btn.connect_clicked(clone!(@to-owned data, @weak clipboard => move |_btn| {
            clipboard.set_text(data.as_str());
        }));
        container.append(&btn);
    }

    // Add a button to clear the clipboard and close the window
    {
        let txt = if secrets.is_empty() {
            "Close"
        } else {
            "Clear & Close"
        };

        let btn = Button::with_label(txt);
        btn.connect_clicked(clone!(@weak window, @weak clipboard => move |_btn| {
            clipboard.set_text("");
            window.close();
        }));
        container.append(&btn);
    }

    // Show some text if there are no secrets
    if secrets.is_empty() {
        let label = Label::builder()
            .label("No secrets found")
            .halign(Align::Center)
            .valign(Align::Center)
            .build();
        container.append(&label);
    }

    window.set_child(Some(&container));
    window.present();
}
