#[derive(Debug, Clone)]
pub struct KeywordNode {
    name: String,
    namespace: Option<String>,
    expanding: bool,
}

