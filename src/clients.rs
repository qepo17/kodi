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
        let resp: Response = client
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
            .json()
            .await?;

        let choice = &resp.choices[0];

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
                    content: output,
                });
            }
        }
    }
}

fn get_tools() -> Vec<Tool> {
    vec![Tool {
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
    }]
}
