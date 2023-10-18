use leptos::{
    ev::{keypress, KeyboardEvent},
    *,
};
use leptos_use::{
    storage::use_local_storage, use_debounce_fn, use_event_listener, use_intl_number_format,
    use_window, UseIntlNumberFormatOptions,
};
// use leptos::{html::Div, *};
// use leptos_use::{
//     core::Position, use_draggable_with_options, UseDraggableOptions, UseDraggableReturn,
// };

use crate::{components::*, layouts::*};

#[component]
pub fn Home() -> impl IntoView {
    let (count, set_count, _) = use_local_storage("count-state", 0);

    // let (count, set_count) = create_signal(0);

    let count_up = move |_| set_count.update(|count| *count += 1);
    let clear_count = move |_| set_count.update(|count| *count = 0);

    // These output the same value but the latter is more efficient
    // when the derivation calculation costs expensive with memoization of the value
    let derived_signal_double_count = move || count.get() * 2;
    let memoized_double_count = create_memo(move |_| count.get() * 2);

    let nf = use_intl_number_format(
        UseIntlNumberFormatOptions::default().locale("zh-Hans-CN-u-nu-hanidec"),
    );

    let zh_count = nf.format::<i32>(count);

    let (key, set_key) = create_signal("".to_string());

    // window() doesn't work on the server so we provide use_window()
    let _ = use_event_listener(use_window(), keypress, move |evt: KeyboardEvent| {
        set_key.set(evt.key())
    });

    let (debounce_value, set_debounce_value) = create_signal("not called");

    let debounced_fn = use_debounce_fn(
        move || {
            set_debounce_value.set("called");
        },
        5000.0,
    );

    create_effect(move |_| {
        debounced_fn();
    });

    // let el = create_node_ref::<Div>();
    // let UseDraggableReturn {
    //     x,
    //     y,
    //     is_dragging,
    //     style,
    //     ..
    // } = use_draggable_with_options(
    //     el,
    //     UseDraggableOptions::default().initial_value(Position { x: 40.0, y: 40.0 }),
    // );

    view! {
        <MainLayout>
            <h1 class="text-4xl bg-yellow-500">"Welcome home!"</h1>
            <div class="flex gap-4">
                <Button on:click=count_up class="bg-sky-300 text-white">"Count up!"</Button>
                <Button
                    on:click=clear_count class="bg-zinc-400 text-white"
                >
                    "Clear"
                </Button>
                <p>"Count:" {count}</p>
                <p>"Memoized double count:" {memoized_double_count}</p>
                <p>"Derived signal double count:" {derived_signal_double_count}</p>
            </div>
            <p>
            "Locale zh-Hans-CN-u-nu-hanidec:"
            {zh_count}
        </p>
        <p>
            "Press any key:"
            {key}
        </p>
        <p>
            "Debounced called:"
            {debounce_value}
        </p>
            // <div node_ref=el style=move || format!("position: fixed; {}", style.get())>
            // "Drag me! I am at" {x.get()} {y.get()}
            // </div>
        </MainLayout>
    }
}
