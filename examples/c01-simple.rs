// region:    --- Modules

use xp_ollama::consts::{DEFAULT_SYSTEM_MOCK, MODEL_MIXTRAL};
use xp_ollama::Result;

use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use xp_ollama::gen::gen_stream_print;

// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
	// By default localhost:11434
	let ollama = Ollama::default();

	let model = MODEL_MIXTRAL.to_string();
	let prompt = "What is the best programming language? (Be concise)".to_string();

	let gen_req = GenerationRequest::new(model, prompt)
		.system(DEFAULT_SYSTEM_MOCK.to_string());

	// -- Single Response Generation
	// let res = ollama.generate(gen_req).await?;
	// println!("->> res {}", res.response);

	// -- Stream Response Generation
	gen_stream_print(&ollama, gen_req).await?;

	Ok(())
}
