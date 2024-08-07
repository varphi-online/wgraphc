use crate::{
    parser::{
        evaluator::string_to_operator,
        structs::{operator::Operator, token::{self, Token}, value::Value},
    },
    //util::clog,
};
//use lazy_static::lazy_static;
//use fancy_regex::Regex;
use num_complex::{Complex64, ComplexFloat};
use serde_json::{self};
use std::{collections::HashMap, f64::consts::{FRAC_PI_2, PI}};
use wasm_bindgen::prelude::*;


// Interface a wasm/rust hashmap with j
#[wasm_bindgen]
pub fn set_var(key: String, value: String, map: String) -> String {
    let mut out: HashMap<String, String> = serde_json::from_str(&map).unwrap();
    out.insert(key, value);
    serde_json::to_string(&out).unwrap()
}

#[wasm_bindgen]
pub fn del_var(key: String, map: String) -> String {
    let mut out: HashMap<String, String> = serde_json::from_str(&map).unwrap();
    out.remove(&key);
    serde_json::to_string(&out).unwrap()
}

#[wasm_bindgen]
pub fn number_operator_from_2df64(real: f64,imag:f64)-> String{
    let mut out = Operator::from_token(Token::Num);
    out.values = Value::Number(Complex64::new(real, imag));
    format!{"{}~{}",serde_json::to_string(&out).unwrap(),out.values}
}

/*
lazy_static! {
    static ref function_id_eq: Regex =Regex::new(r"((([a-zA-Z]+)(_(\{(\w*(})?)?)?)?)\(+.*\)+)=.+").unwrap();
    static ref variable_id_eq: Regex = Regex::new(r"(([a-zA-Z]+)(_(\{(\w*(})?)?)?)?)=.+").unwrap();
}

#[wasm_bindgen]
pub fn parse_input(input: String, map: String) -> String {
    /*
    Output string is formatted in the form:

    "{0}~{1}~{2}~{3}~{4}"

    Where:
    {0} is one of 4 possible states:
        000 - A real number
        001 - An imaginary number
        010 - A complex number
        011 - Not a number
        100 - Any value as the product of an evaluation

    {1} is one of 4 possible states:
        00 - An expression
        01 - A variable
        10 - A function
        11 - A malformed input

    {2} is either an empty string or a serialized operator struct

    {3} is an identifier for a variable or
    
    {4} Is a tuple like representation of two f64 numbers representing
        the real and imaginary components.

    {5} Is a list of dependencies if evaluated as an expression
    */

    // Initialize Regexes

    // Output is stored in a vec of Strings and formatted at the end
    let mut out: [String; 6] = ["".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string()];

    let mut clean_input = String::new();
    if input.contains("=") {
        if function_id_eq.is_match(&input).unwrap() {
        out[1] = "10".to_string();
        let temp: Vec<String> = input.split("=").map(|s| s.to_string()).collect();
        if temp.len() >1 {
            clean_input = temp[1].to_string();
        }
        } else if variable_id_eq.is_match(&input).unwrap() {
            out[1] = "01".to_string();
            let temp: Vec<String> = input.split("=").map(|s| s.to_string()).collect();
            out[3] = temp[0].to_string();
            if temp.len() >1 {
                clean_input = temp[1].to_string();
            }
        } else {
            out[1] = "11".to_string();
        }
    } else {
        out[1] = "00".to_string();
        clean_input = input;
    }
    if let Some(mut parsed) = string_to_operator(clean_input) {
        // "Compiles" a constant expression to a single number, and adds
        // nessecary information
        if parsed.token_type != token::Token::Num {
            //out[1] = "00".to_string();
            out[5] = format!("{:?}",parsed.dependencies());
        }
        parsed = parsed.flatten(str_to_varmap(map));
        if parsed.is_constant() {
            let mut evaluated = Operator::from_token(Token::Num);
            evaluated.values = Value::Number(parsed.eval(Complex64::new(0.0, 0.0)));
            if let Value::Number(number) = evaluated.values {
                // Can't define expressions in a match statement, this is equal to 3pi/2
                const FRAC_PI_3_2: f64 = -FRAC_PI_2;
                out[0] = match number.to_polar().1 {
                    0.0 | PI => "000",
                    FRAC_PI_2 | FRAC_PI_3_2 => "001",
                    _ => "010",
                }
                .to_string();
                out[4] = format!("{},{}",number.re(), number.im());
            } else {
                out[0] = "011".to_string();
            }
            out[2] = serde_json::to_string(&evaluated).unwrap();
        } else {
            out[0] = "100".to_string();
            out[2] = serde_json::to_string(&parsed).unwrap();
        }
    }
    format!("{}~{}~{}~{}~{}~{}", out[0], out[1], out[2], out[3],out[4], out[5])
}
*/

#[wasm_bindgen]
pub fn faster_parse_input(input: String, map: String) -> String {
    /*
    Output string is formatted in the form:

    "{0}~{1}~{2}~{3}"

    Where:
    {0} is one of 4 possible states:
        000 - A real number
        001 - An imaginary number
        010 - A complex number
        011 - Not a number
        100 - Any value as the product of an evaluation

    {1} is either an empty string or a serialized operator struct

    {2} Is a tuple like representation of two f64 numbers representing
        the real and imaginary components.

    {3} Is a list of dependencies if evaluated as an expression
    */

    // Initialize Regexes

    // Output is stored in a vec of Strings and formatted at the end
    let mut out: [String; 4] = ["".to_string(),"".to_string(),"".to_string(),"".to_string()];
    if let Some(mut parsed) = string_to_operator(input) {
        // "Compiles" a constant expression to a single number, and adds
        // nessecary information
        if parsed.token_type != token::Token::Num {
            //out[1] = "00".to_string();
            out[3] = format!("{:?}",parsed.dependencies());
        }
        parsed = parsed.flatten(str_to_varmap(map));
        if parsed.is_constant() {
            let mut evaluated = Operator::from_token(Token::Num);
            evaluated.values = Value::Number(parsed.eval(Complex64::new(0.0, 0.0)));
            if let Value::Number(number) = evaluated.values {
                // Can't define expressions in a match statement, this is equal to 3pi/2
                const FRAC_PI_3_2: f64 = -FRAC_PI_2;
                out[0] = match number.to_polar().1 {
                    0.0 | PI => "000",
                    FRAC_PI_2 | FRAC_PI_3_2 => "001",
                    _ => "010",
                }
                .to_string();
                out[2] = format!("{},{}",number.re(), number.im());
            } else {
                out[0] = "011".to_string();
            }
            out[1] = serde_json::to_string(&evaluated).unwrap();
        } else {
            out[0] = "100".to_string();
            out[1] = serde_json::to_string(&parsed).unwrap();
        }
    }
    format!("{}~{}~{}~{}", out[0], out[1], out[2], out[3])
}

/*
// Older, possibly "safer" implimentation, we'll see

pub fn str_to_varmap(input: String) -> HashMap<String, Operator>{
let string_map: HashMap<String, String> =
    serde_json::from_str::<HashMap<String, String>>(&input).unwrap();

let out: HashMap<String,Result<Operator,serde_json::Error>> = string_map.into_iter().map(|(a,b)|-> (String,Result<Operator,serde_json::Error>){
    (a,serde_json::from_str::<Operator>(&b))
}).collect();

// Filtermap will only return variables that are validly defined
out.into_iter().filter_map(|(a,b)| -> Option<(String,Operator)>{
    if let Ok(op) = b {
        Some((a,op))
    } else {
        None
    }
}).collect()
}
*/

pub fn str_to_varmap(input: String) -> HashMap<String, Operator> {
    let string_map: HashMap<String, String> = serde_json::from_str(&input).expect("Failed to parse JSON");

    string_map
        .into_iter()
        .filter_map(|(key, value)| {
            serde_json::from_str::<Operator>(&value).ok().map(|op| (key, op))
        })
        .collect()
}