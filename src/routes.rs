mod link;
pub use link::link_route;

mod notification;
pub use notification::notif;

mod file;
pub use file::file_route;

use preferences::PreferencesMap;
pub struct AppState {
    pub config: PreferencesMap<String>,
}

mod password;
use password::check_password;
