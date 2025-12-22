use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div class="commit">
                <img src="/images/branch.png" alt="git branch"/>
                {"main (abc123)"}
            </div>
            <div class="rights">
                {"2025 - Gabriel Cardoso"}
            </div>
            <div class="bongo-cat">
                <img src="/images/bongo-temp.png" alt="bongo cat"/>
            </div>
        </footer>
    }
}
