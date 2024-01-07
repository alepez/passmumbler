use clap::{Parser, Subcommand};
use passmumbler::Secrets;
use std::collections::BTreeMap;

use gtk4 as gtk;

use gtk::{
    gdk,
    glib::{self, clone},
    prelude::*,
};

const APPLICATION_ID: &str = "net.pezzato.passmumbler";
const APPLICATION_NAME: &str = "passmumbler";

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id(APPLICATION_ID)
        .build();
    application.connect_activate(build_ui);

    let args: &[&str] = &[];
    application.run_with_args(args)
}

fn build_ui(application: &gtk::Application) {
    let cli = Cli::parse();

    let Commands::Show(cli) = cli.command.unwrap() else {
        eprintln!("No command specified");
        return;
    };

    build_show_ui(cli, application)
}

fn build_show_ui(cli: Show, application: &gtk::Application) {
    if cli.stdin && (cli.username.is_some() || cli.password.is_some()) {
        eprintln!("password or username cannot be specified when reading from stdin");
        return;
    }

    let secrets = if cli.stdin {
        Secrets::from(std::io::stdin().lock())
    } else {
        Secrets::from(cli)
    };

    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title(APPLICATION_NAME)
        .modal(true)
        .build();

    let display = gdk::Display::default().unwrap();
    let clipboard = display.clipboard();

    const SPACING: i32 = 12;

    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(SPACING)
        .margin_start(SPACING)
        .margin_end(SPACING)
        .margin_bottom(SPACING)
        .spacing(SPACING)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    for (label, data) in secrets.iter() {
        let btn = gtk::Button::with_label(label);
        btn.connect_clicked(clone!(@to-owned data, @weak clipboard => move |_btn| {
            clipboard.set_text(data.as_str());
        }));
        container.append(&btn);
    }

    window.set_child(Some(&container));
    window.present();
}

#[derive(Parser)]
struct Show {
    /// Username
    #[arg(short = 'u', long)]
    username: Option<String>,
    /// Password
    #[arg(short = 'p', long)]
    password: Option<String>,
    /// Read from stdin
    /// Check https://www.passwordstore.org/ for the format
    #[arg(short = 'i', long)]
    stdin: bool,
}

#[derive(Subcommand)]
enum Commands {
    Show(Show),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl From<Show> for Secrets {
    fn from(value: Show) -> Self {
        let mut inner = BTreeMap::new();

        if let Some(username) = value.username {
            inner.insert("username".to_string(), username);
        }

        if let Some(password) = value.password {
            inner.insert("password".to_string(), password);
        }

        Self::new(inner)
    }
}
