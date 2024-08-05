use structs::abstract_syntax_tree;

use super::*;

pub fn string_to_ast(input: String)-> String{
    let lexemes = lexer::scan(input);
    let tokens = tokenizer::tokenize(lexemes);
    let mut abstract_tree = abstract_syntax_tree::AbstractSyntaxTree::default();
    serde_json::to_string(&abstract_tree.from_shunting_yard(tokens)).unwrap()
}