use serde::Deserialize;
use envy::Error;


pub struct Settings {
  pub auth0_settings: Auth0Settings
}

#[derive(Deserialize, Clone, Debug)]
pub struct Auth0Settings {
  pub rsa_n: String,
  pub rsa_e: String,
  pub audience: String,
  pub domain: String,
}

pub fn get_configuration() -> Result<Auth0Settings, Error> {
  let config = match envy::from_env::<Auth0Settings>() {
    Ok(config) => config
    ,
    Err(error) => {
      return Err(error);
    }
 };
 Ok(config)
}