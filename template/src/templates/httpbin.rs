use crate::templates::{LapsePreset, TemplateEntry};

impl LapsePreset {
  pub fn httpbin() -> Self {
    use TemplateEntry::File;

    Self::new(vec![
      File(
        "get.md".to_string(),
        include_str!("../../templates/httpbin/requests/get.md").to_string(),
      ),
      File(
        "post.md".to_string(),
        include_str!("../../templates/httpbin/requests/post.md").to_string(),
      ),
      File(
        "patch.md".to_string(),
        include_str!("../../templates/httpbin/requests/patch.md").to_string(),
      ),
      File(
        "delete.md".to_string(),
        include_str!("../../templates/httpbin/requests/delete.md").to_string(),
      ),
      File(
        "put.md".to_string(),
        include_str!("../../templates/httpbin/requests/put.md").to_string(),
      ),
    ])
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
