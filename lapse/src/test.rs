use tempfile::tempdir;

use crate::Lapse;

#[test]
fn test_init_space() {
  let temp_dir = tempdir().unwrap();
  Lapse::init(temp_dir.path()).unwrap();

  assert!(temp_dir.path().join("requests").join("request.md").exists());
  assert!(temp_dir.path().join("env").exists());
  assert!(temp_dir.path().join(".lapse").exists());
}
