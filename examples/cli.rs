use clap::{Parser, ValueEnum};

#[derive(Parser)]
struct Cli {
    #[clap(short, long, value_enum)]
    lang: Language,
    text: String,
}

#[derive(Clone, Copy, ValueEnum)]
enum Language {
    #[cfg(feature = "ja")]
    #[clap(name = "ja")]
    Japanese,
    #[cfg(feature = "zh-hans")]
    #[clap(name = "zh-hans")]
    SimplifiedChinese,
    #[cfg(feature = "zh-hant")]
    #[clap(name = "zh-hant")]
    TraditionalChinese,
    #[cfg(feature = "th")]
    #[clap(name = "th")]
    Thai,
}

fn main() {
    let args = Cli::parse();
    let parser = match args.lang {
        #[cfg(feature = "ja")]
        Language::Japanese => budoux_rs::Parser::load_default_japanese_parser(),
        #[cfg(feature = "zh-hans")]
        Language::SimplifiedChinese => budoux_rs::Parser::load_default_simplified_chinese_parser(),
        #[cfg(feature = "zh-hant")]
        Language::TraditionalChinese => budoux_rs::Parser::load_default_traditional_chinese_parser(),
        #[cfg(feature = "th")]
        Language::Thai => budoux_rs::Parser::load_default_thai_parser(),
    };

    let text = args.text.trim();
    let result = parser.parse(text);

    println!("{}", result.join("\n"));
}
