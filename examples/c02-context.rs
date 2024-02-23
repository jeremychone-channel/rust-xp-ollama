// region:    --- Modules

use xp_ollama::consts::MODEL_MIXTRAL;
use xp_ollama::gen::gen_stream_print;
use xp_ollama::Result;

use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::completion::GenerationContext;
use ollama_rs::Ollama;
use simple_fs::{ensure_file_dir, save_json};

// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
	// By default localhost:11434
	let ollama = Ollama::default();

	let prompts = &[
		"Why the sky is red? (be concise)",
		"What was my first question?",
	];

	let mut last_ctx: Option<GenerationContext> = None;

	for prompt in prompts {
		println!("\n->> prompt: {prompt}");
		let mut gen_req =
			GenerationRequest::new(MODEL_MIXTRAL.to_string(), prompt.to_string());

		if let Some(last_ctx) = last_ctx.take() {
			gen_req = gen_req.context(last_ctx);
		}

		let mut final_data_list = gen_stream_print(&ollama, gen_req).await?;

		// NOTE: For now, just take the latest fina_data (should be fine in this usecase)
		if let Some(final_data) = final_data_list.pop() {
			last_ctx = Some(final_data.context);

			// Save for debug
			let ctx_file_path = ".c02-data/ctx.json";
			ensure_file_dir(ctx_file_path)?;
			save_json(ctx_file_path, &last_ctx)?;
		}
	}

	Ok(())
}
