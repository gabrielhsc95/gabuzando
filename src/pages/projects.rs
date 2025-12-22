use yew::prelude::*;
use crate::components::window::WindowProps;

pub fn get_projects_windows() -> Vec<WindowProps> {
    vec![
        // Cell 1: Crustacean Capital
        WindowProps {
            title: AttrValue::from("project/crustacean-capital"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <>
                    <b><a href="https://github.com/gabrielhsc95/crustacean-capital" target="_blank">{"crustacean-capital"}</a></b>
                    <p>{"A bond pricer in Rust."}</p>
                </>
            }]),
            x: 1.0,
            y: 10.0,
            width: 31.6,
            height: 27.3,
            buffer: 20.0,
        },
        // Cell 2: Gabuzando
        WindowProps {
            title: AttrValue::from("project/gabuzando"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <>
                    <b><a href="https://github.com/gabrielhsc95/gabuzando" target="_blank">{"gabuzando"}</a></b>
                    <p>{"This website! Built in Rust to showcase my experience and some of my thoughts."}</p>
                </>
            }]),
            x: 34.6,
            y: 10.0,
            width: 30.6,
            height: 27.3,
            buffer: 20.0,
        },
        // Cell 3: Cinema City
        WindowProps {
            title: AttrValue::from("project/cinema-city"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <>
                    <b><a href="https://github.com/gabrielhsc95/cinema-city" target="_blank">{"cinema-city"}</a></b>
                    <p>{"Scrapes the movies that are screening in English in Budapest, Hungary. So I could go to the movies."}</p>
                </>
            }]),
            x: 67.2,
            y: 10.0,
            width: 31.6,
            height: 27.3,
            buffer: 20.0,
        },
        // Cell 4: Placeholder
        WindowProps {
            title: AttrValue::from("project/placeholder_1"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <p>{"Future project goes here."}</p>
            }]),
            x: 1.0,
            y: 39.3,
            width: 31.6,
            height: 26.3,
            buffer: 20.0,
        },
        // Cell 5: Tic Tac Toe
        WindowProps {
            title: AttrValue::from("project/tictactoe"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <>
                    <b><a href="https://github.com/gabrielhsc95/tictactoe" target="_blank">{"tictactoe"}</a></b>
                    <p>{"More than just a Tic Tac Toe, it is binary game engine and Artificial Intelligence."}</p>
                </>
            }]),
            x: 34.6,
            y: 39.3,
            width: 30.6,
            height: 26.3,
            buffer: 20.0,
        },
        // Cell 6: Placeholder
        WindowProps {
            title: AttrValue::from("project/placeholder_2"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <p>{"Future project goes here."}</p>
            }]),
            x: 67.2,
            y: 39.3,
            width: 31.6,
            height: 26.3,
            buffer: 20.0,
        },
        // Cell 7: Mentor XMercury
        WindowProps {
            title: AttrValue::from("project/mentor_xmercury"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <>
                    <b><a href="https://github.com/Giovani-Costa/project_xmercury" target="_blank">{"project_xmercury"}</a></b>
                    <p>{"Manage a Tabletop RPG game using a discord bot and streamlit app."}</p>
                </>
            }]),
            x: 1.0,
            y: 67.6,
            width: 31.6,
            height: 27.3,
            buffer: 20.0,
        },
        // Cell 8: Mentor XLunar
        WindowProps {
            title: AttrValue::from("project/mentor_xlunar"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <>
                    <b><a href="https://github.com/Giovani-Costa/project_xlunar" target="_blank">{"project_xlunar"}</a></b>
                    <p>{"Discord bot to help student for standards exams."}</p>
                </>
            }]),
            x: 34.6,
            y: 67.6,
            width: 30.6,
            height: 27.3,
            buffer: 20.0,
        },
        // Cell 9: Placeholder
        WindowProps {
            title: AttrValue::from("project/placeholder_3"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <p>{"Future project goes here."}</p>
            }]),
            x: 67.2,
            y: 67.6,
            width: 31.6,
            height: 27.3,
            buffer: 20.0,
        },
    ]
}
