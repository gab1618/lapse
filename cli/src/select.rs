use inquire::Select;
use lapse::tree::Tree;
use nucleo_matcher::{
  Config, Matcher,
  pattern::{CaseMatching, Normalization, Pattern},
};

use crate::collection::get_tree_flatlist;

pub fn select_tree_entry(tree: &Tree, query: Option<String>) -> crate::Result<String> {
  let flat_list = get_tree_flatlist(tree);

  // Uses fuzzy finder to match some entry by name. If there are multiple matches, pick one
  // using a select
  let match_options = match &query {
    Some(name) => {
      let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
      let pattern = Pattern::parse(name, CaseMatching::Ignore, Normalization::Smart);
      let matches = pattern.match_list(&flat_list, &mut matcher);
      matches
        .into_iter()
        .map(|entry| entry.0.to_string())
        .collect::<Vec<String>>()
    }
    None => flat_list,
  };

  let query_string = query.unwrap_or_default();

  let selected_entry = if match_options.len() > 1 {
    let select = Select::new("Select", match_options).with_starting_filter_input(&query_string);
    select.prompt().map_err(crate::Error::InvokePrompt)?
  } else {
    match_options
      .into_iter()
      .next()
      .ok_or(crate::Error::NoResourceMatch(query_string))?
  };
  Ok(selected_entry)
}
