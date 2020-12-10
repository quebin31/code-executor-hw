#[cfg(not(feature = "lambda"))]
compile_error!("Feature `lambda` should be enabled to compile this binary");

use anyhow::Result as AnyResult;
use code_executor::exec::{self, ExecRequest};
use lambda_http::{lambda, Body, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::json;

fn lambda_handler(req: Request, _c: Context) -> Result<impl IntoResponse, HandlerError> {
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

fn main() -> AnyResult<()> {
    lambda!(lambda_handler);
    Ok(())
}
