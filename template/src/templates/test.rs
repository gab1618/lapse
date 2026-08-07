use std::path::Path;

use lapse::{
  Lapse,
  tree::{TreeEntry, resource::Resource},
};
use tempfile::{TempDir, tempdir};

use crate::templates::{LapsePreset, TemplateEntry};

pub struct TestTemplate {
  tempdir: TempDir,
  inner: LapsePreset,
}

impl TestTemplate {
  pub fn load(&self) {
    self.inner.load(self.tempdir.path().into());
  }
  pub fn path(&self) -> &Path {
    self.tempdir.path()
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
  let template = LapsePreset {
    scripts: vec![
      TemplateEntry {
        name: "script.lua".to_string(),
        content: "print('hey')".to_string(),
      }
      .into(),
    ],
    requests: vec![
      TemplateEntry {
        name: "request.md".to_string(),
        content: "GET https://httpbin.org/get".to_string(),
      }
      .into(),
    ],
  };
  let temp = TestTemplate::from(template);
  let lapse = Lapse::init(temp.path()).unwrap();
  temp.load();

  let scripts_tree = lapse.get_resource_tree(Resource::Scripts, None).unwrap();
  let found_script = scripts_tree.get(0).unwrap();
  match found_script {
    TreeEntry::Subtree(_, _tree) => panic!("This was supposed to be a file"),
    TreeEntry::Entry(entry) => assert_eq!(entry, "script"),
  }

  let requests_tree = lapse.get_resource_tree(Resource::Requests, None).unwrap();
  let found_request = requests_tree.get(0).unwrap();
  match found_request {
    TreeEntry::Subtree(_, _tree) => panic!("This was supposed to be a file"),
    TreeEntry::Entry(entry) => assert_eq!(entry, "request"),
  }
}
