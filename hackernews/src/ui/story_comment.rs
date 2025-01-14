use crate::{Comment, CommentState};
use dioxus::prelude::*;

#[component]
pub fn StoryComment(comment: Comment) -> Element {
    rsx! {
        li {
            article {  class: "mt-8 text-gray-500 leading-7 tracking-wider",
                span {  "{comment.by} {comment.time} | next [-] "  }
                div {  dangerous_inner_html:comment.text }
            }
        }
    }
}

#[component]
pub fn Comments() -> Element {
    let comment_state = use_context::<Signal<CommentState>>();
    match comment_state() {
        CommentState::Unset => rsx! {
            div {

                p { "select a story to view comments" }
            }
        },
        CommentState::Loading => rsx! {
            div {
                class: "mt-6",
                p { "Loading comments" }
            }
        },
        CommentState::Loaded(data) => rsx! {
            ul {
                for comment in data.comments {
                    StoryComment {
                        comment
                    }
                }
            }
        },
    }
}
