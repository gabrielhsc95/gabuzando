use crate::window::{WindowState, WindowWidget};
use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    let photo = WindowState::new(
        String::from("about/photo"),
        String::from("<img src=\"/images/photo.jpg\" alt=\"Gabriel's Photo\"/>"),
        30,
        110,
        485,
        680,
    );
    let me = WindowState::new(
        String::from("about/me"),
        String::from(
            "<p>I am a software developer, a loving partner, a proud step-dad, and an unapologetic nerd with a dash of delightful weirdness, all fueled by my Brazilian roots. Hailing from  <a href=\"https://maps.app.goo.gl/7jXanpdULSnsMbwj7\">Londrina, Paraná, Brazil</a>, my academic background is in the cosmos. I hold a Masters in Cosmology and Astrophysics.</p>
            <br />
            <p>Life took a fascinating turn, leading me to the finance industry. I have navigated roles from Financial Engineer (Analytical Quality Assurance) to my current position as a Quantitative Researcher. Ultimately, I see myself as a tool builder, constantly creating and finding solutions.</p>
            <br />
            <p>Beyond the world of finance, I am passionate about learning new things. You will often find me at the movies, happily coding personal projects, or immersed in the satisfying click of LEGO bricks.</p>"
        ),
        550,
        110,
        620,
        450,
    );
    let countries = WindowState::new(
        String::from("about/countries"),
        String::from(
            "<p>Based: 🇺🇸</p>
            <p>From: 🇧🇷</p>
            <p>Lived: 🇭🇺, 🇷🇸</p>
            <p>Visited: 🇦🇷, 🇵🇾, 🇸🇰, 🏴󠁧󠁢󠁥󠁮󠁧󠁿, 🇮🇹, 🇻🇦, 🇵🇹, 🇩🇪, 🇲🇽</p>",
        ),
        550,
        590,
        300,
        200,
    );
    let contact = WindowState::new(
        String::from("about/contact"),
        String::from(
            "<div class=\"same-line\"><img src=\"/images/gmail.png\" alt=\"Email\" class=\"contact\"/><a href=\"mailto:gabrielhsc95@gmail.com\">gabrielhsc95@gmail.com</a></div>
            <div class=\"same-line\"><img src=\"/images/instagram.png\" alt=\"Instagram\" class=\"contact\"/><a href=\"https://instagram.com/gabrielhsc95\" target=\"_blank\">@gabrielhsc95</a></div>
            <div class=\"same-line\"><img src=\"/images/linkedin.png\" alt=\"LinkedIn\" class=\"contact\"/><a href=\"https://linkedin.com/in/gabrielhsc95\" target=\"_blank\">/in/gabrielhsc95</a></div>
            <div class=\"same-line\"><img src=\"/images/github.png\" alt=\"GitHub\" class=\"contact\"/><a href=\"https://github.com/gabrielhsc95\" target=\"_blank\">gabrielhsc95</a></div>"
        ),
        880,
        590,
        290,
        200,
    );
    let why_gabuzando = WindowState::new(
        String::from("about/why_gabuzando"),
        String::from(
            "<p>My nickname is Gabu, which I playfully turned into a verb. To bring in a touch of my Brazilian roots, I used the gerund form 'gabuzando' instead of a direct English equivalent. So, 'gabuzando' essentially captures the dynamic essence of 'being Gabu,' and this website is where I will share all the random things I'm up to. I am 'gabuzando'.</p>",
        ),
        30,
        820,
        700,
        210,
    );
    let random_quote = WindowState::new(
        String::from("about/random_quote"),
        String::from(
            "<p><q>If I have seen further than others, it is by standing upon the shoulders of giants.</q></p>
            <p class=\"author\">Isaac Newton</p>"
        ),
        760,
        820,
        410,
        210,
    );
    view! {
        <WindowWidget state=photo />
        <WindowWidget state=me />
        <WindowWidget state=countries />
        <WindowWidget state=contact />
        <WindowWidget state=why_gabuzando />
        <WindowWidget state=random_quote />
    }
}
