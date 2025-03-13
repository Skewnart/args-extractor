use std::io::{self, BufRead};
use crate::stdin::Terminal;

pub struct Parameter {
    pub name: String,
    pub arguments: Vec<String>
}

pub struct Prompt {
    pub program_name: String,
    pub content_piped: Option<String>,
    pub arguments: Option<Vec<String>>,
    pub parameters: Option<Vec<Parameter>>,
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
        let mut _parameters: Vec<Parameter> = vec![];

        let mut current_parameter: Option<Parameter> = None;

        for arg in args {
            if arg.starts_with("-") {
                if let Some(parameter) = current_parameter {
                    _parameters.push(parameter);
                }

                current_parameter = Some(Parameter {
                    name: arg,
                    arguments: Vec::<String>::new()
                });
            }
            else {
                match current_parameter {
                    None => { _arguments.push(arg); },
                    Some(ref mut parameter) => {parameter.arguments.push(arg);}
                }
            }
        }

        if let Some(parameter) = current_parameter {
            _parameters.push(parameter);
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
fn test() {
    
    use super::*;
    use std::vec::IntoIter;
    
    fn extract_query_into_iter(input: &str) -> IntoIter<String> {
        input.split_whitespace().map(String::from).collect::<Vec<String>>().into_iter()
    }

    let args= extract_query_into_iter("program.exe query file -i");
    let config = PromptExtractor::new(StdinServiceMock { is_terminal: true }).extract(args);

    assert!(config.is_ok());

    //TODO poursuivre les tests
}
