use lapse::request::collection::{RequestCollection, RequestsCollectionEntry};

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

pub fn get_requests_flatlist(tree: Vec<RequestsCollectionEntry>) -> Vec<String> {
  let mut requests: Vec<String> = vec![];

  for entry in tree {
    match entry {
      RequestsCollectionEntry::Request(req) => {
        requests.push(req);
      }
      RequestsCollectionEntry::Collection(_, items) => {
        let sub_requests = get_requests_flatlist(*items);
        for sub in sub_requests {
          requests.push(sub);
        }
      }
    }
  }

  requests
}
