mod error;
mod exec;

use actix_cors::Cors;
use actix_web::middleware::Logger as ActixLogger;
use actix_web::{post, web};
use actix_web::{App, HttpResponse, HttpServer};
use anyhow::Error as AnyError;
use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use lambda_http::{lambda, Body, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::json;

use crate::exec::ExecRequest;

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(json!({
        "error_type": "not_found",
        "message": "Requested resource/route wasn't found"
    }))
}

#[post("/")]
async fn exec_python(req: web::Json<ExecRequest>) -> HttpResponse {
    match exec::exec_req(&req) {
        Ok(output) => HttpResponse::Ok().json(output),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "error_type": "internal",
            "message": err.to_string(),
        })),
    }
}

fn lambda_exec(req: Request, _c: Context) -> Result<impl IntoResponse, HandlerError> {
    let res = match req.body() {
        Body::Text(text) => {
            let exec_req: ExecRequest = serde_json::from_str(&text)?;
            match exec::exec_req(&exec_req) {
                Ok(res) => serde_json::to_value(res)?,
                Err(e) => json!({ "error_type": "internal", "message": e.to_string() }),
            }
        }

        _ => json!({ "error_type": "bad_request", "message": "Invalid body" }),
    };

    Ok(res)
}

#[actix_rt::main]
async fn main() -> Result<(), AnyError> {
    let args = std::env::args().collect::<Vec<String>>();
    let lambda = matches!(args.get(1).map(|s| s.as_str()), Some("--lambda"));

    if lambda {
        lambda!(lambda_exec);
    } else {
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
    }

    Ok(())
}
