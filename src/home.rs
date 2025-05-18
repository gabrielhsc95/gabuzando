use crate::window::{WindowState, WindowWidget};
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let cv = WindowState::new(
        String::from("cv/experience"),
        view! {
            <p>"professional experience"</p>
        },
        30,
        110,
        490,
        780,
    );
    let about = WindowState::new(
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
        620,
        450,
    );
    let project_mine = WindowState::new(
        String::from("project/something"),
        view! {<p>"a project I did"</p>},
        550,
        590,
        300,
        300,
    );
    let blog = WindowState::new(
        String::from("blog/best"),
        view! {<p>"list, ordered by most likes, of blog posts."</p>},
        870,
        590,
        300,
        450,
    );
    let project_mentor = WindowState::new(
        String::from("project/mentor"),
        view! {
            <p>"a project a mentored"</p>
        },
        30,
        920,
        820,
        250,
    );
    let home = WindowState::new(
        String::from("home/greetings"),
        view! {
            <p>"hi"</p>
        },
        870,
        1070,
        300,
        100,
    );
    view! {
        <WindowWidget state=cv />
        <WindowWidget state=about />
        <WindowWidget state=project_mine />
        <WindowWidget state=blog />
        <WindowWidget state=project_mentor />
        <WindowWidget state=home />
    }
}
