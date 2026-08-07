use crate::templates::{LapsePreset, TemplateEntry};

impl Default for LapsePreset {
  fn default() -> Self {
    let requests: Vec<(String, String)> = vec![(
      "get.md".to_string(),
      include_str!("../../templates/default/requests/sample.md").to_string(),
    )];
    Self {
      requests: requests
        .into_iter()
        .map(|(name, content)| TemplateEntry { name, content }.into())
        .collect::<Vec<_>>(),
      scripts: Default::default(),
    }
  }
}

#[cfg(test)]
mod tests {
  use lapse::Lapse;

  use crate::templates::{LapsePreset, test::TestTemplate};

  #[test]
  fn test_load_httpbin_preset() {
    let template = LapsePreset::default();
    let test = TestTemplate::from(template);
    Lapse::init(test.path()).unwrap();
    test.load();
  }
}
