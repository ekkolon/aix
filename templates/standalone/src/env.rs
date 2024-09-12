use crate::Result;

use std::net::{IpAddr, SocketAddr};

/// When running the app from a Docker container, we usually don't copy .env
/// files as part of or build output to the final bin target folder.
/// Therefore, attempting to load .env files will cause the programm to panic.
pub fn init() -> Result<()> {
    if is_docker() {
        return Ok(());
    }

    dotenvy::dotenv()?;
    Ok(())
}

/// Check if an executable is running in a Docker container
/// based on an environment variable.
pub fn is_docker() -> bool {
    std::env::var("IS_DOCKER")
        .ok()
        .map_or(false, |val| val.parse::<bool>().unwrap_or(false))
}

pub fn get_socket_addrs() -> Result<SocketAddr> {
    let port = get_port()?;
    let host = get_host()?;
    let addr = SocketAddr::new(host, port);
    Ok(addr)
}

const LOCAL_HOST: &str = "127.0.0.1";
const DOCKER_HOST: &str = "0.0.0.0";

/// Return the hostname to bind an application to based on the the
/// current environment type.
///
/// In general, use 127.0.0.1:<port> when running locally and 0.0.0.0:<port>
/// when running on or emulating a remote host (e.g. Docker).
fn get_host() -> Result<IpAddr> {
    let host = if is_docker() { DOCKER_HOST } else { LOCAL_HOST };
    let ip_addr = host.parse::<IpAddr>().unwrap(); // Safe unwrap here
    Ok(ip_addr)
}

/// Get the `PORT` environment variable.
fn get_port() -> Result<u16> {
    let port = std::env::var("PORT")
        .unwrap_or("8080".into())
        .parse::<u16>()
        .unwrap_or(8080);
    Ok(port)
}
