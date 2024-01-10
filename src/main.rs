use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    run(
        TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))
            .expect("Failed to bind port 8000"),
    )?
    .await
}
