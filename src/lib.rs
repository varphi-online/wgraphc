use wasm_bindgen::prelude::*;
mod parser;
mod graph;

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

#[wasm_bindgen]
pub fn debug() -> bool {
    cfg!(debug_assertions)
}

#[wasm_bindgen]
pub fn parse_text(input: String) -> String {
    parser::evaluator::string_to_ast(input)
}
