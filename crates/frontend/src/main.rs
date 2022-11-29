use tauri_sys::tauri::invoke;
use yew::prelude::*;
use yew_hooks::prelude::*;

use types::UserInfo;

#[function_component(App)]
fn app() -> Html {
    // Get backend port automatically from tauri command.
    let port = use_async_with_options(
        async move {
            match invoke::<_, String>("get_port", &()).await {
                Ok(p) => Ok(p),
                Err(e) => Err(format!("Error: {:?}", e)),
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

    let onclick = {
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

    let onclickserver = {
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
    let onclick2 = {
        let ws = ws.clone();
        let history = history.clone();
        Callback::from(move |_| {
            let message = "Hello, backend!".to_string();
            ws.send(message.clone());
            history.push(format!("ws [send]: {}", message));
        })
    };
    let onopen = {
        let ws = ws.clone();
        Callback::from(move |_| {
            ws.open();
        })
    };

    html! {
        <>
            <p>
                <button {onclick}>{ "Load backend api" }</button>
                <button onclick={onclickserver}>{ "Load server api" }</button>
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
            <p>
                <button onclick={onopen} disabled={*ws.ready_state != UseWebSocketReadyState::Closed}>{ "Connect to backend websocket" }</button>
                <button onclick={onclick2} disabled={*ws.ready_state != UseWebSocketReadyState::Open}>{ "Send to backend websocket" }</button>
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
