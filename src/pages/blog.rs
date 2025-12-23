use yew::prelude::*;
use crate::components::window::WindowProps;
use crate::components::loading::Loading;
use crate::hooks::{use_fetch, FetchState};
use crate::types::BlogList;
use rand::prelude::*;

#[derive(Clone, PartialEq)]
pub enum BlogMode {
    Post, // Latest for now
    History,
    Best,
    Random,
}

#[derive(Properties, PartialEq)]
pub struct BlogLoaderProps {
    pub mode: BlogMode,
}

#[function_component(BlogLoader)]
pub fn blog_loader(props: &BlogLoaderProps) -> Html {
    let fetch_state = use_fetch::<BlogList>("/text/blog/posts.json");
    let mode = props.mode.clone();

    match fetch_state {
        FetchState::Success(data) => {
             let mut posts = data.posts.clone();
             // Sort by date descending for most modes
             posts.sort_by(|a, b| b.date.cmp(&a.date));

             match mode {
                 BlogMode::Post => {
                     // Show latest post full content
                     if let Some(post) = posts.first() {
                         let div = web_sys::window()
                             .unwrap()
                             .document()
                             .unwrap()
                             .create_element("div")
                             .unwrap();
                         div.set_inner_html(&post.content);
                         let _ = div.set_attribute("style", "display: flex; flex-direction: column; gap: 1em; line-height: 1.6;");

                         html! {
                             <>
                                 <h2 style="margin-bottom: 0;">{&post.title}</h2>
                                 <p style="font-size: 0.8em; color: #666; margin-top: 0.2rem; margin-bottom: 2rem;">{&post.date}{" | Likes: "}{&post.likes}</p>
                                 { Html::VRef(div.into()) }
                             </>
                         }
                     } else {
                         html! { <p>{"No posts found."}</p> }
                     }
                 },
                 BlogMode::History => {
                     // List all (already sorted by date desc)
                     html! {
                         <ul>
                             { for posts.iter().map(|post| html! {
                                 <li>
                                     <a href={post.url.clone()} target="_blank">{&post.title}</a>
                                     <span style="font-size: 0.8em; color: #666;">{format!(" ({})", post.date)}</span>
                                 </li>
                             }) }
                         </ul>
                     }
                 },
                 BlogMode::Best => {
                     // Sort by likes desc
                     posts.sort_by(|a, b| b.likes.cmp(&a.likes));
                     html! {
                         <ul>
                             { for posts.iter().take(5).map(|post| html! {
                                 <li>
                                     <a href={post.url.clone()} target="_blank">{&post.title}</a>
                                     <span style="font-size: 0.8em; color: #666;">{format!(" ({} likes)", post.likes)}</span>
                                 </li>
                             }) }
                         </ul>
                     }
                 },
                 BlogMode::Random => {
                     let mut rng = rand::thread_rng();
                     if let Some(post) = posts.choose(&mut rng) {
                         html! {
                             <>
                                 <b>{"Random Pick:"}</b>
                                 <p><a href={post.url.clone()} target="_blank">{&post.title}</a></p>
                                 <p>{&post.summary}</p>
                             </>
                         }
                     } else {
                         html! { <p>{"No posts to pick from."}</p> }
                     }
                 }
             }
        },
        FetchState::Failed(err) => html! { <p style="color: red;">{ err }</p> },
        _ => html! { <Loading /> },
    }
}

pub fn get_blog_windows() -> Vec<WindowProps> {
    vec![
        // Post (Cells 1, 2, 4, 5, 7, 8 - Left & Center Cols, Spanning All Rows)
        WindowProps {
            title: AttrValue::from("blog/post"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <BlogLoader mode={BlogMode::Post} />
            }]),
            x: 1.0,
            y: 10.0,
            width: 64.2,
            height: 84.9,
            buffer: 20.0,
        },
        // History (Cell 3 - Top Right)
        WindowProps {
            title: AttrValue::from("blog/history"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <BlogLoader mode={BlogMode::History} />
            }]),
            x: 67.2,
            y: 10.0,
            width: 31.6,
            height: 27.3,
            buffer: 20.0,
        },
        // Best (Cell 6 - Middle Right)
        WindowProps {
            title: AttrValue::from("blog/best"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <BlogLoader mode={BlogMode::Best} />
            }]),
            x: 67.2,
            y: 39.3,
            width: 31.6,
            height: 26.3,
            buffer: 20.0,
        },
        // Random (Cell 9 - Bottom Right)
        WindowProps {
            title: AttrValue::from("blog/random"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <BlogLoader mode={BlogMode::Random} />
            }]),
            x: 67.2,
            y: 67.6,
            width: 31.6,
            height: 27.3,
            buffer: 20.0,
        },
    ]
}
