use std::panic;

use log::Level;
use mogwai::web::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(ViewChild)]
pub struct ButtonClick<V: View> {
    #[child]
    wrapper: V::Element,
    on_click: V::EventListener,
    clicks: Proxy<u32>,
}

impl<V: View> Default for ButtonClick<V> {
    fn default() -> Self {
        let mut clicks = Proxy::default();

        rsx! {
            let wrapper = button(
                style:float = "left",
                style:padding = "1em",
                style:border_radius = ".5em",
                style:border = "1px solid #ddd",
                style:background = "#f7f7f7",
                style:cursor = "pointer",
                on:click = on_click
            ) {
                p() {
                    {clicks(n => match *n {
                        0 => "Hello from mogwai!".to_string(),
                        1 => "Caught 1 click, click again 😀".to_string(),
                        n => format!("Caught {n} clicks"),
                    })}
                }
            }
        }

        Self {
            wrapper,
            clicks,
            on_click,
        }
    }
}

impl<V:View> ButtonClick<V> {
    pub async fn step(&mut self) {
        let _ev = self.on_click.next().await;
        self.clicks.modify(|n| *n += 1);
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();

    let mut view = ButtonClick::<Web>::default();
    mogwai::web::body().append_child(&view);
    wasm_bindgen_futures::spawn_local(async move {
        loop {
            view.step().await;
        }
    });
}
