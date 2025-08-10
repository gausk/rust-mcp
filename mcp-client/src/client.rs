use rmcp::{
    RmcpError, model::CallToolRequestParam, service::ServiceExt, transport::TokioChildProcess,
};
use tokio::process::Command;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), RmcpError> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("info,{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let client = ()
        .serve(
            TokioChildProcess::new(Command::new("target/debug/examples/calculator-stdio"))
                .map_err(RmcpError::transport_creation::<TokioChildProcess>)?,
        )
        .await?;

    // Initialize
    let server_info = client.peer_info();
    tracing::info!("Connected to server: {server_info:#?}");

    // List tools
    let tools = client.list_tools(Default::default()).await?;
    tracing::info!("Available tools: {tools:#?}");

    // Call tool 'add' with arguments = { "a": 2, "b": 3 }
    let tool_result = client
        .call_tool(CallToolRequestParam {
            name: "add".into(),
            arguments: serde_json::json!({ "a": 2, "b": 3 }).as_object().cloned(),
        })
        .await?;
    tracing::info!("Tool result for add: {tool_result:#?}");

    // Call tool 'subtract' with arguments = { "a": 5, "b": 2 }
    let tool_result = client
        .call_tool(CallToolRequestParam {
            name: "subtract".into(),
            arguments: serde_json::json!({ "a": 5, "b": 2 }).as_object().cloned(),
        })
        .await?;
    tracing::info!("Tool result for subtract: {tool_result:#?}");
    client.cancel().await?;
    Ok(())
}
