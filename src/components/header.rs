use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <a href="/" class="profile-pic">
                <div class="title">
                    <img src="/images/profile.png" alt="Profile Picture"/>
                    <h1>{"Gabuzando"}</h1>
                </div>
            </a>
            <nav>
                <a href="/">{"Home"}</a>
                <a href="/cv">{"CV"}</a>
                <a href="/projects">{"Projects"}</a>
                <a href="/blog">{"Blog"}</a>
                <a href="/about">{"About"}</a>
                <a href="https://setup.gabuzando.dev">{"Setup"}</a>
                <a href="https://linkedin.com/in/gabrielhsc95" target="_blank" class="social-icon">
                    <img src="/images/linkedin.png" alt="LinkedIn"/>
                </a>
                <a href="https://github.com/gabrielhsc95" target="_blank" class="social-icon">
                    <img src="/images/github.png" alt="GitHub"/>
                </a>
            </nav>
        </header>
    }
}
