pub mod config;
pub mod error;
pub mod middleware;
pub mod startup;
pub mod context;

pub fn get_env(name: &str) -> Option<String> {
    std::env::var(name).ok()
}
