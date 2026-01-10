use gloo_events::EventListener;
use gloo_timers::callback::Timeout;
use yew::prelude::*;
use log::info;

#[function_component(BongoCat)]
pub fn bongo_cat() -> Html {
    // 0 = both up, 1 = left down, 2 = right down (logic based on user request)
    
    let current_index = use_state(|| 0);
    // Initialize to true so the VALIDATION: first interaction triggers 'else' -> index 2 (Right Down)?
    // WAIT. 
    // Logic: if *left_was_last_down { 2 } else { 1 }
    // If true -> 2 (Right Down).
    // If false -> 1 (Left Down).
    // Original was 'false' -> started with Left.
    // User wants change? Let's start with 'true' -> triggers Right first?
    // Actually, usually Bongo Cat starts Left.
    // But maybe the user perceives the first TAP as "Right" hand on keyboard?
    // Let's try starting with true.
    let left_was_last_down = use_state(|| true);
    // keeping the handle alive in state so it doesn't get dropped immediately
    let timeout_handle = use_state(|| None::<Timeout>);

    let animate = {
        let current_index = current_index.clone();
        let left_was_last_down = left_was_last_down.clone();
        let timeout_handle = timeout_handle.clone();
        
        Callback::from(move |_| {
            info!("BongoCat animate triggered. Left was last down: {}", *left_was_last_down);
            let next_index = if *left_was_last_down { 2 } else { 1 };
            current_index.set(next_index);
            left_was_last_down.set(!*left_was_last_down);

            // Clear existing timeout
            timeout_handle.set(None);

            // Set new timeout to reset to 0
            let current_index_reset = current_index.clone();
            let handle = Timeout::new(500, move || {
                info!("BongoCat reset timeout triggered");
                current_index_reset.set(0);
            });
            timeout_handle.set(Some(handle));
        })
    };

    {
        let animate = animate.clone();
        use_effect(move || {
            let document = web_sys::window().unwrap().document().unwrap();
            let listener = EventListener::new(&document, "keydown", move |_| {
                animate.emit(());
            });
            move || drop(listener)
        });
    }
    
    // Character mapping verification:
    // a: Both Paws UP (Stale state)
    // b: Left Paw DOWN, Right Paw UP
    // c: Left Paw UP, Right Paw DOWN
    // d: Both Paws DOWN
    
    // 0: Stale -> a
    // 1: Left Down -> b
    // 2: Right Down -> c
    
    let cat = match *current_index {
        0 => "bc",
        1 => "ba",
        2 => "dc",
        _ => "bc",
    };

    html! {
        <div class="bongo-cat" onclick={move |_| animate.emit(())}>
            {cat}
        </div>
    }
}
