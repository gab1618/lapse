use std::str::FromStr as _;

use crate::{log::ResponseLog, test::TempLapse};

#[test]
fn test_retrieve_log() {
  let lapse = TempLapse::new();
  let first_entry = ResponseLog {
    request: "testing".to_string(),
    text: "{}".to_string(),
    status: 200,
    ..Default::default()
  };

  let second_entry = ResponseLog {
    request: "testing".to_string(),
    text: "second".to_string(),
    status: 201,
    ..Default::default()
  };

  lapse.save_log(&first_entry).unwrap();
  lapse.save_log(&second_entry).unwrap();

  let mut entries = lapse.logs_iter("testing");

  let last_log = entries.next().unwrap();
  let first_log = entries.next().unwrap();

  assert_eq!(ResponseLog::from_str(&last_log).unwrap(), second_entry);

  assert_eq!(ResponseLog::from_str(&first_log).unwrap(), first_entry);
}
