use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};
use container_health_check::HealthCheck;
use log::info;

#[get("/healthcheck/{application}")]
pub async fn health_check(application: web::Path<String>) -> impl Responder {
    let application_name = application.into_inner();
    let log_path = String::from("/var/log/ApplicationLogs/appLog.log");
    let app = HealthCheck::new(application_name, log_path);
    info!("Checking Health of {}", &app.app_name());
    if app.check_application() {
        info!("{} status = HEALTHY", &app.app_name());
        HttpResponse::build(StatusCode::NO_CONTENT).await
    } else {
        info!("{} status = UNHEALTHY", &app.app_name());
        HttpResponse::build(StatusCode::NOT_FOUND).await
    }
}
