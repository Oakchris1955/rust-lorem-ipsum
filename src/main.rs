mod lorem_ipsum;
use lorem_ipsum::*;

use clap::{Parser, ValueEnum};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Clone)]
#[command(version)]
struct Args {
    /// What kind of text to generate
    #[arg(short, long, value_enum, default_value_t = TextMode::Paragraphs)]
    text_mode: TextMode,

    /// A path to a wordlist to use other than the default
    #[arg(short = 'W', long)]
    wordlist: Option<PathBuf>,

    /// Specify how many words in total / per sentence to generate
    #[arg(short, long = "words", default_value_t = LoremConfig::default().words_count)]
    words_count: UnitCount,

    /// Specify how many sentences into total / per paragraph to generate
    #[arg(short, long = "sentences", default_value_t = LoremConfig::default().sentences_count)]
    sentences_count: UnitCount,

    /// Specify how many paragraphs to generate in total
    #[arg(short, long = "paragraphs", default_value_t = LoremConfig::default().paragraphs_count)]
    paragraphs_count: UnitCount,

    /// Specify about how many words to generate before placing a comma
    #[arg(short, long = "comma", default_value_t = LoremConfig::default().comma_probability)]
    comma_probability: usize,

    /// Whether or not the first sentence / words should be the standard "Lorem ipsum..."
    #[arg(short = 'f', long, action = clap::ArgAction::SetFalse)]
    static_first_sentence: bool,
}

impl From<Args> for LoremConfig {
    fn from(args: Args) -> Self {
        Self {
            words_count: args.words_count,
            sentences_count: args.sentences_count,
            paragraphs_count: args.paragraphs_count,

            comma_probability: args.comma_probability,

            static_first_sentence: args.static_first_sentence,
        }
    }
}

#[derive(ValueEnum, Clone)]
enum TextMode {
    /// Generate a number of words specified by word-count
    Words,
    /// Generate a number of sentences specified by sentence-count
    Sentences,
    /// Generate a number of paragraphs specified by paragraph-count
    Paragraphs,
}

fn main() {
    // Process the cmd-line args
    let args = Args::parse();

    // Obtain a LoremConfig from the args
    let config: LoremConfig = args.clone().into();

    // If the user has supplied a path to a wordlist, use that
    let placeholder: String = if let Some(wordlist_path) = args.wordlist {
        match fs::read_to_string(wordlist_path) {
            Ok(text) => text,
            Err(err) => panic!("{}", err),
        }
    } else {
        DEFAULT_PLACEHOLDER.to_string()
    };

    // Create a generator
    let mut generator = LoremGenerator::new_from_string(placeholder);
    generator.set_config(config);

    let output = match args.text_mode {
        TextMode::Words => generator.generate_words(),
        TextMode::Sentences => generator.generate_sentences(),
        TextMode::Paragraphs => generator.generate_paragraphs(),
    };

    println!("{}", output);
}
