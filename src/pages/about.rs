use rand::prelude::*;
use yew::prelude::*;
use crate::components::window::WindowProps;
use crate::components::loading::Loading;
use crate::hooks::{use_fetch, FetchState};
use crate::types::{SimpleTextContent, CountriesContent, ContactContent, QuotesContent, Quote};

// Loaders
#[function_component(MeLoader)]
fn me_loader() -> Html {
    let fetch_state = use_fetch::<SimpleTextContent>("/text/about/me.json");

    match fetch_state {
        FetchState::Success(data) => {
            let div = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("div")
                .unwrap();
            div.set_inner_html(&data.text);
            html! { Html::VRef(div.into()) }
        }
        FetchState::Failed(err) => html! { <p style="color: red;">{ err }</p> },
        _ => html! { <Loading /> },
    }
}

#[function_component(CountriesLoader)]
fn countries_loader() -> Html {
    let fetch_state = use_fetch::<CountriesContent>("/text/about/countries.json");

    match fetch_state {
        FetchState::Success(data) => html! {
            <>
                <p>{&data.based}</p>
                <p>{&data.from}</p>
                <p>{&data.lived}</p>
                <p>{&data.visited}</p>
            </>
        },
        FetchState::Failed(err) => html! { <p style="color: red;">{ err }</p> },
        _ => html! { <Loading /> },
    }
}

#[function_component(ContactLoader)]
fn contact_loader() -> Html {
    let fetch_state = use_fetch::<ContactContent>("/text/about/contact.json");

    match fetch_state {
        FetchState::Success(data) => html! {
            <>
                { for data.links.iter().map(|link| html! {
                    <div class="same-line">
                        <img src={link.icon.clone()} alt={link.alt.clone()} class="contact"/>
                        <a href={link.url.clone()} target="_blank">{&link.text}</a>
                    </div>
                }) }
            </>
        },
        FetchState::Failed(err) => html! { <p style="color: red;">{ err }</p> },
        _ => html! { <Loading /> },
    }
}

#[function_component(WhyLoader)]
fn why_loader() -> Html {
    let fetch_state = use_fetch::<SimpleTextContent>("/text/about/why_gabuzando.json");

    match fetch_state {
        FetchState::Success(data) => html! { <p>{&data.text}</p> },
        FetchState::Failed(err) => html! { <p style="color: red;">{ err }</p> },
        _ => html! { <Loading /> },
    }
}

#[function_component(QuoteLoader)]
fn quote_loader() -> Html {
    let fetch_state = use_fetch::<QuotesContent>("/text/about/random_quote.json");
    let quote = use_state(|| None::<Quote>);

    {
        let quote = quote.clone();
        let fetch_state = fetch_state.clone();
        use_effect_with(fetch_state, move |state| {
            if let FetchState::Success(data) = state {
                // Only set if not already set, to avoid re-randomizing (optional, but good for stability)
                // Or if we want to ensure it's set when data arrives
                let mut rng = rand::thread_rng();
                if let Some(q) = data.quotes.choose(&mut rng) {
                    quote.set(Some(q.clone()));
                }
            }
            || ()
        });
    }
    
    match fetch_state {
        FetchState::Success(_) => {
            if let Some(quote) = &*quote {
                 html! {
                    <>
                        <p><q>{&quote.text}</q></p>
                        <p class="author">{&quote.author}</p>
                    </>
                }
            } else {
                 html! { <Loading /> }
            }
        },
        FetchState::Failed(err) => html! { <p style="color: red;">{ err }</p> },
        _ => html! { <Loading /> },
    }
}


pub fn get_about_windows() -> Vec<WindowProps> {
    vec![
        // Photo
        WindowProps {
            title: AttrValue::from("about/photo"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <img src="/images/photo.jpg" alt="Gabriel's Photo" style="width:100%; height:100%; object-fit:cover;" />
            }]),
            x: 1.0,
            y: 10.0,
            width: 31.6,
            height: 55.6,
            buffer: 20.0,
        },
        // Me
        WindowProps {
            title: AttrValue::from("about/me"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <MeLoader />
            }]),
            x: 34.6,
            y: 10.0,
            width: 64.2,
            height: 27.3,
            buffer: 20.0,
        },
        // Countries
        WindowProps {
            title: AttrValue::from("about/countries"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <CountriesLoader />
            }]),
            x: 34.6,
            y: 39.3,
            width: 30.6,
            height: 26.3,
            buffer: 20.0,
        },
        // Contact
        WindowProps {
            title: AttrValue::from("about/contact"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <ContactLoader />
            }]),
            x: 67.2,
            y: 39.3,
            width: 31.6,
            height: 26.3,
            buffer: 20.0,
        },
        // Why Gabuzando
        WindowProps {
            title: AttrValue::from("about/why_gabuzando"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <WhyLoader />
            }]),
            x: 1.0,
            y: 67.6,
            width: 64.2,
            height: 27.3,
            buffer: 20.0,
        },
        // Random Quote
        WindowProps {
            title: AttrValue::from("about/random_quote"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <QuoteLoader />
            }]),
            x: 67.2,
            y: 67.6,
            width: 31.6,
            height: 27.3,
            buffer: 20.0,
        },
    ]
}
