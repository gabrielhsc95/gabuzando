use crate::window::{WindowState, WindowWidget};
use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    let win_1 = WindowState::new(
        String::from("about/photo"),
        view! {<img src="/images/photo.jpg" alt="Gabriel's Photo"/>},
        30,
        110,
        485,
        680,
    );
    let win_2 = WindowState::new(
        String::from("about/me"),
        view! {
            <p>"I am a software developer, a proud step-dad, and an unapologetic nerd with a dash of delightful weirdness, all fueled by my Brazilian roots. Hailing from " <a href="https://maps.app.goo.gl/7jXanpdULSnsMbwj7">"Londrina, Paraná, Brazil"</a>", my academic background is in the cosmos. I hold a Masters in Cosmology and Astrophysics."</p>
            <br />
            <p>"Life took a fascinating turn, leading me to the finance industry. I have navigated roles from Financial Engineer (Analytical Quality Assurance) to my current position as a Quantitative Researcher. Ultimately, I see myself as a tool builder, constantly creating and finding solutions."</p>
            <br />
            <p>"Beyond the world of finance, I am passionate about learning new things. You will often find me at the movies, happily coding personal projects, or immersed in the satisfying click of LEGO bricks."</p>
        },
        550,
        110,
        630,
        450,
    );
    let win_3 = WindowState::new(
        String::from("about/countries"),
        view! {
            <p>"Based: 🇺🇸"</p>
            <p>"From: 🇧🇷"</p>
            <p>"Lived: 🇭🇺, 🇷🇸"</p>
            <p>"Visited: 🇦🇷, 🇵🇾, 🇸🇰, 🏴󠁧󠁢󠁥󠁮󠁧󠁿, 🇮🇹, 🇻🇦, 🇵🇹, 🇩🇪, 🇲🇽"</p>
        },
        550,
        590,
        300,
        200,
    );
    let win_4 = WindowState::new(
        String::from("about/contact"),
        view! {
            <div class="same-line"><img src="/images/gmail.png" alt="Email" class="contact"/>":"<a href="mailto:gabrielhsc95@gmail.com">"gabrielhsc95@gmail.com"</a></div>
            <div class="same-line"><img src="/images/instagram.png" alt="Instagram" class="contact"/>":"<a href="https://instagram.com/gabrielhsc95" target="_blank">"@gabrielhsc95"</a></div>
            <div class="same-line"><img src="/images/linkedin.png" alt="LinkedIn" class="contact"/>":"<a href="https://linkedin.com/in/gabrielhsc95" target="_blank">"/in/gabrielhsc95"</a></div>
            <div class="same-line"><img src="/images/github.png" alt="GitHub" class="contact"/>":"<a href="https://github.com/gabrielhsc95" target="_blank">"gabrielhsc95"</a></div>
        },
        880,
        590,
        300,
        200,
    );
    let win_5 = WindowState::new(
        String::from("about/why_gabuzando"),
        view! {
            <p>"My nickname is Gabu, which I playfully turned into a verb. To bring in a touch of my Brazilian roots, I used the gerund form 'gabuzando' instead of a direct English equivalent. So, 'gabuzando' essentially captures the dynamic essence of 'being Gabu,' and this website is where I will share all the random things I'm up to. I am 'gabuzando'."</p>
        },
        30,
        820,
        700,
        210,
    );
    let win_6 = WindowState::new(
        String::from("about/random_quote"),
        view! {
            <p><q>"If I have seen further than others, it is by standing upon the shoulders of giants."</q></p>
            <p class="author">"Isaac Newton"</p>
        },
        760,
        820,
        420,
        210,
    );
    view! {
        <WindowWidget state=win_1 />
        <WindowWidget state=win_2 />
        <WindowWidget state=win_3 />
        <WindowWidget state=win_4 />
        <WindowWidget state=win_5 />
        <WindowWidget state=win_6 />
    }
}
