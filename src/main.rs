use anyhow::{anyhow, Result};
use base64::prelude::*;
use clap::Parser;
use colored::*;
use serde_json::Value;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The JWT string (if not provided, reads from stdin)
    token: Option<String>,
}

fn parse_and_print(part_name: &str, encoded: &str, color: colored::Color) -> Result<()> {
    let decoded = match BASE64_URL_SAFE_NO_PAD.decode(encoded) {
        Ok(d) => d,
        Err(_) => BASE64_URL_SAFE
            .decode(encoded)
            .map_err(|e| anyhow!("Failed to decode base64: {}", e))?,
    };

    let json: Value =
        serde_json::from_slice(&decoded).map_err(|e| anyhow!("Failed to parse JSON: {}", e))?;

    println!("{}:", part_name.color(color).bold());
    println!("{}\n", serde_json::to_string_pretty(&json)?);

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut token = String::new();
    if let Some(t) = args.token {
        token = t;
    } else {
        io::stdin().read_to_string(&mut token)?;
    }

    let token = token.trim();
    if token.is_empty() {
        return Err(anyhow!("No JWT provided"));
    }

    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(anyhow!(
            "Invalid JWT format. Expected 3 parts separated by dots."
        ));
    }

    parse_and_print("HEADER", parts[0], Color::Red)?;
    parse_and_print("PAYLOAD", parts[1], Color::Magenta)?;

    println!("{}: {}", "SIGNATURE".blue().bold(), parts[2]);

    Ok(())
}
