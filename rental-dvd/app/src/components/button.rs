use leptos::*;
use leptos_dom::*;
use leptos_router::*;

#[component]
pub fn Button(
    cx: Scope,
    children: Children,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let count_up = move || set_count.update(|count| *count += 1);
    let clear_count = move |_| set_count.update(|count| *count = 0);

    view! {
      cx,
      <div>
        <button
          on:click=move |_| count_up()
          class=format!("{class}")
        >
          {children(cx)}
        </button>
        <button
          on:click=clear_count
          class=format!("{class}")
        >
          "Clear"
        </button>
        <p>"Count:" {count}</p>
        <p>{move || count.get()}</p>
      </div>
    }
}
