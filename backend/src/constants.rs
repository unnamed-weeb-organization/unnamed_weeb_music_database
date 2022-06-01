use hyper::header::HeaderValue;
use std::net::{IpAddr, Ipv4Addr};

// Environment Variables
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const ENV_CONFIG_PATH: &str = "UNK_DB_CONFIG";

pub const SERVER_DEFAULT_PORT: u16 = 6001;
pub const SERVER_DEFAULT_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
pub static ALLOWED_CONTROL_HOSTS: HeaderValue = HeaderValue::from_static("*");
