use app::*;
use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();
    tracing::info!("tracing on frontend...");

    leptos::mount_to_body(move || {
        view! { <App/> }
    });
}
