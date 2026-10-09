use std::net::TcpListener;

use actix_web::{App, HttpServer, dev::Server, web};

use crate::{configuration::Settings, routes::{health_check,create_multipart_upload}};

pub struct Application {
    server: Server,
    pub port: u16,
}

#[derive(Clone)]
pub struct S3Bucket( pub String );

impl Application {

    pub async fn build(config: Settings) -> Self {


        let listener = TcpListener::bind(format!("{}:{}", config.application.host, config.application.port)).expect("failed to bind address");

        let port = listener.local_addr().expect("error reading address").port();

        let bucket = S3Bucket (config.s3.bucket);
        let server = run(listener, bucket).await.expect("failed to start server");

        Application {server, port}
    }

    pub async fn run(self) {
        self.server.await.expect("failed to start server")
    }
}

pub async fn run(listener: TcpListener, bucket: S3Bucket ) -> Result<Server, std::io::Error> {

    let bucket = web::Data::new(bucket);
    let server = HttpServer::new(
        move || {
            App::new()
                .route("/health_check", web::get().to(health_check))
                .service(
                    web::scope("/files")
                    .route("/create_multipart/{key}", web::get().to(create_multipart_upload))
                )
                .app_data(bucket.clone())
        }
    )
    .listen(listener)?
    .run();

    Ok(server)
}