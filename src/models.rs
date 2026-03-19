// OpenAI-compatible format

// === Request types ===

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Request {
    pub model: String,
    pub messages: Vec<Message>,
    pub tools: Vec<Tool>,
    pub max_tokens: u32,
}

#[derive(Serialize, Clone)]
#[serde(tag = "role")]
pub enum Message {
    #[serde(rename = "system")]
    System { content: String },
    #[serde(rename = "user")]
    User { content: String },
    #[serde(rename = "assistant")]
    Assistant {
        content: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ToolCall>>,
    },
    #[serde(rename = "tool")]
    Tool {
        tool_call_id: String,
        content: String,
    },
}

impl Message {
    pub fn system(text: &str) -> Self {
        Message::System {
            content: text.to_string(),
        }
    }
    pub fn user(text: &str) -> Self {
        Message::User {
            content: text.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct Tool {
    pub r#type: String,
    pub function: ToolFunction,
}

#[derive(Serialize)]
pub struct ToolFunction {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

// === Response types ===

#[derive(Deserialize)]
pub struct Response {
    pub choices: Option<Vec<Choice>>,
    pub error: Option<ApiError>,
}

#[derive(Deserialize)]
pub struct ApiError {
    pub message: String,
}

#[derive(Deserialize)]
pub struct Choice {
    pub message: AssistantMessage,
    pub finish_reason: String, // "stop" or "tool_calls"
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AssistantMessage {
    role: String,
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ToolCall {
    pub id: String,
    r#type: String, // always "function"
    pub function: FunctionCall,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String, // JSON string, not object — you parse it yourself
}
