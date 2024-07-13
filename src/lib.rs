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
mod parser;

#[wasm_bindgen]
pub fn return_string(input: String) -> Vec<String> {
    util::clog!("lol");
    let lexemes = parser::scanner::scan(input);
    util::clog!("{:?}", lexemes);
    let tokens = parser::evaluator::evaluate(lexemes);
    let mut tok_out: String = String::new();
    for token in tokens {
        tok_out += &token.to_string();
        tok_out += ",";
    }
    util::clog!("{}", tok_out);
    let out: Vec<String> = vec![tok_out];
    out
}
