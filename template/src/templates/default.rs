use crate::templates::{LapsePreset, TemplateEntry};

impl Default for LapsePreset {
  fn default() -> Self {
    use TemplateEntry::File;

    Self::new(vec![File(
      "requests/get.md".to_string(),
      include_str!("../../templates/default/requests/sample.md").to_string(),
    )])
  }
}

#[cfg(test)]
mod tests {
  use crate::templates::{LapsePreset, test::TestTemplate};

  #[test]
  fn test_load_httpbin_preset() {
    let template = LapsePreset::default();
    let test = TestTemplate::from(template);
    test.load();
  }
}
