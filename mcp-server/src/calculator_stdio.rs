use common::calculator::Calculator;
use rmcp::ServiceExt;
use rmcp::transport::stdio;
mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Calculator::new().serve(stdio()).await?;
    // println! writes to stdout, it will be sent through
    // the same pipe RMCP uses to communicate.
    tracing::info!("Calculator server is running...");
    server.waiting().await?;
    Ok(())
}
