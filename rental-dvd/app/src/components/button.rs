use leptos::*;
use leptos_dom::*;
use leptos_router::*;

#[component]
pub fn Button(children: Children) -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let count_up = move || set_count.update(|count| *count += 1);
    let clear_count = move |_| set_count.update(|count| *count = 0);

    view! {
      <div class="flex gap-4">
        <button
          on:click=move |_| count_up()
          class="btn"
        >
          {children()}
        </button>
        <button
          on:click=clear_count
          class="btn"
        >
          "Clear"
        </button>
        <p>"Count:" {count}</p>
        // <p>{move || count.get()}</p>
      </div>
    }
}
