use yew::prelude::*;

mod shiny_thing;
use shiny_thing::ShinyThing;

#[function_component(App)]
fn app() -> Html {
    let shiny_things = use_state(Vec::<ShinyThing>::new);
    {
        let shiny_things = shiny_things.clone();
        use_effect_with_deps(move |_| {
            let shiny_things = shiny_things;
            wasm_bindgen_futures::spawn_local(async move {
                let mut gen_shiny_things = vec![];
                for _ in 0..5 {
                    gen_shiny_things.push(
                        ShinyThing::gen_new()
                            .await
                            .unwrap_or_else(|e| panic!("building shiny thing failed: {e}"))
                    );
                }
                shiny_things.set(gen_shiny_things);
            });
            || ()
        }, ());
    }
    let shiny_things = (*shiny_things).iter().map(|s| html! {
        <p>{format!("{}: {}", s.name, s.url)}</p>
    }).collect::<Html>();
    html! {
        <>
            <h1>{ "shiny thing catalog" }</h1>
            <p>{ "under development" }</p>
            { shiny_things }
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}