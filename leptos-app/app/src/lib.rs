#![allow(unused_imports)]
#![allow(dead_code)]
#![forbid(unsafe_code)]
pub mod error_template;

mod components;
mod layouts;
mod pages;
mod server_function;
mod utils;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{components::*, error_template::*, layouts::*, pages::*};

/* Notes:
 * - The CSS file name will automatically be the "name" in Cargo.toml + ".css"
 * unless you explicitly add the "output-name"
 *  - i.e. output-name = "output" | <Stylesheet ~ href="/pkg/output.css"/>
 * - The "pkg" indicates the site root's relative dir defined as the "site-pkg-dir"
 * in Cargo.toml
 * - The `id="leptos"` in Stylesheet tells cargo-leptos to hot-reload the stylesheet
 */

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Leptos Practice"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/leptos-app.css"/>
        <Router>
            <ErrorBoundary fallback=|errors| {
                view! {
                    <ErrorTemplate errors=errors/>
                }
            }>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/todo" view=MyTodoListsPage/>
                    <Route path="/todo/:list_id" view=TodoListPage/>
                </Routes>
            </ErrorBoundary>
        </Router>
    }
}
