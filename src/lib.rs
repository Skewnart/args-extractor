#![doc = include_str!("../README.md")]

mod prompt;
mod builder;
mod extractor;
mod stdin;

pub use extractor::PromptExtractor;
pub use prompt::{Prompt, Parameters};
