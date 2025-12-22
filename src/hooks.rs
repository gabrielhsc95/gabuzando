use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(String),
}

#[hook]
pub fn use_fetch<T>(url: &'static str) -> FetchState<T>
where
    T: DeserializeOwned + Clone + PartialEq + 'static,
{
    let state = use_state(|| FetchState::NotFetching);

    {
        let state = state.clone();
        use_effect_with((), move |_| {
            state.set(FetchState::Fetching);
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get(url).send().await {
                    Ok(resp) => {
                        if resp.ok() {
                            match resp.json::<T>().await {
                                Ok(json) => state.set(FetchState::Success(json)),
                                Err(e) => state.set(FetchState::Failed(format!("Parse error: {}", e))),
                            }
                        } else {
                            state.set(FetchState::Failed(format!("Fetch error: {}", resp.status_text())));
                        }
                    }
                    Err(e) => state.set(FetchState::Failed(format!("Network error: {}", e))),
                }
            });
            || ()
        });
    }

    (*state).clone()
}
