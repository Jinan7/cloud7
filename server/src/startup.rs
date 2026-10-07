use std::net::TcpListener;

use actix_web::{App, HttpServer, dev::Server, web};

use crate::{configuration::{Settings, get_configuration}, routes::health_check};

pub struct Application {
    server: Server,
    pub port: u16,
}

impl Application {

    pub async fn build(config: Settings) -> Self {


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
                .route("/health_check", web::get().to(health_check))
        }
    )
    .listen(listener)?
    .run();

    Ok(server)
}