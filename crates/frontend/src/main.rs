use tauri_sys::core::invoke_result;
use yew::prelude::*;
use yew_hooks::prelude::*;

use types::UserInfo;

#[function_component(App)]
fn app() -> Html {
    // Get backend port automatically from tauri command.
    let port = use_async_with_options(
        async move {
                Ok(p) => Ok(p),
                Err(e) => Err(format!("Error: {:?}", e)),
            match invoke_result::<String, String>("get_port", &()).await {
            }
        },
        UseAsyncOptions::enable_auto(),
    );

    // Fetch data from backend.
    let state = {
        let port = port.clone();
        use_async(async move {
            match &port.data {
                Some(port) => {
                    let response = reqwest::get(format!("http://localhost:{}/user", port)).await;
                    match response {
                        Ok(data) => match data.json::<UserInfo>().await {
                            Ok(user) => Ok(user),
                            Err(_) => Err("Backend body Error".to_owned()),
                        },
                        Err(_) => Err("Backend request Error".to_owned()),
                    }
                }
                _ => Err("Backend is unavailable".to_owned()),
            }
        })
    };

    let on_load_backend_api = {
        let state = state.clone();
        Callback::from(move |_| {
            state.run();
        })
    };

    // Fetch data from server.
    let state_server = use_async(async move {
        let response = reqwest::get("http://localhost:3001/user").await;
        match response {
            Ok(data) => match data.json::<UserInfo>().await {
                Ok(user) => Ok(user),
                Err(_) => Err("Body Error".to_string()),
            },
            Err(_) => Err("Request Error".to_string()),
        }
    });

    let on_load_server_api = {
        let state_server = state_server.clone();
        Callback::from(move |_| {
            state_server.run();
        })
    };

    let history = use_list(vec![]);

    // Manually connect to websocket with custom options.
    let ws = {
        let history = history.clone();
        let port = port.data.clone().unwrap_or_default();
        use_websocket_with_options(
            format!("ws://localhost:{}/ws", port),
            UseWebSocketOptions {
                // Receive message by callback `onmessage`.
                onmessage: Some(Box::new(move |message| {
                    history.push(format!("ws [recv]: {}", message));
                })),
                manual: Some(true),
                ..Default::default()
            },
        )
    };
    let on_send_to_backend_websocket = {
        let ws = ws.clone();
        let history = history.clone();
        Callback::from(move |_| {
            let message = "Hello, backend!".to_string();
            ws.send(message.clone());
            history.push(format!("ws [send]: {}", message));
        })
    };
    let on_connect_to_backend_websocket = {
        let ws = ws.clone();
        Callback::from(move |_| {
            ws.open();
        })
    };

    html! {
        <>
            <p class="space-x-4 m-4">
                <button onclick={on_load_backend_api} class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 h-10 px-4 py-2 bg-slate-900 text-slate-100 hover:bg-slate-900/90">{ "Load backend api" }</button>
                <button onclick={on_load_server_api} class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 h-10 px-4 py-2 bg-slate-900 text-slate-100 hover:bg-slate-900/90">{ "Load server api" }</button>
            </p>
            {
                if let Some(response) = &state.data {
                    html! {
                        <p>{ "From backend: " }<b>{ &response.name }</b></p>
                    }
                } else {
                    html! {}
                }
            }
            {
                if let Some(response) = &state_server.data {
                    html! {
                        <p>{ "From server: " }<b>{ &response.name }</b></p>
                    }
                } else {
                    html! {}
                }
            }
            <p class="space-x-4 m-4">
                <button onclick={on_connect_to_backend_websocket} disabled={*ws.ready_state != UseWebSocketReadyState::Closed} class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 h-10 px-4 py-2 bg-slate-900 text-slate-100 hover:bg-slate-900/90">{ "Connect to backend websocket" }</button>
                <button onclick={on_send_to_backend_websocket} disabled={*ws.ready_state != UseWebSocketReadyState::Open} class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 h-10 px-4 py-2 bg-slate-900 text-slate-100 hover:bg-slate-900/90">{ "Send to backend websocket" }</button>
            </p>
            {
                for history.current().iter().map(|message| {
                    html! {
                        <p>{ message }</p>
                    }
                })
            }
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
