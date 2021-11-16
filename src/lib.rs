use log::Level;
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone)]
enum AppLogic {
    Click,
}

#[derive(Clone)]
enum AppView {
    Clicked(u32),
}

async fn logic(mut rx_logic: broadcast::Receiver<AppLogic>, tx_view: broadcast::Sender<AppView>) {
    let mut clicks = 0u32;
    while let Some(msg) = rx_logic.next().await {
        match msg {
            AppLogic::Click => {
                clicks += 1;
                tx_view.broadcast(AppView::Clicked(clicks)).await.unwrap();
            }
        }
    }
}

fn view(
    tx_logic: broadcast::Sender<AppLogic>,
    rx_view: broadcast::Receiver<AppView>,
) -> ViewBuilder<Dom> {
    builder! {
        <div
         style:float="left"
         style:padding="1em"
         style:border_radius=".5em"
         style:border="1px solid #ddd"
         style:background="#f7f7f7"
         style:cursor="pointer"
         on:click=tx_logic.sink().contra_map(|_| AppLogic::Click)>
            <p>
                {(
                    "Hello from mogwai!",
                    rx_view.clone().map(|msg| {
                        match msg {
                            AppView::Clicked(1) => format!("Caught 1 click, click again 😀"),
                            AppView::Clicked(n) => format!("Caught {} clicks", n),
                        }
                    })
                )}
            </p>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();

    let (tx_logic, rx_logic) = broadcast::bounded(1);
    let (tx_view, rx_view) = broadcast::bounded(1);
    Component::from(view(tx_logic, rx_view))
        .with_logic(logic(rx_logic, tx_view))
        .build()
        .unwrap()
        .run()
}
