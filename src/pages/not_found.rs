use yew::prelude::*;
use rand::prelude::*;
use crate::components::language_context::use_language;
use crate::types::Language;

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
    let language = use_language().language;
    let mut rng = rand::thread_rng();

    let (title, message) = match language {
        Language::English => {
            let animals = vec![
                "monkey", "platypus", "phoenix", "koala", "horse", "cat", "dog", "panda", "axolotl",
                "dolphin"
            ];
            let animal = animals.choose(&mut rng).unwrap_or(&"monkey");
            
            let adjectives = vec![
                "curious", "crazy", "majestic", "ancient", "gentle", "brilliant", "calm", "humble",
                "tenacious", "loyal",
            ];
            let adjective = adjectives.choose(&mut rng).unwrap_or(&"curious");

            ("Page not Found", format!("The {adjective} {animal} is looking into it!"))
        },
        Language::Portuguese => {
            // Using masculine nouns to simplify adjective agreement
            let animals = vec![
                "macaco", "ornitorrinco", "fénix", "coala", " cavalo", "gato", "cachorro", "panda", "axolote",
                "golfinho"
            ];
            let animal = animals.choose(&mut rng).unwrap_or(&"macaco");

            let adjectives = vec![
                "curioso", "louco", "majestoso", "ancião", "gentil", "brilhante", "calmo", "humilde",
                "tenaz", "leal",
            ];
            let adjective = adjectives.choose(&mut rng).unwrap_or(&"curioso");

            ("Página não encontrada", format!("O {animal} {adjective} está cuidando disso!"))
        }
    };

    html! {
        <div>
            <p><b>{title}</b></p>
            <p>{message}</p>
        </div>
    }
}
