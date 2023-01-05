use log::Level;
use mogwai_dom::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn counter() -> ViewBuilder {
    let output_clicked = Output::<()>::default();
    let input_msg = Input::<String>::default();
    rsx! {
        div(
            style:float = "left",
            style:padding = "1em",
            style:border_radius = ".5em",
            style:border = "1px solid #ddd",
            style:background = "#f7f7f7",
            style:cursor = "pointer",
            on:click = output_clicked.sink().contra_map(|_: JsDomEvent| ())
        ) {
            p() {
                {(
                    "Hello from mogwai!",
                    input_msg.stream().unwrap(),
                )}
            }
        }
    }
    .with_task(async move {
        let mut clicks = 0u32;
        while let Some(()) = output_clicked.get().await {
            clicks += 1;

            input_msg.set(if clicks == 1 {
                "Caught 1 click, click again 😀".to_string()
            } else {
                format!("Caught {} clicks", clicks)
            }).await.unwrap();
        }
    })
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();

    let view = JsDom::try_from(counter())
        .unwrap()
        .run()
        .unwrap();

    Ok(())
}
