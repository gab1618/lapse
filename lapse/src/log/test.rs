use crate::{
  log::{ResponseLog, ResponseLogsIter},
  test::TempLapse,
};

#[test]
fn test_retrieve_log() {
  let lapse = TempLapse::new();

  let first_entry = ResponseLog {
    request: "testing".to_string(),
    text: "{}".to_string(),
    status: 200,
    ..Default::default()
  };

  let last_entry = ResponseLog {
    request: "testing".to_string(),
    text: "second".to_string(),
    status: 201,
    ..Default::default()
  };

  lapse.save_log(&first_entry).unwrap();
  lapse.save_log(&last_entry).unwrap();

  let mut entries = lapse.logs_iter("testing");
  let mut entries = ResponseLogsIter::new(&mut entries);

  assert_eq!(entries.next().unwrap(), last_entry);
  assert_eq!(entries.next().unwrap(), first_entry);
}
