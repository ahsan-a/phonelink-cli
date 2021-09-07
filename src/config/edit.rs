use ansi_term::Colour::{Green, Red};
use preferences::{AppInfo, Preferences, PreferencesMap};
use std::path::Path;

pub fn save_config(config: &PreferencesMap<String>, app_info: &AppInfo) {
    assert!(config.save(&app_info, whoami::username()).is_ok());
}

pub fn edit_config_menu(config: &mut PreferencesMap<String>, app_info: AppInfo) {
    print!("\x1B[2J\x1B[1;1H");
    let edit = inquire::Select::new(
        "What would you like to edit?",
        vec![
            "Port",
            "Enable Password",
            "Password",
            "Save File Path",
            "Exit",
        ],
    )
    .with_vim_mode(true)
    .without_help_message()
    .prompt()
    .unwrap();

    match edit {
        "Port" => {
            let port: u16 = inquire::CustomType::new("Select a port:")
                .with_error_message("Please type a valid number, that is able to be a port")
                .with_placeholder(config.get("port").unwrap())
                .prompt()
                .unwrap();

            config.insert("port".into(), port.to_string());
        }
        "Enable Password" => {
            config.insert(
                "enable_password".into(),
                inquire::Confirm::new("Enable password?")
                    .with_placeholder(config.get("enable_password").unwrap())
                    .prompt()
                    .unwrap()
                    .to_string(),
            );
        }
        "Password" => {
            config.insert(
                "password".into(),
                inquire::Text::new("Choose a password (this goes in the password header):")
                    .with_default(config.get("password").unwrap())
                    .prompt()
                    .unwrap(),
            );
        }
        "Save File Path" => {
            let mut path: String;
            loop {
                path = inquire::Text::new(
                    "Choose a path to save files (Path must be absolute, folder must exist):",
                )
                .with_default(config.get("save_path").unwrap())
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
            config.insert("save_path".into(), path);
        }
        "Exit" => {
            print!("\x1B[2J\x1B[1;1H");
            println!("{}", Green.paint("Have a nice day!"));
            std::process::exit(0);
        }
        _ => (),
    }

    save_config(config, &app_info);
    edit_config_menu(config, app_info)
}
