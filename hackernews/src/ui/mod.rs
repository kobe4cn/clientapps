use dioxus::prelude::*;
use story_comment::Comments;
use story_item::Story;

use crate::{get_top_stories, StoryPageData};
mod story_comment;
mod story_item;

#[derive(Clone, Debug)]
pub enum CommentState {
    Unset,
    Loading,
    Loaded(Box<StoryPageData>),
}

pub fn app() -> Element {
    use_context_provider(|| Signal::new(CommentState::Unset));
    rsx! {
        main { class: "flex w-full h-full shadow-lg rounded-3xl",
        section { class: "flex flex-col w-4/12 h-full pt-3 overflow-y-scroll bg-gray-50",
          Stories {}
        }
        section { class: "flex flex-col w-8/12 px-4 bg-white rounded-r-3xl", Comments {} }
      }
    }
}

#[component]
fn Stories() -> Element {
    let stories = use_resource(move || get_top_stories(10));
    match &*stories.read_unchecked() {
        Some(Ok(stories)) => {
            rsx! {

                    for story in stories {
                        Story { item: story.clone() }

                 }

            }
        }
        Some(Err(e)) => {
            rsx! {
                div { class: "text-red-500",
                p { "Failed to load stories" }
                p {  "{e}"}
                }
            }
        }

        None => {
            rsx! {
                div { class: "text-gray", "Loading..." }
            }
        }
    }
}
