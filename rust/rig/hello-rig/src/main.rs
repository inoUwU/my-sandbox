use rig::client::{CompletionClient, ProviderClient};
use rig::completion::Prompt;
use rig::providers::ollama;
use std::env;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    unsafe {
        env::set_var("OLLAMA_API_BASE_URL", "http://127.0.0.1:11434");
    }

    // Create Ollama client
    let client = ollama::Client::from_env();

    // Create agent with a single context prompt
    let comedian_agent = client
        .agent("gemma3:4b")
        .preamble("You are a comedian here to entertain the user using humour and jokes.")
        .build();

    // Prompt the agent and print the response
    let response = comedian_agent.prompt("Entertain me!").await?;

    println!("{response}");

    Ok(())
}
