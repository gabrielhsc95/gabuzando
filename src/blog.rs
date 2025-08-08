use crate::window::{WindowState, WindowWidget};
use leptos::prelude::*;

#[component]
pub fn BlogPage() -> impl IntoView {
    let post = WindowState::new(
        String::from("blog/post"),
        String::from("<p>The latest blog post will go here.</p>"),
        30,
        110,
        810,
        1110,
    );
    let history = WindowState::new(
        String::from("blog/history"),
        String::from("<p>list, order from recent to old, of blog posts.</p>"),
        870,
        110,
        300,
        450,
    );
    let best = WindowState::new(
        String::from("blog/best"),
        String::from("<p>list, ordered by most likes, of blog posts.</p>"),
        870,
        590,
        300,
        450,
    );
    let random = WindowState::new(
        String::from("blog/random"),
        String::from("<p>random blog post title.</p>"),
        870,
        1070,
        300,
        150,
    );
    view! {
        <WindowWidget state=post />
        <WindowWidget state=history />
        <WindowWidget state=best />
        <WindowWidget state=random />
    }
}
