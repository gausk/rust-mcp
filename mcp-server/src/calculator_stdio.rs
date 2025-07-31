use common::calculator::Calculator;
use rmcp::transport::stdio;
use rmcp::ServiceExt;
mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Calculator::new().serve(stdio()).await?;
    println!("Calculator server is running...");
    server.waiting().await?;
    Ok(())
}