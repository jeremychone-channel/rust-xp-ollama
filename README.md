# Rust Ollama Code Examples

- [YouTube Rust Ollama Video](https://www.youtube.com/watch?v=OcH-zT5VNgM&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)
	- [GitHub Repo Tag: E01](https://github.com/jeremychone-channel/rust-xp-ollama/tree/E01)
	- [Fix for ollama-rs 0.1.6 - Tag: E01-01](https://github.com/jeremychone-channel/rust-xp-ollama/tree/E01-01)

- Related Links: 
	- [Ollama API documentation](https://github.com/ollama/ollama/blob/main/docs/api.md#generate-a-chat-completion)
	- [Learn Rust OpenAI API - Building AI Buddy from Scratch!!!](https://www.youtube.com/watch?v=PHbCmIckV20&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)
	- [Rust10x VSCode Extension](https://rust10x.com/vscode)

> **IMPORTANT** 
`ollama-rs 0.1.6` introduced a change where `stream.next()` now returns a `Result<Vec<GenerationResponse>>` instead of a single `GenerationResponse`. The fix is straightforward. It has been implemented in the main branch and tagged as [E01-01](https://github.com/jeremychone-channel/rust-xp-ollama/tree/E01-01).


## Cargo Runs & Watches

```sh
cargo run --example c01-simple
cargo watch -q -c -x "run -q --example c01-simple"

cargo run --example c02-context
cargo watch -q -c -x "run -q --example c02-context"

cargo run --example c03-chat
cargo watch -q -c -x "run -q --example c03-chat"

cargo run --example c04-embeddings
cargo watch -q -c -x "run -q --example c04-embeddings"

cargo run --example c05-models
cargo watch -q -c -x "run -q --example c05-models"
```

## Some `.sh/.zsh` aliases / functions

```sh
# Cargo watch quiet clear (user need to add the -x)
alias cw="cargo watch -q -c"

# Cargo watch run
alias cwr="cargo watch -q -c -w src/ -x 'run -q'"

# watch example
function cwe() {
  cargo watch -q -c -x "run -q --example '$1'"
}
```

## Other Links

- [ollama-rs](https://github.com/pepperoni21/ollama-rs)

- [ollama Official API](https://github.com/jmorganca/ollama/blob/main/docs/api.md)


<br />

[This repo on GitHub](https://github.com/jeremychone-channel/rust-xp-ollama)