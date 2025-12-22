use yew::prelude::*;
use crate::window::WindowProps;

pub fn get_blog_windows() -> Vec<WindowProps> {
    vec![
        // Post (Cells 1, 2, 4, 5, 7, 8 - Left & Center Cols, Spanning All Rows)
        WindowProps {
            title: AttrValue::from("blog/post"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <p>{"The latest blog post will go here."}</p>
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
                <p>{"list, order from recent to old, of blog posts."}</p>
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
                <p>{"list, ordered by most likes, of blog posts."}</p>
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
                <p>{"random blog post title."}</p>
            }]),
            x: 67.2,
            y: 67.6,
            width: 31.6,
            height: 27.3,
            buffer: 20.0,
        },
    ]
}
