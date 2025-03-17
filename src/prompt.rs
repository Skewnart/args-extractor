use std::collections::HashMap;

pub type Parameters = HashMap<String, Vec<String>>;

pub struct Prompt {
    pub program_name: String,
    pub content_piped: Option<String>,
    pub arguments: Option<Vec<String>>,
    pub parameters: Option<Parameters>,
}