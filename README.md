# MCP Rust

This repository contains three main components:

- **mcp-server**: A Model Context Protocol (MCP) server exposing basic calculator tools (add, subtract).
- **mcp-client**: A client that connects to the MCP server and calls its tools.
- **chat-bot**: A chatbot implementation using the MCP client.

## Testing the MCP Server

You can inspect and interact with the server using the MCP Inspector:

```sh
npx @modelcontextprotocol/inspector cargo run --example calculator-stdio
```

Or run the server directly:

```sh
cargo run --example calculator-stdio
```

## Running the MCP Client

First, build the server:

```sh
cargo build --example calculator-stdio
```
Then, run the MCP client to connect to the server and use its tools:

```sh
cargo run --example simple-client
```

## Chat-bot

The `chat-bot` crate demonstrates using the MCP client for conversational AI. See its directory for details.