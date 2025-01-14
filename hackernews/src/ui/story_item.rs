use dioxus::prelude::*;

use crate::{get_story_comments, CommentState, StoryItem};

#[component]
pub fn Story(item: StoryItem) -> Element {
    let mut comment_state = use_context::<Signal<CommentState>>();
    //cache of the already loaded comments
    // let full_story = use_signal(|| None);
    rsx! {
        li { class: "py-5 border-b px-3 transition hover:bg-indigo-100",
        a { href: "#", class: "flex justify-between items-center",
            h3 { class: "text-lg font-semibold", "{item.title}" }
            p { class: "text-md text-gray-400"}
        }
        div { class: "text-md italic text-gray-400",
            span{"{item.score} points by {item.by} {item.time} "}
             a { href: "#",

             onclick: move |e| {
                 e.prevent_default();
                 let item=item.clone();
                 async move {
                    *comment_state.write() = CommentState::Loading;
                    if let Ok(story_page_data) = get_story_comments(item).await {
                        *comment_state.write() = CommentState::Loaded(Box::new(story_page_data));
                    }
                 }
                //  load_comments(comment_state, item.clone(), full_story);
             },
              "{item.kids.len()} comments" }
        }
    }
    }
}
// #[allow(unused)]
// async fn load_comments(
//     mut comment_state: Signal<CommentState>,
//     item: StoryItem,
//     mut full_story: Signal<Option<StoryPageData>>,
// ) {
//     if let Some(story_data) = full_story.as_ref() {
//         *comment_state.write() = CommentState::Loaded(story_data.clone());
//     }
//     *comment_state.write() = CommentState::Loading;
//     if let Ok(story_page_data) = get_story_comments(item).await {
//         *full_story.write() = Some(story_page_data.clone());
//         *comment_state.write() = CommentState::Loaded(story_page_data);
//     }
// }
