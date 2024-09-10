use actix_web::middleware;
use actix_web::{App, HttpServer};

use {{crate_name}}::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();

    let socket_addr = rustx::env::get_socket_addrs()?;

    HttpServer::new(move || {
        App::new()
            // --- Global middleware
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(actix_cors::Cors::default())
        // --- App state

        // --- Route handlers
    })
    .bind(socket_addr)?
    .run()
    .await?;

    Ok(())
}
