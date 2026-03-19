use crate::models::*;
use crate::tools::execute_tool;
use anyhow::Result;

const API_URL: &str = "https://openrouter.ai/api/v1/chat/completions";

pub async fn agent_loop(client: &reqwest::Client, api_key: &str, query: &str) -> Result<()> {
    let mut messages: Vec<Message> = vec![
        Message::system("You are a helpful assistant."),
        Message::user(query),
    ];

    loop {
        let body = client
            .post(API_URL)
            .bearer_auth(api_key)
            .json(&Request {
                model: "anthropic/claude-sonnet-4".to_string(),
                messages: messages.clone(),
                tools: get_tools(),
                max_tokens: 4096,
            })
            .send()
            .await?
            .text()
            .await?;

        let resp: Response = serde_json::from_str(&body)?;

        if let Some(err) = &resp.error {
            return Err(anyhow::anyhow!("API error: {}", err.message));
        }

        let choice = &resp
            .choices
            .as_ref()
            .and_then(|c| c.first())
            .ok_or_else(|| anyhow::anyhow!("No choices in response"))?;

        // Add assistant message to history
        messages.push(Message::Assistant {
            content: choice.message.content.clone(),
            tool_calls: choice.message.tool_calls.clone(),
        });

        // if no tools calls, we're done
        if choice.finish_reason != "tool_calls" {
            if let Some(text) = &choice.message.content {
                println!("{}", text)
            }
            return Ok(());
        }

        if let Some(tool_calls) = &choice.message.tool_calls {
            for tc in tool_calls {
                let output = execute_tool(&tc.function.name, &tc.function.arguments);
                messages.push(Message::Tool {
                    tool_call_id: tc.id.clone(),
                    content: output.unwrap_or_else(|e| format!("Error: {e}")),
                });
            }
        }
    }
}

fn get_tools() -> Vec<Tool> {
    vec![
        Tool {
            r#type: "function".to_string(),
            function: ToolFunction {
                name: "bash".to_string(),
                description: "Run a bash command".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "command": {
                            "type": "string",
                            "description": "The bash command to run"
                        }
                    },
                    "required": ["command"]
                }),
            },
        },
        Tool {
            r#type: "function".to_string(),
            function: ToolFunction {
                name: "read_file".to_string(),
                description: "Read the contents of a file at the given path".to_string(),
                parameters: serde_json::json!({
                   "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path of the file"
                        },
                        "limit": {
                            "type":
                            "integer",
                            "description": "Limit lines to see the file"
                        }
                    },
                    "required": ["path"]
                }),
            },
        },
    ]
}
