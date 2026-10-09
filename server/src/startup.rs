use std::net::TcpListener;

use actix_web::{App, HttpServer, dev::Server, web};
use aws_sdk_s3::Client as S3Client;

use crate::{configuration::Settings, routes::{create_multipart_upload, health_check, upload_part_presigned}, s3::{get_config, s3client}};

pub struct Application {
    server: Server,
    pub port: u16,
}

#[derive(Clone)]
pub struct S3Bucket( pub String );
#[derive(Clone)]
pub struct PresignedExpiresIn( pub u64 );

impl Application {

    pub async fn build(config: Settings) -> Self {


        let listener = TcpListener::bind(format!("{}:{}", config.application.host, config.application.port)).expect("failed to bind address");

        let port = listener.local_addr().expect("error reading address").port();
        let client = s3client(get_config().await);
        let bucket = S3Bucket (config.s3.bucket);
        let expires_in = PresignedExpiresIn (config.s3.expires_in);
        let server = run(listener, client, bucket, expires_in).await.expect("failed to start server");

        Application {server, port}
    }

    pub async fn run(self) {
        self.server.await.expect("failed to start server")
    }
}

pub async fn run(listener: TcpListener, client: S3Client, bucket: S3Bucket, expires_in: PresignedExpiresIn ) -> Result<Server, std::io::Error> {

    let client = web::Data::new(client);
    let bucket = web::Data::new(bucket);
    let expires_in = web::Data::new(expires_in);
    let server = HttpServer::new(
        move || {
            App::new()
                .route("/health_check", web::get().to(health_check))
                .service(
                    web::scope("/files")
                    .route("/create_multipart/{key}", web::get().to(create_multipart_upload))
                    .route("/upload_part_presigned", web::post().to(upload_part_presigned))
                )
                .app_data(client.clone())
                .app_data(bucket.clone())
                .app_data(expires_in.clone())
        }
    )
    .listen(listener)?
    .run();

    Ok(server)
}