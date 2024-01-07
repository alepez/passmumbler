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

    match cli.command {
        Some(Commands::Show(cli)) => {
            let secrets = Secrets::try_from(cli).unwrap();
            build_show_ui(secrets, application);
        }
        Some(Commands::Select) => {
            eprintln!("Select not implemented yet");
        }
        None => {
            eprintln!("No command specified");
        }
    }
}

fn build_show_ui(secrets: Secrets, application: &gtk::Application) {
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
    /// Show a window with the secrets to copy to the clipboard
    Show(Show),
    /// Select a secret to copy to the clipboard
    Select,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl TryFrom<Show> for Secrets {
    type Error = &'static str;

    fn try_from(cli: Show) -> Result<Self, Self::Error> {
        if cli.stdin && (cli.username.is_some() || cli.password.is_some()) {
            return Err("password or username cannot be specified when reading from stdin");
        }

        if cli.stdin {
            return Ok(Secrets::from(std::io::stdin().lock()));
        }

        let mut inner = BTreeMap::new();

        if let Some(username) = cli.username {
            inner.insert("username".to_string(), username);
        }

        if let Some(password) = cli.password {
            inner.insert("password".to_string(), password);
        }

        Ok(Self::new(inner))
    }
}
