use yew::prelude::*;
use yew_hooks::use_async;

#[function_component(App)]
fn app() -> Html {
    let state = use_async(async move {
        let response = reqwest::get("http://localhost:3000/").await;
        match response {
            Ok(data) => match data.text().await {
                Ok(body) => Ok(body),
                Err(_) => Err("Body Error".to_string()),
            },
            Err(_) => Err("Request Error".to_string()),
        }
    });
    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.run();
        })
    };

    html! {
        <>
            { "Hello, world" }
            <button {onclick}>{ "Load backend api" }</button>
            {
                if let Some(response) = &state.data {
                    html! {
                        <>
                            <p>{ "From backend: " }<b>{ response }</b></p>
                        </>
                        }
                } else {
                    html! {}
                }
            }
        </>
    }
}

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
