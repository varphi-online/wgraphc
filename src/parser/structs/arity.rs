use serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Arities {
    #[default]
    BINARY,
    UNARY,
    NULLARY,
}