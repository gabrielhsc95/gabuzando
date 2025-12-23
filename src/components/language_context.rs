use yew::prelude::*;
use crate::types::Language;

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageContextType {
    pub language: Language,
    pub set_language: Callback<Language>,
}

#[derive(Properties, PartialEq)]
pub struct LanguageProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(LanguageProvider)]
pub fn language_provider(props: &LanguageProviderProps) -> Html {
    let language = use_state(|| {
        if let Some(win) = web_sys::window() {
            if let Ok(Some(storage)) = win.local_storage() {
                if let Ok(Some(lang_str)) = storage.get_item("language") {
                    if let Ok(lang) = serde_json::from_str::<Language>(&lang_str) {
                        return lang;
                    }
                }
            }
        }
        Language::English
    });

    let set_language = {
        let language = language.clone();
        Callback::from(move |lang: Language| {
            if let Some(win) = web_sys::window() {
                if let Ok(Some(storage)) = win.local_storage() {
                    if let Ok(lang_str) = serde_json::to_string(&lang) {
                        let _ = storage.set_item("language", &lang_str);
                    }
                }
            }
            language.set(lang);
        })
    };

    let context_value = LanguageContextType {
        language: *language,
        set_language,
    };

    html! {
        <ContextProvider<LanguageContextType> context={context_value}>
            { props.children.clone() }
        </ContextProvider<LanguageContextType>>
    }
}

#[hook]
pub fn use_language() -> LanguageContextType {
    use_context::<LanguageContextType>().expect("Language context not found")
}
