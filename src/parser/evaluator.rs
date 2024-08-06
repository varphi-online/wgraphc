use structs::{abstract_syntax_tree, operator::Operator};

use super::*;

pub fn string_to_ast_str(input: String)-> String{
    let lexemes = lexer::scan(input);
    let tokens = tokenizer::tokenize(lexemes);
    let mut abstract_tree = abstract_syntax_tree::AbstractSyntaxTree::default();
    serde_json::to_string(&abstract_tree.from_shunting_yard(tokens)).unwrap()
}

pub fn string_to_operator(input: String)-> Option<Operator> {
    let lexemes = lexer::scan(input);
    let tokens = tokenizer::tokenize(lexemes);
    let mut abstract_tree = abstract_syntax_tree::AbstractSyntaxTree::default();
    abstract_tree.from_shunting_yard(tokens)
}