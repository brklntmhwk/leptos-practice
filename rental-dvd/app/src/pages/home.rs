use leptos::*;

use crate::components::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <HeaderNav />
        <h1>"Welcome home!"</h1>
        <Button>aaaa</Button>
    }
}
