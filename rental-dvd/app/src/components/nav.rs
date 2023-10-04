use leptos::*;
use leptos_dom::*;
use leptos_router::*;

#[component]
fn HeaderNavMenu() -> impl IntoView {
    view! {
      <div class="bg-zinc-400 w-full">"Header Nav"</div>
    }
}

#[component]
pub fn HeaderNav() -> impl IntoView {
    view! {
      <h1>"Header"</h1>
      <div><HeaderNavMenu /></div>
    }
}
