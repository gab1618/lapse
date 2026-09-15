use lapse::{
  Lapse,
  tree::{TraverseEntryKind, Tree, resource::Resource},
};

use colored::{Color, Colorize as _};

use crate::command::{log::display::method_color, open_lapse};

fn output_requests_tree(lapse: &Lapse, root: &Tree) {
  root.traverse(String::default(), 0, &|entry| {
    let depth_spacing = " ".repeat(entry.depth);

    match entry.kind {
      TraverseEntryKind::Entry => {
        if let Ok(head) = lapse.get_request_head(&entry.name) {
          println!(
            "{}{} {} {}",
            depth_spacing,
            entry.name,
            head.method.color(method_color(&head.method)),
            head.url.color(Color::Black)
          );
        } else {
          println!("{}{}", depth_spacing, entry.name,)
        }
      }
      TraverseEntryKind::Subtree => {
        println!("{}{}", depth_spacing, entry.name);
      }
    }

    crate::Result::Ok(())
  });
}

pub fn ls(path: Option<String>) -> crate::Result<()> {
  let lapse = open_lapse()?;
  let collection = lapse.get_resource_tree(Resource::Requests, path)?;

  output_requests_tree(&lapse, &collection);

  Ok(())
}
