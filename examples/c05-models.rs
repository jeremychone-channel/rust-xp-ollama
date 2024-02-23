// region:    --- Modules

use xp_ollama::Result;

use futures::StreamExt;
use ollama_rs::models::LocalModel;
use ollama_rs::Ollama;
use std::collections::HashMap;
use xp_ollama::consts::MODEL_NOMIC;

// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
	// By default localhost:11434
	let ollama = Ollama::default();

	let models = ollama.list_local_models().await?;
	let model_by_name: HashMap<&str, &LocalModel> = models
		.iter()
		.map(|model| (model.name.as_ref(), model))
		.collect();

	// -- List the local models
	println!("\n=== List installed models:");
	for model in models.iter() {
		println!("LocalModel {model:?}");
	}

	// -- If we have the test model, we delete it.
	if model_by_name.get(MODEL_NOMIC).is_some() {
		println!("\n=== Deleting LocalModel: {MODEL_NOMIC}");
		ollama.delete_model(MODEL_NOMIC.to_string()).await?;
		println!("DONE");
	}

	// -- Redownload the test model
	println!("\n=== Pulling model: {MODEL_NOMIC}");
	let mut stream = ollama
		.pull_model_stream(MODEL_NOMIC.to_string(), false)
		.await?;

	while let Some(res) = stream.next().await {
		match res {
			Ok(status) => println!("downloading status: {status:?}"),
			Err(ex) => println!("ERROR downloading {MODEL_NOMIC}: {ex:?}"),
		}
	}
	println!("DONE");

	// -- Relist the local models
	let models = ollama.list_local_models().await?;
	println!("\n=== List installed models:");
	for model in models.iter() {
		println!("LocalModel {model:?}");
	}

	Ok(())
}
