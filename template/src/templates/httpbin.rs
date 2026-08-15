use crate::templates::{LapsePreset, TemplateEntry};

impl LapsePreset {
  pub fn httpbin() -> Self {
    let requests: Vec<(String, String)> = vec![
      (
        "get.md".to_string(),
        include_str!("../../templates/httpbin/requests/get.md").to_string(),
      ),
      (
        "post.md".to_string(),
        include_str!("../../templates/httpbin/requests/post.md").to_string(),
      ),
      (
        "patch.md".to_string(),
        include_str!("../../templates/httpbin/requests/patch.md").to_string(),
      ),
      (
        "delete.md".to_string(),
        include_str!("../../templates/httpbin/requests/delete.md").to_string(),
      ),
      (
        "put.md".to_string(),
        include_str!("../../templates/httpbin/requests/put.md").to_string(),
      ),
    ];
    Self {
      requests: requests
        .into_iter()
        .map(|(name, content)| TemplateEntry { name, content }.into())
        .collect::<Vec<_>>()
        .into(),
      scripts: Default::default(),
      envs: Default::default(),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::templates::{LapsePreset, test::TestTemplate};

  #[test]
  fn test_load_httpbin_preset() {
    let template = LapsePreset::httpbin();
    let test = TestTemplate::from(template);
    test.load();
  }
}
