use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage />} />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|cnt| *cnt += 1);

    view! {
        cx,
        <h1>"Welcome home!"</h1>
        <button on:click=on_click>"Count up"</button>
        <p>
            {move || count.get()}
        </p>
        <p>
            {count}
        </p>
    }
}
