use tempfile::{TempDir, tempdir};

use crate::templates::{LapsePreset, TemplateEntry};

pub struct TestTemplate {
  tempdir: TempDir,
  inner: LapsePreset,
}

impl TestTemplate {
  pub fn load(&self) {
    self.inner.load(self.tempdir.path()).unwrap();
  }
}

impl From<LapsePreset> for TestTemplate {
  fn from(value: LapsePreset) -> Self {
    let dir = tempdir().unwrap();

    Self {
      tempdir: dir,
      inner: value,
    }
  }
}

#[test]
fn test_load_template() {
  use TemplateEntry::File;
  let template = LapsePreset::new(vec![
    File("scripts/script.lua".into(), "print('hey')".into()),
    File(
      "requests/request.md".into(),
      "GET https://httpbin.org/get".into(),
    ),
  ]);
  let temp = TestTemplate::from(template);
  temp.load();
  // TODO: validate further
}
