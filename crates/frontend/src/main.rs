use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_hooks::{use_async, use_async_with_options, UseAsyncOptions};

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[function_component(App)]
fn app() -> Html {
    // Get backend url with a port automatically.
    let backend = use_async_with_options(
        async move {
            match get_port().await {
                Ok(p) => Ok(format!("http://localhost:{}/", p.as_string().unwrap())),
                Err(e) => Err(format!("Error: {:?}", e)),
            }
        },
        UseAsyncOptions::enable_auto(),
    );

    let state = use_async(async move {
        match &backend.data {
            Some(url) => {
                let response = reqwest::get(url).await;
                match response {
                    Ok(data) => match data.text().await {
                        Ok(body) => Ok(body),
                        Err(_) => Err("Backend body Error".to_owned()),
                    },
                    Err(_) => Err("Backend request Error".to_owned()),
                }
            }
            _ => Err("Backend is unavailable".to_owned()),
        }
    });

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.run();
        })
    };

    let state_server = use_async(async move {
        let response = reqwest::get("http://localhost:3001/").await;
        match response {
            Ok(data) => match data.text().await {
                Ok(body) => Ok(body),
                Err(_) => Err("Body Error".to_string()),
            },
            Err(_) => Err("Request Error".to_string()),
        }
    });

    let onclickserver = {
        let state_server = state_server.clone();
        Callback::from(move |_| {
            state_server.run();
        })
    };

    html! {
        <>
            <button {onclick}>{ "Load backend api" }</button>
            <button onclick={onclickserver}>{ "Load server api" }</button>
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
            {
                if let Some(response) = &state_server.data {
                    html! {
                        <>
                            <p>{ "From server: " }<b>{ response }</b></p>
                        </>
                        }
                } else {
                    html! {}
                }
            }
        </>
    }
}

#[wasm_bindgen(module = "/public/tauri.js")]
extern "C" {
    /// Get backend port from tauri commands.
    #[wasm_bindgen(js_name = getPort, catch)]
    pub async fn get_port() -> Result<JsValue, JsValue>;
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
