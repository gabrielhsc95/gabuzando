use yew::prelude::*;
use crate::types::Language;
use crate::components::language_context::use_language;
use crate::components::bongo_cat::BongoCat;
use crate::components::loading::Loading;
use serde::Deserialize;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, PartialEq, Deserialize)]
struct GitHubCommit {
    sha: String,
    html_url: String,
}

#[function_component(Footer)]
pub fn footer() -> Html {
    let language_context = use_language();
    let current_language = language_context.language;
    let show_menu = use_state(|| false);
    let commit_data = use_state(|| None::<GitHubCommit>);

    {
        let commit_data = commit_data.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let url = "https://api.github.com/repos/gabrielhsc95/gabuzando/commits/main";
                match Request::get(url).send().await {
                    Ok(response) => {
                        if let Ok(data) = response.json::<GitHubCommit>().await {
                            commit_data.set(Some(data));
                        }
                    }
                    Err(e) => log::error!("Failed to fetch commit: {}", e),
                }
            });
            || ()
        });
    }

    let toggle_menu = {
        let show_menu = show_menu.clone();
        Callback::from(move |_| show_menu.set(!*show_menu))
    };

    let on_select_en = {
        let set_language = language_context.set_language.clone();
        let show_menu = show_menu.clone();
        Callback::from(move |_| {
            set_language.emit(Language::English);
            show_menu.set(false);
        })
    };

    let on_select_pt = {
        let set_language = language_context.set_language.clone();
        let show_menu = show_menu.clone();
        Callback::from(move |_| {
            set_language.emit(Language::Portuguese);
            show_menu.set(false);
        })
    };

    let lang_text = match current_language {
        Language::English => "English",
        Language::Portuguese => "Português",
    };

    html! {
        <footer>
            <div class="footer-left">
                if let Some(commit) = (*commit_data).clone() {
                    <a href={commit.html_url} target="_blank" class="commit" style="text-decoration: none; color: inherit;">
                        <img src="/images/branch.png" alt="git branch"/>
                        {format!("main ({})", &commit.sha[0..7])}
                    </a>
                } else {
                    <div class="commit">
                        <img src="/images/branch.png" alt="git branch"/>
                        <Loading />
                    </div>
                }
            </div>

            <div class="footer-center">
                <div class="rights">
                    {"2026 - Gabriel Cardoso"}
                </div>
            </div>
            
            <div class="footer-right">
                <div class="language-status" onclick={toggle_menu}>
                    <span class="lang-icon">{"🌐"}</span>
                    {lang_text}
                    if *show_menu {
                        <div class="lang-popup">
                            <div onclick={on_select_en} class={classes!("lang-option", (current_language == Language::English).then(|| "active"))}>
                                {"English"}
                            </div>
                            <div onclick={on_select_pt} class={classes!("lang-option", (current_language == Language::Portuguese).then(|| "active"))}>
                                {"Português"}
                            </div>
                        </div>
                    }
                </div>
                <a href="https://marketplace.visualstudio.com/items?itemName=pixl-garden.BongoCat" target="_blank" style="text-decoration: none; color: inherit;">
                    <BongoCat />
                </a>
            </div>
        </footer>
    }
}
