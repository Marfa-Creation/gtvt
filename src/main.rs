use std::error::Error;

use handle_args::args_handler;
use handle_response::handle_response;

mod handle_response;
mod handle_args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let (response, long, status) = args_handler().await.map_err(|e| format!("{}", e))?;

    handle_response(response, long, status);
    return Ok(());
}
