use clap::Parser;
use std::collections::BTreeMap;
use std::io::{BufRead, Read};

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

    for (label, data) in secrets.0.iter() {
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
#[command(author, version, about, long_about = None)]
struct Cli {
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

type SecretId = String;

type SecretData = String;

struct Secrets(BTreeMap<SecretId, SecretData>);

impl Secrets {
    fn get(&self, id: &str) -> Option<&SecretData> {
        self.0.get(id)
    }
}

impl<T> From<T> for Secrets
where
    T: Read + BufRead,
{
    fn from(reader: T) -> Self {
        let mut lines = reader.lines();

        // First line is the password
        let password = lines.next().map(|line| line.unwrap());

        // All other lines are key-value pairs, separated by a colon
        let mut other: BTreeMap<SecretId, SecretData> = lines
            .filter_map(|line| {
                let line = line.unwrap();
                let mut parts = line.splitn(2, ':');
                let id = parts.next()?;
                let data = parts.next()?.trim_start();
                Some((id.to_string(), data.to_string()))
            })
            .collect();

        // Add the password, only if it is non-empty
        if let Some(password) = password {
            other.insert("password".to_string(), password);
        }

        Self(other)
    }
}

impl From<Cli> for Secrets {
    fn from(value: Cli) -> Self {
        let mut inner = BTreeMap::new();

        if let Some(username) = value.username {
            inner.insert("username".to_string(), username);
        }

        if let Some(password) = value.password {
            inner.insert("password".to_string(), password);
        }

        Self(inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_store_single_line() {
        let input = b"THIS_IS_THE_PASSWORD".as_slice();
        let secrets = Secrets::from(input);
        assert_eq!(secrets.get("password").unwrap(), "THIS_IS_THE_PASSWORD");
    }

    #[test]
    fn test_password_store_multi_line() {
        let input = b"THIS_IS_THE_PASSWORD\nusername: test".as_slice();
        let secrets = Secrets::from(input);
        assert_eq!(secrets.get("password").unwrap(), "THIS_IS_THE_PASSWORD");
        assert_eq!(secrets.get("username").unwrap(), "test");
    }

    #[test]
    fn test_invalid_multiline() {
        let input = b"THIS_IS_THE_PASSWORD\nasdasd\nqweqwe".as_slice();
        let secrets = Secrets::from(input);
        assert_eq!(secrets.get("password").unwrap(), "THIS_IS_THE_PASSWORD");
    }

    #[test]
    fn test_empty_input() {
        let input = b"".as_slice();
        let secrets = Secrets::from(input);
        assert!(secrets.get("password").is_none());
        assert!(secrets.0.is_empty());
    }
}
