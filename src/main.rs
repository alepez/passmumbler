use clap::Parser;
use std::collections::BTreeMap;
use std::io::{BufRead, Read};

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

    let secrets = if cli.stdin {
        Secrets::from(std::io::stdin().lock())
    } else {
        Secrets::from(cli)
    };

    let username = secrets
        .other
        .get("username")
        .unwrap_or(&"".to_string())
        .to_string();

    let password = secrets.password;

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
    #[arg(short = 'u', long)]
    username: Option<String>,
    /// Password
    #[arg(short = 'p', long)]
    password: Option<String>,
    /// Read from stdin
    #[arg(short = 'i', long)]
    stdin: bool,
}

type SecretId = String;

type SecretData = String;

struct Secrets {
    password: SecretData,
    other: BTreeMap<SecretId, SecretData>,
}

impl<T> From<T> for Secrets
where
    T: Read + BufRead,
{
    fn from(reader: T) -> Self {
        let mut lines = reader.lines();
        let password = lines.next().unwrap().unwrap();
        let other = lines
            .map(|line| {
                let line = line.unwrap();
                let mut parts = line.splitn(2, ':');
                let id = parts.next().unwrap();
                let data = parts.next().unwrap().trim_start();
                (id.to_string(), data.to_string())
            })
            .collect();
        Self { password, other }
    }
}

impl From<Cli> for Secrets {
    fn from(value: Cli) -> Self {
        let mut other = BTreeMap::new();

        if let Some(username) = value.username {
            other.insert("username".to_string(), username);
        }

        Self {
            password: value.password.unwrap_or_default(),
            other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_store_single_line() {
        let input = b"password".as_slice();
        let secrets = Secrets::from(input);
        assert_eq!(secrets.password, "password");
    }

    #[test]
    fn test_password_store_multi_line() {
        let input = b"password\nusername: test".as_slice();
        let secrets = Secrets::from(input);
        assert_eq!(secrets.password, "password");
        assert_eq!(secrets.other.len(), 1);
        assert_eq!(secrets.other.get("username").unwrap(), "test");
    }
}
