use leptos::{html::Div, *};
use leptos_use::{
    core::Position, use_draggable_with_options, UseDraggableOptions, UseDraggableReturn,
};

use crate::{components::*, layouts::*};

#[component]
pub fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let count_up = move || set_count.update(|count| *count += 1);
    let clear_count = move |_| set_count.update(|count| *count = 0);

    // let el = create_node_ref::<Div>();
    // let UseDraggableReturn { x, y, style, .. } = use_draggable_with_options(
    //     el,
    //     UseDraggableOptions::default().initial_value(Position { x: 40.0, y: 40.0 }),
    // );

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
            // <div node_ref=el style=move || format!("position: fixed; {}", style.get())>
            // "Drag me! I am at" {x} {y}
            // </div>
        </MainLayout>
    }
}
