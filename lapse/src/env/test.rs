use tempfile::tempdir;

use crate::Lapse;

#[test]
fn test_switch_env() {
  let temp_dir = tempdir().unwrap();
  let lapse = Lapse::init(temp_dir.path()).unwrap();

  lapse.switch_env("prod");

  assert_eq!(lapse.current_env(), "prod");
}
