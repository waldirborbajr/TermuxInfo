// main.rs
use std::process::Command;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file if present
    dotenvy::dotenv().ok();

    // Get Telegram credentials from environment
    let bot_token = env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN not set");
    let chat_id = env::var("TELEGRAM_CHAT_ID")
        .expect("TELEGRAM_CHAT_ID not set");

    // Run ifconfig (Termux supports this via `ifconfig` or fallback to `ip addr`)
    let output = run_ifconfig()?;

    // Format the message
    let message = format!(
        "📡 *Termux Network Info*\n```\n{}\n```",
        output.trim()
    );

    // Send via Telegram
    send_telegram(&bot_token, &chat_id, &message).await?;

    println!("Message sent successfully!");
    Ok(())
}

fn run_ifconfig() -> Result<String, Box<dyn std::error::Error>> {
    // Try `ifconfig` first (available in Termux with `pkg install net-tools`)
    if let Ok(out) = Command::new("ifconfig").output() {
        if out.status.success() {
            return Ok(String::from_utf8_lossy(&out.stdout).to_string());
        }
    }

    // Fallback to `ip addr` (usually available by default in Termux)
    let out = Command::new("ip").arg("addr").output()?;
    if !out.status.success() {
        return Err(format!(
            "Both ifconfig and ip addr failed: {}",
            String::from_utf8_lossy(&out.stderr)
        )
        .into());
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

async fn send_telegram(
    bot_token: &str,
    chat_id: &str,
    text: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);

    let client = reqwest::Client::new();
    let resp = client
        .post(&url)
        .form(&[
            ("chat_id", chat_id),
            ("text", text),
            ("parse_mode", "Markdown"),
        ])
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await?;
        return Err(format!("Telegram API error {}: {}", status, body).into());
    }

    Ok(())
}
