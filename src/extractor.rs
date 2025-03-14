use std::{collections::HashMap, io::{self, BufRead}};
use crate::stdin::Terminal;

type Parameters = HashMap<String, Vec<String>>;

pub struct Prompt {
    pub program_name: String,
    pub content_piped: Option<String>,
    pub arguments: Option<Vec<String>>,
    pub parameters: Option<Parameters>,
}

pub struct PromptExtractor<T> where T: Terminal {
    terminal_service : T
}
impl<T> PromptExtractor<T>  where T: Terminal {
    pub fn new (_terminal_service : T) -> Self {
        Self {
            terminal_service : _terminal_service
        }
    }

    pub fn extract(self, mut args : impl Iterator<Item = String>) -> Result<Prompt, String> {

        let Some(_program_name) = args.next() else {
            return Err("Program name was not automatically provided.".to_string());
        };

        let _from_pipe = !self.terminal_service.is_terminal();

        let _content_piped = if _from_pipe {
            Some(io::stdin().lock().lines().fold(String::from(""), |acc, line| acc + &line.unwrap() + "\n"))
        } else {
            None
        };

        let mut _arguments: Vec<String> = vec![];
        let mut _parameters: Parameters = HashMap::new();

        let mut current_parameter: Option<(String, Vec<String>)> = None;

        for arg in args {
            if arg.starts_with("-") {
                if let Some(parameter) = current_parameter {
                    _parameters.insert(parameter.0, parameter.1);
                }

                current_parameter = Some((arg, Vec::<String>::new()));
            }
            else {
                match current_parameter {
                    None => { _arguments.push(arg); },
                    Some(ref mut parameter) => {parameter.1.push(arg);}
                }
            }
        }

        if let Some(parameter) = current_parameter {
            _parameters.insert(parameter.0, parameter.1);
        }

        Ok(Prompt{
            program_name: _program_name,
            content_piped : _content_piped,
            arguments : if !_arguments.is_empty() { Some(_arguments) } else { None },
            parameters : if !_parameters.is_empty() { Some(_parameters) } else { None }
        })
    }
}

#[test]
fn check_arguments() {
    
    use super::*;
    use std::vec::IntoIter;
    
    fn extract_query_into_iter(input: &str) -> IntoIter<String> {
        input.split_whitespace().map(String::from).collect::<Vec<String>>().into_iter()
    }

    let args= extract_query_into_iter("program.exe arg1 arg2");
    let config = PromptExtractor::new(StdinServiceMock { is_terminal: true }).extract(args);

    assert!(config.is_ok());
    let config = config.expect("Cannot be None");

    assert_eq!(config.program_name, "program.exe");
    assert_eq!(config.arguments.is_some_and(|args| args.len() == 2), true);
    assert!(config.parameters.is_none());
}

#[test]
fn check_parameter() {
    
    use super::*;
    use std::vec::IntoIter;
    
    fn extract_query_into_iter(input: &str) -> IntoIter<String> {
        input.split_whitespace().map(String::from).collect::<Vec<String>>().into_iter()
    }

    let args= extract_query_into_iter("program.exe -h");
    let config = PromptExtractor::new(StdinServiceMock { is_terminal: true }).extract(args);

    assert!(config.is_ok());
    let config = config.expect("Cannot be None");

    assert_eq!(config.program_name, "program.exe");
    assert!(config.arguments.is_none());
    assert_eq!(config.parameters.is_some_and(|params| params.contains_key("-h")), true);
}

#[test]
fn check_two_parameters() {
    
    use super::*;
    use std::vec::IntoIter;
    
    fn extract_query_into_iter(input: &str) -> IntoIter<String> {
        input.split_whitespace().map(String::from).collect::<Vec<String>>().into_iter()
    }

    let args= extract_query_into_iter("program.exe -h --test");
    let config = PromptExtractor::new(StdinServiceMock { is_terminal: true }).extract(args);

    assert!(config.is_ok());
    let config = config.expect("Cannot be None");

    assert_eq!(config.program_name, "program.exe");
    assert!(config.arguments.is_none());
    assert_eq!(config.parameters.is_some_and(|params| params.len() == 2 && params.contains_key("--test")), true);
}

#[test]
fn check_argument_parameters() {
    
    use super::*;
    use std::vec::IntoIter;
    
    fn extract_query_into_iter(input: &str) -> IntoIter<String> {
        input.split_whitespace().map(String::from).collect::<Vec<String>>().into_iter()
    }

    let args= extract_query_into_iter("program.exe test -h --test");
    let config = PromptExtractor::new(StdinServiceMock { is_terminal: true }).extract(args);

    assert!(config.is_ok());
    let config = config.expect("Cannot be None");

    assert_eq!(config.program_name, "program.exe");
    assert_eq!(config.arguments.is_some_and(|args| args.len() == 1), true);
    assert_eq!(config.parameters.is_some_and(|params| params.len() == 2 && params.contains_key("--test")), true);
}

#[test]
fn check_parameter_with_arguments() {
    
    use super::*;
    use std::vec::IntoIter;
    
    fn extract_query_into_iter(input: &str) -> IntoIter<String> {
        input.split_whitespace().map(String::from).collect::<Vec<String>>().into_iter()
    }

    let args= extract_query_into_iter("program.exe -h --test 1");
    let config = PromptExtractor::new(StdinServiceMock { is_terminal: true }).extract(args);

    assert!(config.is_ok());
    let config = config.expect("Cannot be None");

    assert_eq!(config.program_name, "program.exe");
    assert!(config.arguments.is_none());
    assert_eq!(config.parameters.is_some_and(|params| params.len() == 2 && params.contains_key("--test") && params.get("--test").is_some_and(|param| param.len() == 1)), true);
}
