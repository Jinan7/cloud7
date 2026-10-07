use std::net::TcpListener;

use actix_web::{App, HttpServer, dev::Server};

use crate::configuration::get_configuration;

pub struct Application {
    server: Server,
    port: u16,
}

impl Application {

    pub async fn build() -> Self {

        let config = get_configuration().expect("failed to get configuration");

        let listener = TcpListener::bind(format!("{}:{}", config.application.host, config.application.port)).expect("failed to bind address");

        let port = listener.local_addr().expect("error reading address").port();
        let server = run(listener).await.expect("failed to start server");

        Application {server, port}
    }

    pub async fn run(self) {
        self.server.await.expect("failed to start server")
    }
}

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {

    let server = HttpServer::new(
        || {
            App::new()
        }
    )
    .listen(listener)?
    .run();

    Ok(server)
}