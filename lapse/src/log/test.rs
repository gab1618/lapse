use std::str::FromStr as _;

use crate::{log::ResponseLog, test::TempLapse};

#[test]
fn test_retrieve_log() {
  let lapse = TempLapse::new();
  let first_entry = ResponseLog {
    request: "testing".to_string(),
    text: "{}".to_string(),
    status: 200,
    headers: Default::default(),
  };

  let second_entry = ResponseLog {
    request: "testing".to_string(),
    text: "second".to_string(),
    status: 201,
    headers: Default::default(),
  };

  lapse.save_log(&first_entry).unwrap();

  let log_names = lapse.get_response_logs_names("testing").unwrap();

  let last_log_name = log_names.get(0).unwrap();
  let last_log = lapse
    .get_response_log_entry("testing", last_log_name)
    .unwrap();
  assert_eq!(ResponseLog::from_str(&last_log).unwrap(), first_entry);

  lapse.save_log(&second_entry).unwrap();

  let log_names = lapse.get_response_logs_names("testing").unwrap();

  let last_log_name = log_names.get(1).unwrap();
  let last_log = lapse
    .get_response_log_entry("testing", last_log_name)
    .unwrap();
  assert_eq!(ResponseLog::from_str(&last_log).unwrap(), first_entry);

  let second_last_log_name = log_names.get(0).unwrap();
  let second_last_log = lapse
    .get_response_log_entry("testing", second_last_log_name)
    .unwrap();
  assert_eq!(
    ResponseLog::from_str(&second_last_log).unwrap(),
    second_entry
  );
}
