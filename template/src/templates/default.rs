use crate::templates::{LapsePreset, TemplateEntry};

impl Default for LapsePreset {
  fn default() -> Self {
    use TemplateEntry::{Dir, File};

    Self::new(vec![
      File(
        "requests/get.md".to_string(),
        include_str!("../../templates/default/requests/sample.md").to_string(),
      ),
      File(
        "env/variables.json".to_string(),
        include_str!("../../templates/default/env/variables.json").to_string(),
      ),
      File(
        "env/secrets.json".to_string(),
        include_str!("../../templates/default/env/secrets.json").to_string(),
      ),
      File(
        "env/hooks.json".to_string(),
        include_str!("../../templates/default/env/hooks.json").to_string(),
      ),
      File(
        ".gitignore".to_string(),
        include_str!("../../templates/default/gitignore").to_string(),
      ),
      File(
        ".luarc.json".to_string(),
        include_str!("../../templates/default/.luarc.json").to_string(),
      ),
      Dir(".lapse/typings".to_string()),
      File(
        ".lapse/typings/api.lua".to_string(),
        include_str!("../../templates/default/.lapse/typings/api.lua").to_string(),
      ),
      File(
        ".lapse/typings/globals.lua".to_string(),
        include_str!("../../templates/default/.lapse/typings/globals.lua").to_string(),
      ),
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
