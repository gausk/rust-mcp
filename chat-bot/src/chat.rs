use crate::model::CompletionRequest;
use crate::{client::ChatClient, model::Message, tool::ToolSet};
use anyhow::Result;
use std::io;
use std::{io::Write, sync::Arc};

pub struct ChatSession {
    pub client: Arc<dyn ChatClient>,
    pub tool_set: ToolSet,
    pub model: String,
    pub messages: Vec<Message>,
}

impl ChatSession {
    pub fn new(client: Arc<dyn ChatClient>, tool_set: ToolSet, model: String) -> Self {
        Self {
            client,
            model,
            tool_set,
            messages: Vec::new(),
        }
    }

    pub fn add_system_prompt(&mut self, prompt: impl ToString) {
        self.messages.push(Message::system(prompt))
    }

    pub async fn chat(&mut self, _support_tool: bool) -> Result<()> {
        println!("Welcome to Gaurav Chat Bot, use 'exit' to quit");

        loop {
            print!("> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input = input.trim().to_string();

            if input.is_empty() {
                continue;
            }

            if input == "exit" {
                break;
            }

            self.messages.push(Message::user(&input));

            let request = CompletionRequest {
                model: self.model.clone(),
                messages: self.messages.clone(),
                temprature: Some(0.7),
                tools: None,
            };

            let response = self.client.complete(request).await?;

            let choice = response.choice.first().unwrap();
            println!("AI > {:?}", choice);
        }

        Ok(())
    }
}
