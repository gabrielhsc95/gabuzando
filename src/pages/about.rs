use yew::prelude::*;
use crate::components::window::WindowProps;

pub fn get_about_windows() -> Vec<WindowProps> {
    vec![
        // Photo
        WindowProps {
            title: AttrValue::from("about/photo"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <img src="/images/photo.jpg" alt="Gabriel's Photo" style="width:100%; height:100%; object-fit:cover;" />
            }]),
            x: 1.0,
            y: 10.0,
            width: 31.6,
            height: 55.6,
            buffer: 20.0,
        },
        // Me
        WindowProps {
            title: AttrValue::from("about/me"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <>
                    <p>{"I am a software developer, a loving partner, a proud step-dad, and an unapologetic nerd with a dash of delightful weirdness, all fueled by my Brazilian roots. Hailing from "} <a href="https://maps.app.goo.gl/7jXanpdULSnsMbwj7">{"Londrina, Paraná, Brazil"}</a>{", my academic background is in the cosmos. I hold a Masters in Cosmology and Astrophysics."}</p>
                    <br />
                    <p>{"Life took a fascinating turn, leading me to the finance industry. I have navigated roles from Financial Engineer (Analytical Quality Assurance) to my current position as a Quantitative Researcher. Ultimately, I see myself as a tool builder, constantly creating and finding solutions."}</p>
                    <br />
                    <p>{"Beyond the world of finance, I am passionate about learning new things. You will often find me at the movies, happily coding personal projects, or immersed in the satisfying click of LEGO bricks."}</p>
                </>
            }]),
            x: 34.6,
            y: 10.0,
            width: 64.2,
            height: 27.3,
            buffer: 20.0,
        },
        // Countries
        WindowProps {
            title: AttrValue::from("about/countries"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <>
                    <p>{"Based: 🇺🇸"}</p>
                    <p>{"From: 🇧🇷"}</p>
                    <p>{"Lived: 🇭🇺, 🇷🇸"}</p>
                    <p>{"Visited: 🇦🇷, 🇵🇾, 🇸🇰, 🏴󠁧󠁢󠁥󠁮󠁧󠁿, 🇮🇹, 🇻🇦, 🇵🇹, 🇩🇪, 🇲🇽"}</p>
                </>
            }]),
            x: 34.6,
            y: 39.3,
            width: 30.6,
            height: 26.3,
            buffer: 20.0,
        },
        // Contact
        WindowProps {
            title: AttrValue::from("about/contact"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <>
                    <div class="same-line"><img src="/images/gmail.png" alt="Email" class="contact"/><a href="mailto:gabrielhsc95@gmail.com">{"gabrielhsc95@gmail.com"}</a></div>
                    <div class="same-line"><img src="/images/instagram.png" alt="Instagram" class="contact"/><a href="https://instagram.com/gabrielhsc95" target="_blank">{"@gabrielhsc95"}</a></div>
                    <div class="same-line"><img src="/images/linkedin.png" alt="LinkedIn" class="contact"/><a href="https://linkedin.com/in/gabrielhsc95" target="_blank">{"/in/gabrielhsc95"}</a></div>
                    <div class="same-line"><img src="/images/github.png" alt="GitHub" class="contact"/><a href="https://github.com/gabrielhsc95" target="_blank">{"gabrielhsc95"}</a></div>
                </>
            }]),
            x: 67.2,
            y: 39.3,
            width: 31.6,
            height: 26.3,
            buffer: 20.0,
        },
        // Why Gabuzando
        WindowProps {
            title: AttrValue::from("about/why_gabuzando"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <p>{"My nickname is Gabu, which I playfully turned into a verb. To bring in a touch of my Brazilian roots, I used the gerund form 'gabuzando' instead of a direct English equivalent. So, 'gabuzando' essentially captures the dynamic essence of 'being Gabu,' and this website is where I will share all the random things I'm up to. I am 'gabuzando'."}</p>
            }]),
            x: 1.0,
            y: 67.6,
            width: 64.2,
            height: 27.3,
            buffer: 20.0,
        },
        // Random Quote
        WindowProps {
            title: AttrValue::from("about/random_quote"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <>
                    <p><q>{"If I have seen further than others, it is by standing upon the shoulders of giants."}</q></p>
                    <p class="author">{"Isaac Newton"}</p>
                </>
            }]),
            x: 67.2,
            y: 67.6,
            width: 31.6,
            height: 27.3,
            buffer: 20.0,
        },
    ]
}
