use leptos::*;
use leptos_dom::*;
use leptos_router::*;

#[component]
fn HeaderNavMenu(cx: Scope) -> impl IntoView {
    view! {
      cx,
      <div>"Header Nav"</div>
    }
}

#[component]
pub fn HeaderNav(cx: Scope) -> impl IntoView {
    view! {
      cx,
      <h1>"Header"</h1>
      <div><HeaderNavMenu /></div>
    }
}
