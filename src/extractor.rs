use std::env;
use crate::{builder::PromptBuilder, prompt::Prompt, stdin::{StdinService, StdinServiceMock}};

pub struct PromptExtractor{}
impl PromptExtractor {

    pub fn extract() -> Result<Prompt, String> {
        let args = env::args();

        Ok(PromptBuilder::new(StdinService::new())
            .build(args)?)
    }

    pub fn extract_from(args : impl Iterator<Item = String>) -> Result<Prompt, String> {
        Ok(PromptBuilder::new(StdinServiceMock { is_terminal: true })
        .build(args)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::IntoIter;
    
    fn query_into_iter(input: &str) -> IntoIter<String> {
        input.split_whitespace().map(String::from).collect::<Vec<String>>().into_iter()
    }

    #[test]
    fn check_no_arguments() {

        let args= query_into_iter("");
        let prompt = PromptExtractor::extract_from(args);

        assert!(prompt.is_err());
    }

    #[test]
    fn check_arguments() {

        let args= query_into_iter("program.exe arg1 arg2");
        let prompt = PromptExtractor::extract_from(args);

        assert!(prompt.is_ok());
        let prompt = prompt.expect("Cannot be None");

        assert_eq!(prompt.program_name, "program.exe");
        assert_eq!(prompt.arguments.is_some_and(|args| args.len() == 2), true);
        assert!(prompt.parameters.is_none());
    }

    #[test]
    fn check_parameter() {

        let args= query_into_iter("program.exe -h");
        let prompt = PromptExtractor::extract_from(args);

        assert!(prompt.is_ok());
        let prompt = prompt.expect("Cannot be None");

        assert_eq!(prompt.program_name, "program.exe");
        assert!(prompt.arguments.is_none());
        assert_eq!(prompt.parameters.is_some_and(|params| params.contains_key("-h")), true);
    }

    #[test]
    fn check_two_parameters() {
        
        let args= query_into_iter("program.exe -h --test");
        let prompt = PromptExtractor::extract_from(args);

        assert!(prompt.is_ok());
        let prompt = prompt.expect("Cannot be None");

        assert_eq!(prompt.program_name, "program.exe");
        assert!(prompt.arguments.is_none());
        assert_eq!(prompt.parameters.is_some_and(|params| params.len() == 2 && params.contains_key("--test")), true);
    }

    #[test]
    fn check_argument_parameters() {

        let args= query_into_iter("program.exe test -h --test");
        let prompt = PromptExtractor::extract_from(args);

        assert!(prompt.is_ok());
        let prompt = prompt.expect("Cannot be None");

        assert_eq!(prompt.program_name, "program.exe");
        assert_eq!(prompt.arguments.is_some_and(|args| args.len() == 1), true);
        assert_eq!(prompt.parameters.is_some_and(|params| params.len() == 2 && params.contains_key("--test")), true);
    }

    #[test]
    fn check_parameter_with_arguments() {

        let args= query_into_iter("program.exe -h --test 1");
        let prompt = PromptExtractor::extract_from(args);

        assert!(prompt.is_ok());
        let prompt = prompt.expect("Cannot be None");

        assert_eq!(prompt.program_name, "program.exe");
        assert!(prompt.arguments.is_none());
        assert_eq!(prompt.parameters.is_some_and(|params| params.len() == 2 && params.get("--test").is_some_and(|param| param.len() == 1)), true);
    }

}