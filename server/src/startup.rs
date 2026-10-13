use std::net::TcpListener;

use actix_cors::Cors;
use actix_web::{App, HttpServer, dev::Server, web};
use aws_sdk_s3::Client as S3Client;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::{configuration::Settings, routes::{complete_multipart_upload, create_multipart_upload, health_check, post, upload_part_presigned}, s3::{get_config, s3client}};

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
        let pool = get_connection_pool(&config);
        let port = listener.local_addr().expect("error reading address").port();
        let client = s3client(get_config().await);
        let bucket = S3Bucket (config.s3.bucket);
        let expires_in = PresignedExpiresIn (config.s3.expires_in);
        let server = run(listener, pool, client, bucket, expires_in).await.expect("failed to start server");

        Application {server, port}
    }

    pub async fn run(self) {
        self.server.await.expect("failed to start server")
    }
}

pub fn get_connection_pool(config: &Settings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(config.database.with_db())
}
pub async fn run(listener: TcpListener, pool: PgPool, s3_client: S3Client, bucket: S3Bucket, expires_in: PresignedExpiresIn ) -> Result<Server, std::io::Error> {

    let pool = web::Data::new(pool);
    let s3_client = web::Data::new(s3_client);
    let bucket = web::Data::new(bucket);
    let expires_in = web::Data::new(expires_in);
    let server = HttpServer::new(
        move || {
            let cors = Cors::default()
                .allow_any_origin()
                .send_wildcard()
                .allow_any_method()
                .allow_any_header();

            App::new()
                .wrap(cors)
                .route("/health_check", web::get().to(health_check))
                .service(
                    web::scope("/files")
                    .route("", web::post().to(post))
                    .route("/create_multipart/{key}", web::get().to(create_multipart_upload))
                    .route("/upload_part_presigned", web::post().to(upload_part_presigned))
                    .route("/complete_multipart_upload", web::post().to(complete_multipart_upload))
                )
                .app_data(pool.clone())
                .app_data(s3_client.clone())
                .app_data(bucket.clone())
                .app_data(expires_in.clone())
        }
    )
    .listen(listener)?
    .run();

    Ok(server)
}