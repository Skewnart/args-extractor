use std::env;
use crate::{extractor::PromptExtractor, prompt::Prompt, stdin::{StdinService, StdinServiceMock}};

pub struct PromptService{}
impl PromptService {

    pub fn get() -> Result<Prompt, String> {
        let args = env::args();

        Ok(PromptExtractor::new(StdinService::new())
            .extract(args)?)
    }

    pub fn get_with_args(args : impl Iterator<Item = String>) -> Result<Prompt, String> {
        Ok(PromptExtractor::new(StdinServiceMock { is_terminal: true })
        .extract(args)?)
    }
}

#[cfg(test)]
use std::vec::IntoIter;

#[cfg(test)]
fn extract_query_into_iter(input: &str) -> IntoIter<String> {
    input.split_whitespace().map(String::from).collect::<Vec<String>>().into_iter()
}

#[test]
fn check_no_arguments() {
    
    use super::*;

    let args= extract_query_into_iter("");
    let prompt = PromptService::get_with_args(args);

    assert!(prompt.is_err());
}

#[test]
fn check_arguments() {
    
    use super::*;

    let args= extract_query_into_iter("program.exe arg1 arg2");
    let prompt = PromptService::get_with_args(args);

    assert!(prompt.is_ok());
    let prompt = prompt.expect("Cannot be None");

    assert_eq!(prompt.program_name, "program.exe");
    assert_eq!(prompt.arguments.is_some_and(|args| args.len() == 2), true);
    assert!(prompt.parameters.is_none());
}

#[test]
fn check_parameter() {
    
    use super::*;

    let args= extract_query_into_iter("program.exe -h");
    let prompt = PromptService::get_with_args(args);

    assert!(prompt.is_ok());
    let prompt = prompt.expect("Cannot be None");

    assert_eq!(prompt.program_name, "program.exe");
    assert!(prompt.arguments.is_none());
    assert_eq!(prompt.parameters.is_some_and(|params| params.contains_key("-h")), true);
}

#[test]
fn check_two_parameters() {
    
    use super::*;

    let args= extract_query_into_iter("program.exe -h --test");
    let prompt = PromptService::get_with_args(args);

    assert!(prompt.is_ok());
    let prompt = prompt.expect("Cannot be None");

    assert_eq!(prompt.program_name, "program.exe");
    assert!(prompt.arguments.is_none());
    assert_eq!(prompt.parameters.is_some_and(|params| params.len() == 2 && params.contains_key("--test")), true);
}

#[test]
fn check_argument_parameters() {
    
    use super::*;

    let args= extract_query_into_iter("program.exe test -h --test");
    let prompt = PromptService::get_with_args(args);

    assert!(prompt.is_ok());
    let prompt = prompt.expect("Cannot be None");

    assert_eq!(prompt.program_name, "program.exe");
    assert_eq!(prompt.arguments.is_some_and(|args| args.len() == 1), true);
    assert_eq!(prompt.parameters.is_some_and(|params| params.len() == 2 && params.contains_key("--test")), true);
}

#[test]
fn check_parameter_with_arguments() {
    
    use super::*;

    let args= extract_query_into_iter("program.exe -h --test 1");
    let prompt = PromptService::get_with_args(args);

    assert!(prompt.is_ok());
    let prompt = prompt.expect("Cannot be None");

    assert_eq!(prompt.program_name, "program.exe");
    assert!(prompt.arguments.is_none());
    assert_eq!(prompt.parameters.is_some_and(|params| params.len() == 2 && params.contains_key("--test") && params.get("--test").is_some_and(|param| param.len() == 1)), true);
}
