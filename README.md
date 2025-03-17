# BudouX Rust Implementation

Unofficial Rust implementation of BudouX, the machine learning powered line break organizer tool.

## About BudouX

BudouX is the successor to [Budou](https://github.com/google/budou), providing standalone, small, and language-neutral line break organization for Japanese, Chinese, and Thai text.

## Languages Supported

- Japanese
- Simplified Chinese
- Traditional Chinese
- Thai

## Usage

### Basic Usage

```rust
use budoux_rs::Parser;

// Load the default Japanese parser
#[cfg(feature = "ja")]
let parser = Parser::load_default_japanese_parser();

// Parse a sentence into semantic chunks
let chunks = parser.parse("今日は天気です。");
// Returns ["今日は", "天気です。"]
```

## Features

```toml
# Cargo.toml
[dependencies]
budoux-rs = { git = "https://github.com/yu7400ki/budoux-rs", features = ["ja", "zh-hans", "zh-hant", "th"] }
```

Available features:
- `ja`: Japanese model
- `zh-hans`: Simplified Chinese model
- `zh-hant`: Traditional Chinese model
- `th`: Thai model

## Caveat

This implementation is unofficial and not affiliated with Google or the original BudouX project.

## License

[Apache License 2.0](LICENSE)

## See Also

- [Original BudouX Project](https://github.com/google/budoux)
