mod error;
mod exec;

use actix_cors::Cors;
use actix_web::middleware::Logger as ActixLogger;
use actix_web::{post, web};
use actix_web::{App, HttpResponse, HttpServer};
use anyhow::Error as AnyError;
use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use lambda_runtime::{error::HandlerError, lambda, Context};
use serde_json::json;

use crate::exec::{ExecRequest, ExecResponse};

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

fn lambda_handler(req: ExecRequest, _c: Context) -> Result<ExecResponse, HandlerError> {
    match exec::exec_req(&req) {
        Ok(res) => Ok(res),
        Err(e) => {
            let msg = e.to_string();
            Err(HandlerError::from(msg.as_str()))
        }
    }
}

#[actix_rt::main]
async fn main() -> Result<(), AnyError> {
    let args = std::env::args().collect::<Vec<String>>();
    let lambda = matches!(args.get(1).map(|s| s.as_str()), Some("--lambda"));

    if lambda {
        lambda!(lambda_handler);
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

#[cfg(test)]
mod tests {
    use crate::exec::{ExecCode, ExecRequest, ExecResponse};
    use lambda_runtime::Context;

    #[test]
    fn test_lambda_handler() {
        let expected_response = ExecResponse {
            stdout: "Hola\n".to_string(),
            stderr: "".to_string(),
            code: Some(0),
            signal: None,
        };

        let lambda_context = Context {
            aws_request_id: "0123456789".to_string(),
            function_name: "test_function_name".to_string(),
            memory_limit_in_mb: 128,
            function_version: "$LATEST".to_string(),
            invoked_function_arn: "arn:aws:lambda".to_string(),
            xray_trace_id: Some("0987654321".to_string()),
            client_context: Option::default(),
            identity: Option::default(),
            log_stream_name: "logStreamName".to_string(),
            log_group_name: "logGroupName".to_string(),
            deadline: 0,
        };

        let lambda_request = ExecRequest {
            code: ExecCode::Line("print(\"Hola\")".to_string()),
        };

        // Check the result is ok
        let result = super::lambda_handler(lambda_request, lambda_context);
        assert_eq!(result.is_err(), false, "Error: {}", result.err().unwrap());

        // Confirm the expected values in result
        let value = result.ok().unwrap();
        assert_eq!(value.stdout, expected_response.stdout);
        assert_eq!(value.stderr, expected_response.stderr);
        assert_eq!(value.code, expected_response.code);
        assert_eq!(value.signal, expected_response.signal);
    }
}
