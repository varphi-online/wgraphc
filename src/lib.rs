use once_cell::sync::Lazy;
use std::sync::RwLock;
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

static MAIN_FUNC: Lazy<RwLock<Option<parser::token::Operator>>> = Lazy::new(|| RwLock::new(None));

#[wasm_bindgen]
pub fn return_string(input: String) -> String {
    let lexemes = parser::scanner::scan(input);
    util::clog!("Lexemes: {:?}", lexemes);
    let tokens = parser::evaluator::evaluate(lexemes);
    let tokens2 = parser::evaluator::analyze(tokens);

    util::clog!("Analyzed: {}", tokens2);
    let mut abstract_tree = parser::ast::AST::default();
    let AST = abstract_tree.from_shunting_yard(tokens2.clone());
    util::clog!("AST: {}", abstract_tree.operands);

    update_main_func(AST.clone());
    format!("{}", tokens2)
}

fn update_main_func(op: Option<parser::token::Operator>) {
    if let Ok(mut guard) = MAIN_FUNC.write() {
        *guard = op;
        std::mem::drop(guard)
    }
}

pub fn get_main_func() -> Option<Option<parser::token::Operator>> {
    if let Ok(guard) = MAIN_FUNC.read() {
        Some(guard.clone())
    } else {
        None
    }
}
