use crate::test::TempLapse;
use std::fs;

#[test]
fn test_get_httponly_req() {
  let lapse = TempLapse::new();
  let example_req_content = include_str!("../../assets/without-markdown.md");
  let example_req_path = lapse.path().join("requests/without-markdown.md");
  fs::write(example_req_path, example_req_content).unwrap();

  let request_http = lapse.get_raw_request_http("without-markdown").unwrap();

  assert!(!request_http.is_empty());
  assert_eq!(request_http.trim(), example_req_content.trim());
}
