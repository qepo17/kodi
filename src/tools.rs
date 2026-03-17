use std::process::Command;

pub fn execute_tool(name: &str, arguments: &str) -> String {
    match name {
        "bash" => {
            let args: serde_json::Value = serde_json::from_str(arguments).unwrap_or_default();
            let cmd = args["command"].as_str().unwrap_or("echo 'no command'");

            let output = Command::new("sh").arg("-c").arg(cmd).output();

            match output {
                Ok(o) => {
                    let stdout = String::from_utf8_lossy(&o.stdout);
                    let stderr = String::from_utf8_lossy(&o.stderr);
                    format!("{}{}", stdout, stderr)
                }
                Err(err) => {
                    format!("{}", err)
                }
            }
        }
        _ => format!("Unknown tool: {}", name),
    }
}
