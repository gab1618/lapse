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

  let last_log = lapse.get_response_log("testing", 0).unwrap();
  assert_eq!(last_log, first_entry);

  lapse.save_log(&second_entry).unwrap();

  let last_log = lapse.get_response_log("testing", 1).unwrap();
  assert_eq!(last_log, first_entry);

  let second_last_log = lapse.get_response_log("testing", 0).unwrap();
  assert_eq!(second_last_log, second_entry);
}
