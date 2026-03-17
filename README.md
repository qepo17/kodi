# kodi

An AI coding agent built in Rust. Kodi connects to LLMs via OpenRouter and gives them the ability to execute tools — starting with a bash shell — to autonomously complete coding tasks.

The goal is to build a fully-featured coding agent that can understand codebases, write and edit code, run tests, and iterate on solutions with minimal human intervention.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (edition 2024)
- An [OpenRouter](https://openrouter.ai/) API key

### Setup

```sh
cp .env.example .env
# Add your OpenRouter API key to .env
```

### Usage

```sh
cargo run -- "your query here"
```

### Build for release

```sh
cargo build --release
./target/release/kodi "your query here"
```

## License

[MIT](LICENSE)
