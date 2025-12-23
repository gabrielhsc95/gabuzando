use yew::prelude::*;
use crate::types::Language;
use crate::components::language_context::use_language;

#[function_component(Footer)]
pub fn footer() -> Html {
    let language_context = use_language();
    let current_language = language_context.language;
    let show_menu = use_state(|| false);

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
                <div class="commit">
                    <img src="/images/branch.png" alt="git branch"/>
                    {"main (abc123)"}
                </div>
            </div>

            <div class="footer-center">
                <div class="rights">
                    {"2025 - Gabriel Cardoso"}
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
                <div class="bongo-cat">
                    <img src="/images/bongo-temp.png" alt="bongo cat"/>
                </div>
            </div>
        </footer>
    }
}
