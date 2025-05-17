use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
mod about;
mod blog;
mod cv;
mod footer;
mod header;
mod home;
mod projects;
mod window;

use about::AboutPage;
use blog::BlogPage;
use cv::CVPage;
use footer::Footer;
use header::Header;
use home::HomePage;
use projects::ProjectsPage;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Header />
            <main>
                <Routes fallback=|| "Not found.">
                    <Route
                        path=path!("/")
                        view=HomePage
                    />
                    <Route
                        path=path!("/cv")
                        view=CVPage
                    />
                    <Route
                        path=path!("/projects")
                        view=ProjectsPage
                    />
                    <Route
                        path=path!("/blog")
                        view=BlogPage
                    />
                    <Route
                        path=path!("/about")
                        view=AboutPage
                    />
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
