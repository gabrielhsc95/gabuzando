use yew::prelude::*;

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
    // let mut rng = rand::rng();
    // let animals = vec![
    //     "monkey", "platypus", "phoenix", "koala", "house", "cat", "dog", "panda", "axolotl",
    //     "dolphin",
    // ];
    // let animal = animals
    //     .choose(&mut rng)
    //     .expect("There should have been an animal option to choose from!");
    // let adjectives = vec![
    //     "curious",
    //     "crazy",
    //     "majestic",
    //     "ancient",
    //     "gentle",
    //     "brilliant",
    //     "calm",
    //     "humble",
    //     "tenacious",
    //     "loyal",
    // ];
    // let adjective = adjectives
    //     .choose(&mut rng)
    //     .expect("There should have been an adjective option to choose from!");
    let animal = "monkey";
    let adjective = "curious";

    html! {
        <div>
            <p><b>{"Page not Found"}</b></p>
            <p>{format!("The {adjective} {animal} is looking into it!")}</p>
        </div>
    }
}
