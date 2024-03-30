use rand::{
    distributions::{Bernoulli, Distribution},
    rngs::ThreadRng,
    seq::SliceRandom,
    thread_rng, Rng,
};
use regex::Regex;
use std::convert::From;
use std::ffi::OsStr;
use std::fmt;
use std::ops;

pub static DEFAULT_PLACEHOLDER: &'static str = include_str!("LOREM_IPSUM.txt");

/// A wrapper to quickly convert [`Option<String>`] or [`Option<&'static str>`] to [`String`]
pub struct Placeholder<S>(pub Option<S>)
where
    S: ToString;

/// By implementing [`fmt::Display`], we also implement the [`ToString`] trait
impl<S> fmt::Display for Placeholder<S>
where
    S: ToString,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match &self.0 {
            Some(into) => into.to_string(),
            None => DEFAULT_PLACEHOLDER.to_string(),
        };
        write!(f, "{}", output)
    }
}

impl<S> From<Option<S>> for Placeholder<S>
where
    S: ToString,
{
    fn from(value: Option<S>) -> Self {
        Self(value)
    }
}

impl<S> From<Placeholder<S>> for String
where
    S: ToString,
{
    fn from(value: Placeholder<S>) -> Self {
        value.to_string()
    }
}

type WordList = Vec<String>;

type Range = ops::RangeInclusive<usize>;

#[derive(Clone)]
pub enum UnitCount {
    Num(usize),
    Range(Range),
}

impl fmt::Display for UnitCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            Self::Num(num) => num.to_string(),
            Self::Range(range) => format!("{}..{}", range.start(), range.end()),
        };
        write!(f, "{}", output)
    }
}

impl From<UnitCount> for usize {
    fn from(value: UnitCount) -> Self {
        match value {
            UnitCount::Num(num) => num,
            UnitCount::Range(range) => thread_rng().gen_range(range),
        }
    }
}

impl clap::builder::ValueParserFactory for UnitCount {
    type Parser = UnitCountParser;
    fn value_parser() -> Self::Parser {
        UnitCountParser
    }
}

#[derive(Clone)]
pub struct UnitCountParser;
impl clap::builder::TypedValueParser for UnitCountParser {
    type Value = UnitCount;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let quick_err = |errorkind: clap::error::ErrorKind, value: Option<&str>| {
            let mut err: clap::error::Error = clap::error::Error::new(errorkind).with_cmd(cmd);

            if let Some(arg) = arg {
                err.insert(
                    clap::error::ContextKind::InvalidArg,
                    clap::error::ContextValue::String(arg.to_string()),
                );
            }

            if let Some(value) = value {
                err.insert(
                    clap::error::ContextKind::InvalidValue,
                    clap::error::ContextValue::String(value.to_string()),
                );
            }

            err
        };

        let str_value = value
            .to_str()
            .ok_or(quick_err(clap::error::ErrorKind::InvalidUtf8, None))?;

        let split: Vec<&str> = str_value.split("..").collect();

        // Local function to quickly pass a &str to a Result<usize>
        let parse_to_num = |value: &str| -> Result<usize, clap::error::Error> {
            Ok(value
                .parse()
                .map_err(|_| quick_err(clap::error::ErrorKind::ValueValidation, Some(str_value)))?)
        };

        match split.len() {
            // Convert to Num variant
            1 => Ok(UnitCount::Num(parse_to_num(split[0])?)),
            // Convert to Range variant
            2 => {
                let start = parse_to_num(split[0])?;
                let end = parse_to_num(split[1])?;

                if start <= end {
                    Ok(UnitCount::Range(Range::new(start, end)))
                } else {
                    Err(quick_err(
                        clap::error::ErrorKind::ValueValidation,
                        Some(str_value),
                    ))
                }
            }
            _ => Err(quick_err(
                clap::error::ErrorKind::ValueValidation,
                Some(str_value),
            )),
        }
    }
}

trait GeneratorPrivate {
    // https://stackoverflow.com/a/38406885/ with some variations
    fn capitalize_first_letter<S>(string: S) -> String
    where
        S: ToString,
    {
        // Generics are probably unnecessary here, but it doesn't matter
        let string = string.to_string();
        let mut chars = string.chars();
        match chars.next() {
            None => String::new(),
            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }

    fn get_first_sentence<S>(placeholder: &Placeholder<S>) -> String
    where
        S: ToString,
    {
        placeholder
            .to_string()
            .split(".")
            .next()
            .unwrap()
            .to_string()
            + "."
    }

    fn get_wordlist<S>(placeholder: &Placeholder<S>) -> Vec<String>
    where
        S: ToString,
    {
        // Convert to String
        let placeholder: String = placeholder.to_string();
        // A very simple regex to remove commas and full stops
        let re = Regex::new(r"[.,]").unwrap();
        // Remove those punctuations signs from the string
        let clean_str: String = re.replace_all(&placeholder, "").to_string();
        // Split the string into different words
        let mut result: Vec<String> = clean_str
            .split_ascii_whitespace()
            .map(|s| s.to_lowercase())
            .collect();
        // Sort the result vector and remove duplicates
        result.sort();
        result.dedup();
        result
    }
}

pub trait GeneratorPublic {
    fn generate_words(&mut self) -> String;
    fn generate_sentences(&mut self) -> String;
    fn generate_paragraphs(&mut self) -> String;
}

impl GeneratorPrivate for LoremGenerator {}
impl GeneratorPublic for LoremGenerator {
    fn generate_words(&mut self) -> String {
        let num = self.config.words_count.clone().into();

        let comma_distribution =
            Bernoulli::new(1.0 / self.config.comma_probability as f64).unwrap();

        self.wordlist
            .choose_multiple(&mut self.rng, num)
            .cloned()
            .enumerate()
            .map(|(index, mut string)| {
                // Randomly add commas
                let trailing_substring = if comma_distribution.sample(&mut self.rng) {
                    ","
                } else {
                    ""
                };

                // If this is the first word of a sentence, capitalize the first letter
                if index == 0 {
                    string = Self::capitalize_first_letter(string)
                }

                string + trailing_substring
            })
            .collect::<Vec<String>>()
            .join(" ")
            .trim_matches(',') // Remove trailing commas
            .to_string()
            + "." // Add a period mark
    }

    fn generate_sentences(&mut self) -> String {
        let num = self.config.sentences_count.clone().into();

        let mut output = String::new();

        for i in 0..num {
            let sentence = self.generate_words();

            output.push_str(
                // If this is the first sentence being generated,
                // push the `first_sentence` field to the output
                if i == 0 && self.config.static_first_sentence {
                    &self.first_sentence
                } else {
                    &sentence
                },
            );
            output += " ";
        }

        // Set the static_first_sentence option to false
        self.config.static_first_sentence = false;

        // Trim the output string
        output.trim().to_string()
    }

    fn generate_paragraphs(&mut self) -> String {
        let num = self.config.paragraphs_count.clone().into();

        let mut output = String::new();

        for _ in 0..num {
            output.push_str(&format!("{}\n\n", self.generate_sentences()))
        }

        // Trim the output string
        output.trim().to_string()
    }
}

/// The range to use when generating sentences
const DEFAULT_WORDS_COUNT: UnitCount = UnitCount::Range(7..=15);
/// The range to use when generating paragraphs
const DEFAULT_SENTENCES_COUNT: UnitCount = UnitCount::Range(3..=5);
/// How many paragraphs to generate by default
const DEFAULT_PARAGRAPHS_COUNT: UnitCount = UnitCount::Num(2);

/// The default average ratio of words per commas
const DEFAULT_COMMA_PROBABILITY: usize = 7;

pub struct LoremConfig {
    pub words_count: UnitCount,
    pub sentences_count: UnitCount,
    pub paragraphs_count: UnitCount,

    pub comma_probability: usize,

    pub static_first_sentence: bool,
}

impl Default for LoremConfig {
    fn default() -> Self {
        Self {
            words_count: DEFAULT_WORDS_COUNT,
            sentences_count: DEFAULT_SENTENCES_COUNT,
            paragraphs_count: DEFAULT_PARAGRAPHS_COUNT,

            comma_probability: DEFAULT_COMMA_PROBABILITY,

            static_first_sentence: true,
        }
    }
}

/// Note: each [`LoremGenerator`] is intended to be used only once
pub struct LoremGenerator {
    rng: ThreadRng,
    wordlist: WordList,
    first_sentence: String,

    pub config: LoremConfig,
}

impl LoremGenerator {
    pub fn new_from_string<S>(string: S) -> Self
    where
        S: ToString,
    {
        let placeholder = Placeholder::from(Some(string));

        Self::new_from_placeholder(&placeholder)
    }

    pub fn new_from_placeholder<S>(placeholder: &Placeholder<S>) -> Self
    where
        S: ToString,
    {
        Self {
            rng: thread_rng(),
            wordlist: Self::get_wordlist(placeholder),
            first_sentence: Self::get_first_sentence(placeholder),

            config: LoremConfig::default(),
        }
    }

    pub fn set_config(&mut self, config: LoremConfig) {
        self.config = config;
    }
}
