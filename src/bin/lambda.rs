use anyhow::Result as AnyResult;
use code_executor::exec::{self, ExecRequest};
use code_executor::{handler_error, Error};
use lambda_http::{lambda, Body, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::{self as json};

fn handler(req: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    let res = match req.body() {
        Body::Text(text) => {
            let exec_req: ExecRequest = json::from_str(&text)?;
            let exec_res = exec::exec(&exec_req).map_err(handler_error)?;
            json::to_value(exec_res)?
        }

        _ => return Err(handler_error(Error::InvalidBody)),
    };

    Ok(res)
}

fn main() -> AnyResult<()> {
    lambda!(handler);
    Ok(())
}
