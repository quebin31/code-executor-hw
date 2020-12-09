pub mod error;
pub mod exec;

use actix_cors::Cors;
use actix_web::middleware::Logger as ActixLogger;
use actix_web::{post, web};
use actix_web::{App, HttpResponse, HttpServer};
use anyhow::Error as AnyError;
use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use serde_json::json;

use crate::exec::{ExecCode, ExecInput};

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(json!({
        "error_type": "not_found",
        "message": "Requested resource/route wasn't found"
    }))
}

#[post("/exec-python")]
async fn exec_python(input: web::Json<ExecInput>) -> HttpResponse {
    let code = match &input.code {
        ExecCode::Line(line) => line.to_owned(),
        ExecCode::Multi(lines) => {
            let mut code = String::new();
            for line in lines {
                code.push_str(&line);
                code.push('\n');
            }

            code
        }
    };

    match exec::python(&code) {
        Ok(output) => HttpResponse::Ok().json(output),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "error_type": "internal",
            "message": err.to_string(),
        })),
    }
}

#[actix_rt::main]
async fn main() -> Result<(), AnyError> {
    dotenv::dotenv().ok();

    let colors = ColoredLevelConfig::default()
        .info(Color::Cyan)
        .trace(Color::BrightBlue)
        .debug(Color::BrightMagenta);

    Dispatch::new()
        .level(log::LevelFilter::Info)
        .level_for("code_executor", log::LevelFilter::Debug)
        .level_for("actix_server", log::LevelFilter::Debug)
        .chain(
            Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "{}[{}][{}] {}",
                        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                        colors.color(record.level()),
                        record.target(),
                        message
                    ))
                })
                .chain(std::io::stdout()),
        )
        .apply()?;

    log::info!("Loaded .env file (if exists)");
    log::info!("Current directory: {}", std::env::current_dir()?.display());

    let port = envmnt::get_or("PORT", "8000");
    let address = format!("0.0.0.0:{}", port);
    log::info!("Address is {}", address);

    let app_factory = || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .wrap(ActixLogger::default())
            .service(exec_python)
            .default_service(web::to(not_found))
    };

    log::info!("Initializing http server");
    HttpServer::new(app_factory).bind(address)?.run().await?;

    log::info!("Bye");
    Ok(())
}
