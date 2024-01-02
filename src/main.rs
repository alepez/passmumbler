use clap::Parser;

use gtk4 as gtk;

use gtk::{
    gdk,
    glib::{self, clone},
    prelude::*,
};

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("net.pezzato.passmumbler")
        .build();
    application.connect_activate(build_ui);

    let args: &[&str] = &[];
    application.run_with_args(args)
}

fn build_ui(application: &gtk::Application) {
    let cli = Cli::parse();
    let username = cli.username;
    let password = cli.password;

    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("passmumbler")
        .default_width(660)
        .default_height(420)
        .build();

    let display = gdk::Display::default().unwrap();
    let clipboard = display.clipboard();

    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(24)
        .build();

    let text_container = gtk::Box::builder()
        .halign(gtk::Align::Center)
        .orientation(gtk::Orientation::Horizontal)
        .spacing(24)
        .build();

    let username_btn = gtk::Button::with_label("Username");
    username_btn.connect_clicked(clone!(@weak clipboard => move |_btn| {
        clipboard.set_text(username.as_str());
    }));
    text_container.append(&username_btn);

    let password_btn = gtk::Button::with_label("Password");
    password_btn.connect_clicked(clone!(@weak clipboard => move |_btn| {
        clipboard.set_text(password.as_str());
    }));
    text_container.append(&password_btn);

    container.append(&text_container);

    let texture_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .halign(gtk::Align::Center)
        .spacing(24)
        .build();

    container.append(&texture_container);

    window.set_child(Some(&container));
    window.present();
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Username
    #[arg(short = 'u', long, default_value = "")]
    username: String,
    /// Password
    #[arg(short = 'p', long, default_value = "")]
    password: String,
}
