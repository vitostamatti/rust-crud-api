use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use crud_rest_api::configuration::get_configuration;
use crud_rest_api::database::get_db;
use crud_rest_api::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = match get_configuration() {
        Err(err) => {
            eprintln!("🔥 Failed to read configurations: {:?}", err);
            std::process::exit(1);
        }
        Ok(config) => {
            println!("✅ Configuration read successfully!");
            config
        }
    };

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let pool = match get_db(&config.db).await {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("✅ Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind((config.app.host, config.app.port))?
    .run()
    .await
}
