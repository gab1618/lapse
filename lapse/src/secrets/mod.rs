pub mod error;

use std::{collections::HashMap, fs::OpenOptions};

use crate::{Lapse, env::EnvVariable, secrets::error::SecretsError};

pub type Secrets = HashMap<String, EnvVariable>;

impl Lapse {
  pub fn load_secrets(&self) -> crate::Result<Secrets> {
    let f = OpenOptions::new()
      .read(true)
      .open(self.secrets_path())
      .map_err(SecretsError::OpenSecretsFile)?;

    let parsed: Secrets = serde_json::from_reader(f).map_err(|_| SecretsError::ParseSecrets)?;

    Ok(parsed)
  }
}
