use gloo_timers::callback::Interval;
use yew::prelude::*;

#[function_component(Loading)]
pub fn loading() -> Html {
    let dots = use_state(|| "...");
    
    {
        let dots = dots.clone();
        use_effect_with((), move |_| {
            let interval = Interval::new(500, move || {
                let current = *dots;
                let next = match current {
                    "." => "..",
                    ".." => "...",
                    "..." => ".",
                    _ => ".",
                };
                dots.set(next);
            });
            
            move || drop(interval)
        });
    }

    html! {
        <p style="color: #888;">{ format!("Loading{}", *dots) }</p>
    }
}
