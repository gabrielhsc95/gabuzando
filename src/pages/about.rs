use gloo_net::http::Request;
use rand::prelude::*;
use serde::Deserialize;
use yew::prelude::*;
use crate::components::window::WindowProps;

// Structures


#[derive(Clone, PartialEq, Deserialize)]
struct CountriesContent {
    based: String,
    from: String,
    lived: String,
    visited: String,
}

#[derive(Clone, PartialEq, Deserialize)]
struct ContactLink {
    icon: String,
    alt: String,
    text: String,
    url: String,
}

#[derive(Clone, PartialEq, Deserialize)]
struct ContactContent {
    links: Vec<ContactLink>,
}

#[derive(Clone, PartialEq, Deserialize)]
struct SimpleTextContent {
    text: String,
}

#[derive(Clone, PartialEq, Deserialize)]
struct Quote {
    text: String,
    author: String,
}

#[derive(Clone, PartialEq, Deserialize)]
struct QuotesContent {
    quotes: Vec<Quote>,
}

// Loaders
#[function_component(MeLoader)]
fn me_loader() -> Html {
    let content = use_state(|| None::<SimpleTextContent>);
    {
        let content = content.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = Request::get("/text/about/me.json").send().await {
                    if let Ok(json) = resp.json::<SimpleTextContent>().await {
                        content.set(Some(json));
                    }
                }
            });
            || ()
        });
    }

    if let Some(data) = &*content {
        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        div.set_inner_html(&data.text);

        html! {
            <>
                { Html::VRef(div.into()) }
            </>
        }
    } else {
        html! { <p>{"Loading..."}</p> }
    }
}

#[function_component(CountriesLoader)]
fn countries_loader() -> Html {
    let content = use_state(|| None::<CountriesContent>);
    {
        let content = content.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = Request::get("/text/about/countries.json").send().await {
                    if let Ok(json) = resp.json::<CountriesContent>().await {
                        content.set(Some(json));
                    }
                }
            });
            || ()
        });
    }

    if let Some(data) = &*content {
        html! {
            <>
                <p>{&data.based}</p>
                <p>{&data.from}</p>
                <p>{&data.lived}</p>
                <p>{&data.visited}</p>
            </>
        }
    } else {
        html! { <p>{"Loading..."}</p> }
    }
}

#[function_component(ContactLoader)]
fn contact_loader() -> Html {
    let content = use_state(|| None::<ContactContent>);
    {
        let content = content.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = Request::get("/text/about/contact.json").send().await {
                    if let Ok(json) = resp.json::<ContactContent>().await {
                        content.set(Some(json));
                    }
                }
            });
            || ()
        });
    }

    if let Some(data) = &*content {
        html! {
            <>
                { for data.links.iter().map(|link| html! {
                    <div class="same-line">
                        <img src={link.icon.clone()} alt={link.alt.clone()} class="contact"/>
                        <a href={link.url.clone()} target="_blank">{&link.text}</a>
                    </div>
                }) }
            </>
        }
    } else {
        html! { <p>{"Loading..."}</p> }
    }
}

#[function_component(WhyLoader)]
fn why_loader() -> Html {
    let content = use_state(|| None::<SimpleTextContent>);
    {
        let content = content.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = Request::get("/text/about/why_gabuzando.json").send().await {
                    if let Ok(json) = resp.json::<SimpleTextContent>().await {
                        content.set(Some(json));
                    }
                }
            });
            || ()
        });
    }

    if let Some(data) = &*content {
        html! { <p>{&data.text}</p> }
    } else {
        html! { <p>{"Loading..."}</p> }
    }
}

#[function_component(QuoteLoader)]
fn quote_loader() -> Html {
    let content = use_state(|| None::<Quote>);
    {
        let content = content.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = Request::get("/text/about/random_quote.json").send().await {
                    if let Ok(json) = resp.json::<QuotesContent>().await {
                        let mut rng = rand::thread_rng();
                        if let Some(quote) = json.quotes.choose(&mut rng) {
                            content.set(Some(quote.clone()));
                        }
                    }
                }
            });
            || ()
        });
    }

    if let Some(data) = &*content {
        html! {
            <>
                <p><q>{&data.text}</q></p>
                <p class="author">{&data.author}</p>
            </>
        }
    } else {
        html! { <p>{"Loading..."}</p> }
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
