use wasm_bindgen::prelude::*;

pub mod util {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);
    }
    #[cfg(debug_assertions)]
    macro_rules! clog {

    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (crate::util::log(&format_args!($($t)*).to_string()))
}
    #[cfg(not(debug_assertions))]
    macro_rules! clog {
        ($($t:tt)*) => {};
    }

    pub(crate) use clog;
}
mod graph;
mod parser;

#[wasm_bindgen]
pub fn parse_text(input: String) -> String {
    let lexemes = parser::scanner::scan(input);
    util::clog!("Lexemes: {:?}", lexemes);
    let tokens = parser::evaluator::evaluate(lexemes);
    let tokens2 = parser::evaluator::analyze(tokens);

    util::clog!("Analyzed: {}", tokens2);
    let mut abstract_tree = parser::ast::AST::default();
    let AST = abstract_tree.from_shunting_yard(tokens2.clone());
    util::clog!("AST: {}", abstract_tree.operands);

    serde_json::to_string(&AST).unwrap()
}
