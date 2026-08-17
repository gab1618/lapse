use crate::templates::{LapsePreset, TemplateEntry};

impl Default for LapsePreset {
  fn default() -> Self {
    use TemplateEntry::{Dir, File};

    Self::new(vec![
      File(
        "requests/get.md".to_string(),
        include_str!("../../templates/default/requests/sample.md").to_string(),
      ),
      Dir("env/default".to_string()),
      File(
        "env/default/variables.json".to_string(),
        include_str!("../../templates/default/env/default/variables.json").to_string(),
      ),
      File(
        "env/default/secrets.json".to_string(),
        include_str!("../../templates/default/env/default/secrets.json").to_string(),
      ),
      File(
        "env/default/hooks.json".to_string(),
        include_str!("../../templates/default/env/default/hooks.json").to_string(),
      ),
      File(
        ".gitignore".to_string(),
        include_str!("../../templates/default/gitignore").to_string(),
      ),
      File(".lapse/.gitkeep".to_string(), Default::default()),
    ])
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
