use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
            <div style="display: flex; align-items: center;">
                <img src="/images/profile.png" alt="Profile Picture" class="profile-pic"/>
                <h1>"Gabuzando"</h1>
            </div>

            <nav>
                <a href="/">"Home"</a>
                <a href="/cv">"CV"</a>
                <a href="/projects">"Projects"</a>
                <a href="/blog">"Blog"</a>

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
