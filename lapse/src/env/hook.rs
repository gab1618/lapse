#[cfg_attr(test, derive(Debug))]
#[derive(serde::Deserialize, serde::Serialize, Hash, PartialEq, Eq)]
pub enum Event {
  #[serde(rename = "pre-request")]
  PreRequest,
  #[serde(rename = "post-request")]
  PostRequest,
}

#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct HookEntry {
  pub enabled: bool,
  pub scripts: Vec<String>,
}
