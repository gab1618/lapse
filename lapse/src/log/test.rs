use crate::{log::ResponseLog, runner::ExecutionResult, test::TempLapse};

#[test]
fn test_retrieve_log() {
  let lapse = TempLapse::new();

  let first_entry = ResponseLog {
    request: "testing".to_string(),
    result: ExecutionResult {
      status: 200,
      text: "{}".to_string(),
      timestamp: 1,
      ..Default::default()
    },
  };

  let last_entry = ResponseLog {
    request: "testing".to_string(),
    result: ExecutionResult {
      status: 201,
      text: "second".to_string(),
      timestamp: 2,
      ..Default::default()
    },
  };

  lapse.save_log(&first_entry).unwrap();
  lapse.save_log(&last_entry).unwrap();

  let mut entries = lapse.logs_iter().into_parsed();

  assert_eq!(entries.next().unwrap(), last_entry);
  assert_eq!(entries.next().unwrap(), first_entry);
}
