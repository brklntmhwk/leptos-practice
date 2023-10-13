use leptos::*;

use crate::{components::*, layouts::*};

#[component]
pub fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let count_up = move || set_count.update(|count| *count += 1);
    let clear_count = move |_| set_count.update(|count| *count = 0);

    view! {
        <MainLayout>
            <h1 class="text-4xl bg-yellow-500">"Welcome home!"</h1>
            <div class="flex gap-4">
                <Button on:click=move |_| count_up()>"Count up!"</Button>
                <Button
                    on:click=clear_count
                >
                    "Clear"
                </Button>
                <p>"Count:" {count}</p>
            </div>
        </MainLayout>
    }
}
