use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String,
    pub exp: i64,
    iss: String,
}
