# Args Extractor

[![latest version](https://img.shields.io/crates/v/args-extractor.svg)](https://crates.io/crates/args-extractor)
[![build status](https://img.shields.io/github/actions/workflow/status/skewnart/args-extractor/ci.yml)](https://github.com/Skewnart/args-extractor/actions)
[![dependency status](https://deps.rs/repo/github/skewnart/args-extractor/status.svg)](https://deps.rs/repo/github/skewnart/args-extractor)
[![downloads](https://img.shields.io/crates/d/args-extractor.svg)](https://crates.io/crates/args-extractor)
[![docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/args-extractor/latest/args_extractor)
![license](https://img.shields.io/crates/l/args-extractor.svg)

Extracting arguments from a command line (prompt) is much easier than ever !

## Classic usage

For this prompt : `echo "hello world" | ./your_program arg1 arg2 -i -f file.txt`

PromptExtractor will extract :
- program_name : `"./your_program"`
- content_piped : `Some("hello world\n")`
- arguments : `Some(["arg1", "arg2"])`
- parameters: `Some({"-i": [], "-f": ["file.txt"]})`

``` rust
use args_extractor::{Prompt, PromptExtractor};

fn main() -> Result<(), String> {
    let prompt = PromptExtractor::extract()?;
    println!("{:?}", prompt);

    // # For your own Configuration building process
    // let config = Config::build(prompt)?;

    Ok(())
}
```

## Mock usage

``` rust
use args_extractor::{Prompt, PromptExtractor};
use std::vec::IntoIter;

fn string_into_iter(input: &str) -> IntoIter<String> {
    input.split_whitespace().map(String::from).collect::<Vec<String>>().into_iter()
}

fn main() -> Result<(), String> {
    let args = string_into_iter("program.exe arg1 arg2 -i -f file.txt");
    let prompt = PromptExtractor::extract_from(args)?;
    println!("{:?}", prompt);

    // # For your own Configuration building process
    // let config = Config::build(prompt)?;

    Ok(())
}
```
