mod edit;

use ansi_term::Colour::{Green, Red};

use preferences::{AppInfo, Preferences, PreferencesMap};
use std::path::Path;

pub const APP_INFO: AppInfo = AppInfo {
    name: "phonelink-rs",
    author: "ahsan-a",
};
const CONFIG_KEYS: [&str; 4] = ["port", "password", "enable_password", "save_path"];

pub fn config_check() {
    let config = PreferencesMap::<String>::load(&APP_INFO, whoami::username()).unwrap();

    for key in config.keys() {
        if !CONFIG_KEYS.contains(&key.as_str()) {
            get_config();
            break;
        }
    }

    for key in CONFIG_KEYS {
        if !config.contains_key(key) {
            get_config();
            break;
        }
    }
}

pub fn config_menu() {
    print!("\x1B[2J\x1B[1;1H");
    println!("{}\n", Green.paint("Phonelink for Rust - Config"));

    let mut config = PreferencesMap::<String>::load(&APP_INFO, whoami::username()).unwrap();

    let ans = inquire::Select::new(
        "What would you like to do?",
        vec!["View config", "Edit config", "Exit"],
    )
    .with_vim_mode(true)
    .without_help_message()
    .prompt()
    .unwrap();

    match ans {
        "View config" => println!("{:#?}", config),
        "Edit config" => edit::edit_config_menu(&mut config, APP_INFO),
        "Exit" => {
            print!("\x1B[2J\x1B[1;1H");
            println!("{}", Green.paint("Have a nice day!"));
            std::process::exit(0);
        }
        _ => (),
    }
}

fn get_config() {
    print!("\x1B[2J\x1B[1;1H");

    println!(
        "{}",
        Red.paint("It appears that you don't have a config yet. Starting setup... \n")
    );
    let port: u16 = inquire::CustomType::new("Select a port:")
        .with_error_message("Please type a valid number, that is able to be a port")
        .with_placeholder("1234")
        .prompt()
        .unwrap();
    let enable_password: bool = inquire::Confirm::new("Enable password?")
        .with_default(true)
        .prompt()
        .unwrap();
    let mut password = String::from("");
    if enable_password {
        password = inquire::Text::new("Choose a password (this goes in the password header):")
            .with_placeholder("1234")
            .prompt()
            .unwrap();
    }

    let mut path: String;

    loop {
        path = inquire::Text::new(
            "Choose a path to save files (Path must be absolute, folder must exist):",
        )
        .with_default("~/")
        .prompt()
        .unwrap();

        // append slash
        if !(path.ends_with("/") || path.ends_with("\\")) {
            path.push_str("/");
        }

        // check if path exists
        if !Path::new(&path).exists() {
            println!("{}", Red.paint(format!("{} does not exist. Please create this directory and reinput, or select a new path.", path)));
            continue;
        }

        break;
    }
    println!(
        "\nPort: {} \nPassword: {} \nPassword Enabled: {} \nSave file path: {}",
        port, password, enable_password, path
    );

    let mut config: PreferencesMap<String> = PreferencesMap::new();
    config.clear();

    config.insert("port".into(), port.to_string());
    config.insert("enable_password".into(), enable_password.to_string());
    config.insert("password".into(), password);
    config.insert("save_path".into(), path);

    edit::save_config(&config, &APP_INFO);

    println!("\n{}", Green.paint("Saved config!"));
}
