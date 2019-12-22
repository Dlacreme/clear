use crate::config::Config;

pub fn build_app(config: Config) {
    crate::log::log(format!("Starting to build {}...", config.app.target));
}
