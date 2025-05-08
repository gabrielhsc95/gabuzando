use leptos::prelude::*;

mod header;
use header::Header;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Header/>

    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
