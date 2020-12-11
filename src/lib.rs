pub mod exec;

use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Couldn't execute the Python code ({source})")]
    ExecutionFail {
        #[from]
        source: io::Error,
    },

    #[cfg(feature = "lambda")]
    #[error("Bad request, received invalid body")]
    InvalidBody,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "lambda")] {
        use lambda_runtime::error::HandlerError;
        use std::error::Error as StdError;

        pub fn handler_error<E: StdError>(e: E) -> HandlerError {
            let message = e.to_string();
            HandlerError::from(message.as_str())
        }
    }
}
