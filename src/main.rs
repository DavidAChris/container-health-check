mod api;

use crate::api::health_check;
use actix_web::{App, HttpServer};
use container_health_check::get_eth0_ip;
use log::info;
use std::io;

struct LoggerConfig {
    log_format: &'static str,
    log_level: log::LevelFilter,
    log_file: &'static str,
}

impl LoggerConfig {
    fn set_logger(self) {
        fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format(self.log_format),
                    record.target(),
                    record.level(),
                    message
                ))
            })
            .level(self.log_level)
            .chain(io::stdout())
            .chain(fern::log_file(self.log_file).unwrap())
            .apply()
            .unwrap();
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let log = LoggerConfig {
        log_format: "[%Y-%m-%d][%H:%M:%S]",
        log_level: log::LevelFilter::Debug,
        log_file: "HealthCheck.log",
    };
    log.set_logger();
    let ip = get_eth0_ip();

    info!("Kubernetes Docker Image Health Check");
    info!("IP Address: {}", &ip);
    HttpServer::new(|| App::new().service(health_check))
        .bind((ip, 5000))?
        .run()
        .await
}
