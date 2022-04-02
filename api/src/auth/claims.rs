use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    _permissions: Option<HashSet<String>>,
}
