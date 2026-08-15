use crate::{test::TempLapse, tree::resource::Resource};
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
