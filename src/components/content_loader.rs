use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ContentLoaderProps {
    pub url: AttrValue,
}

#[derive(Deserialize)]
struct GreetingContent {
    text: String,
}

#[function_component(ContentLoader)]
pub fn content_loader(props: &ContentLoaderProps) -> Html {
    let content = use_state(|| None::<String>);
    let error = use_state(|| None::<String>);

    {
        let content = content.clone();
        let error = error.clone();
        let url = props.url.clone();

        use_effect_with(url, move |url| {
            let url = url.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get(&url).send().await {
                    Ok(resp) => {
                        if resp.ok() {
                            match resp.json::<GreetingContent>().await {
                                Ok(json) => content.set(Some(json.text)),
                                Err(e) => error.set(Some(format!("Failed to parse JSON: {}", e))),
                            }
                        } else {
                            error.set(Some(format!("Failed to fetch: {}", resp.status_text())));
                        }
                    }
                    Err(e) => error.set(Some(format!("Network error: {}", e))),
                }
            });
            || ()
        });
    }

    if let Some(err) = &*error {
        return html! { <p style="color: red;">{ err }</p> };
    }

    if let Some(text) = &*content {
        html! { <p>{ text }</p> }
    } else {
        html! { <p>{ "Loading..." }</p> }
    }
}
