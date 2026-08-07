use crate::templates::{LapseTemplate, TemplateEntry};

impl LapseTemplate {
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
        .collect::<Vec<_>>(),
      scripts: vec![
        TemplateEntry {
          name: "test.lua".to_string(),
          content: include_str!("../../templates/httpbin/scripts/test.lua").to_string(),
        }
        .into(),
      ],
    }
  }
}

#[cfg(test)]
mod tests {
  use lapse::Lapse;

  use crate::templates::{LapseTemplate, test::TestTemplate};

  #[test]
  fn test_load_httpbin_preset() {
    let template = LapseTemplate::httpbin();
    let test = TestTemplate::from(template);
    Lapse::init(test.path()).unwrap();
    test.load();
  }
}
