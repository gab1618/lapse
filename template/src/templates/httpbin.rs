use crate::templates::{LapsePreset, TemplateEntry};

impl LapsePreset {
  pub fn httpbin() -> Self {
    use TemplateEntry::File;

    let base = Self::default();

    let superset = Self::new(vec![
      File(
        "requests/get.md".to_string(),
        include_str!("../../templates/httpbin/requests/get.md").to_string(),
      ),
      File(
        "requests/post.md".to_string(),
        include_str!("../../templates/httpbin/requests/post.md").to_string(),
      ),
      File(
        "requests/patch.md".to_string(),
        include_str!("../../templates/httpbin/requests/patch.md").to_string(),
      ),
      File(
        "requests/delete.md".to_string(),
        include_str!("../../templates/httpbin/requests/delete.md").to_string(),
      ),
      File(
        "requests/put.md".to_string(),
        include_str!("../../templates/httpbin/requests/put.md").to_string(),
      ),
    ]);

    base + superset
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
