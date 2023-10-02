#![allow(unused_imports)]
#![allow(dead_code)]
#![forbid(unsafe_code)]
pub mod error_template;

mod components;
mod pages;
mod server_function;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{components::*, error_template::*, pages::*};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Title text="Rental DVD Shop"/>
        <Router>
            <ErrorBoundary fallback=|cx, errors| {
                view! {
                    cx,
                    <ErrorTemplate errors=errors/>
                }
            }>
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <HomePage />} />
                </Routes>
            </ErrorBoundary>
        </Router>
    }
}
