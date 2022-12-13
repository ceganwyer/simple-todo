use crate::todo::{todo, utils};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use requestty::question::{completions, Completions};
use requestty::{OnEsc, Question};

pub fn main_menu<'a>() -> Question<'a> {
    Question::expand("main_menu")
        .message("")
        .choices(vec![
            ('l', "List todo item"),
            ('a', "Add todo item"),
            ('d', "Mark item as done"),
            ('r', "Remove todo item"),
        ])
        .default_separator()
        .choice('x', "Exit")
        .on_esc(OnEsc::Terminate)
        .build()
}

pub fn title<'a>() -> Question<'a> {
    Question::input("title")
        .message("ToDo Item Title: ")
        .build()
}

pub fn id<'a>() -> Question<'a> {
    Question::input("id")
        .message("ID of ToDo Item to remove: ")
        .auto_complete(|i, _| auto_complete_ids(i))
        .validate(|i, _| match todo::exists(i) {
            true => Ok(()),
            false => Err(format!("Todo `{}` doesn't exist.", i)),
        })
        .build()
}

fn auto_complete_ids(i: String) -> Completions<String> {
    let mut ids: Completions<String> = utils::get_todo_ids()
        .unwrap_or(Vec::new())
        .iter()
        .map(|id| {
            let sid = id.simple().to_string();
            let id = sid.get(27..).unwrap();
            id.to_string()
        })
        .collect();

    if ids.is_empty() {
        return completions![i];
    }

    let fuzzer = SkimMatcherV2::default();
    ids.sort_by_cached_key(|id| fuzzer.fuzzy_match(id, &i).unwrap_or(i64::MAX));
    ids
}
