use lapse::request::{RequestCollection, RequestsCollectionEntry};

pub fn output_requests_collection(level: usize, root: &RequestCollection) {
  let level_spacing = " ".repeat(level);

  for entry in root {
    match entry {
      RequestsCollectionEntry::Request(name) => {
        println!("{}{}", level_spacing, name);
      }
      RequestsCollectionEntry::Collection(name, items) => {
        println!("{}{}", level_spacing, name);

        output_requests_collection(level + 1, items);
      }
    }
  }
}
