use leptos::*;
use leptos_dom::*;
use leptos_router::*;

#[component]
pub fn Button(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] button_type: String,
) -> impl IntoView {
    view! {
      <button
        type=button_type
        class=format!("btn {class}")
      >
        {children()}
      </button>
    }
}
