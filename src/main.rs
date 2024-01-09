use clap::{Parser, Subcommand, ValueEnum};
use passmumbler::pass::{list_entries, load_secrets};
use passmumbler::select::SelectTool;
use passmumbler::{select, Secrets};
use std::collections::BTreeMap;

use gtk4 as gtk;

use gtk::{glib, prelude::*, Application};
use passmumbler::show::gtk4::build_show_ui;
use passmumbler::show::Props;

const APPLICATION_ID: &str = "net.pezzato.passmumbler";

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id(APPLICATION_ID)
        .build();
    application.connect_activate(build_ui);

    let args: &[&str] = &[];
    application.run_with_args(args)
}

fn build_ui(application: &Application) {
    let cli = Cli::parse();

    match cli.command {
        Commands::Show(cli) => {
            let title = cli.title.clone();
            let secrets = Secrets::try_from(cli).unwrap();
            let props = Props { title, secrets };
            build_show_ui(props, application);
        }
        Commands::Select(cli) => {
            let (title, secrets) = select_and_load_secrets(cli);
            let title = Some(title);
            let props = Props { title, secrets };
            build_show_ui(props, application);
        }
    }
}

fn select_and_load_secrets(cli: Select) -> (String, Secrets) {
    let entries = list_entries();

    // Empty string is a valid prefix
    let prefix = cli.prefix.unwrap_or_default();

    let selected = match cli.interface {
        SelectInterface::Rofi => select::rofi::RofiSelectTool.select(&prefix, &entries),
        SelectInterface::Dmenu => select::dmenu::DmenuSelectTool.select(&prefix, &entries),
    };

    let selected = selected.expect("No secret selected");

    let secrets = load_secrets(&selected).unwrap();

    let selected = selected.strip_prefix(&prefix).unwrap().to_string();

    (selected, secrets)
}

#[derive(Parser)]
struct Show {
    /// Title
    #[arg(short = 't', long)]
    title: Option<String>,
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

#[derive(Parser)]
struct Select {
    /// The interface to use to select the secret
    #[arg(value_enum, short = 'i', long, default_value = "dmenu")]
    interface: SelectInterface,
    /// The prefix to use to filter the secrets
    #[arg(short = 'p', long)]
    prefix: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum SelectInterface {
    /// Use rofi to select the secret
    Rofi,
    /// Use dmenu to select the secret
    Dmenu,
}

#[derive(Subcommand)]
enum Commands {
    /// Show a window with the secrets to copy to the clipboard
    Show(Show),
    /// Select a secret to copy to the clipboard
    Select(Select),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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
