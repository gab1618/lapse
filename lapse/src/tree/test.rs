use std::fs;

use crate::{
  test::TempLapse,
  tree::{FlatTreeConfig, resource::Resource},
};
use lapse_template::templates::LapsePreset;

#[test]
fn test_flatten() {
  let lapse = TempLapse::new();
  let preset = LapsePreset::httpbin();
  preset.load(lapse.path()).unwrap();
  let tree = lapse.get_resource_tree(Resource::Requests, None).unwrap();
  let flat = tree.as_flat(Default::default());
  assert_eq!(flat.len(), 5);
}

#[test]
fn test_flatten_dirs_only() {
  let lapse = TempLapse::new();
  let preset = LapsePreset::httpbin();
  preset.load(lapse.path()).unwrap();
  let tree = lapse.get_resource_tree(Resource::Requests, None).unwrap();
  let flat_config = FlatTreeConfig::default()
    .include_files(false)
    .include_dirs(true);
  let flat = tree.as_flat(flat_config);
  assert_eq!(flat.len(), 0);

  fs::create_dir(lapse.requests_path().join("collection")).unwrap();
  fs::create_dir(lapse.requests_path().join("collection/sub")).unwrap();
  fs::create_dir(lapse.requests_path().join("collection/sub2")).unwrap();

  let tree = lapse.get_resource_tree(Resource::Requests, None).unwrap();
  let flat = tree.as_flat(flat_config);
  assert_eq!(flat.len(), 3);

  assert!(flat.contains(&String::from("collection")));
  assert!(flat.contains(&String::from("collection/sub")));
  assert!(flat.contains(&String::from("collection/sub2")));
}
