use clap::Parser;
use std::io::BufRead;

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

fn read_from_stdin() -> (Option<String>, Option<String>) {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let password = lines.next().and_then(|x| x.ok());
    let username = lines.find_map(|line| {
        let line = line.ok()?;
        if line.starts_with("username: ") {
            Some(line.trim_start_matches("username: ").to_string())
        } else {
            None
        }
    });
    (username, password)
}

fn build_ui(application: &gtk::Application) {
    let cli = Cli::parse();

    let (username, password) = if cli.stdin {
        read_from_stdin()
    } else {
        (Some(cli.username), Some(cli.password))
    };

    let username = username.unwrap_or_default();
    let password = password.unwrap_or_default();

    if username.is_empty() || password.is_empty() {
        return;
    }

    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("passmumbler")
        .modal(true)
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
        .build();

    let text_container = gtk::Box::builder()
        .halign(gtk::Align::Center)
        .orientation(gtk::Orientation::Horizontal)
        .spacing(24)
        .build();

    let username_btn = gtk::Button::with_label(username.as_str());
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
    /// Read from stdin
    #[arg(short = 'i', long)]
    stdin: bool,
}
