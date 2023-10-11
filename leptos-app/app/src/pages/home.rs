use leptos::*;

use crate::{components::*, layouts::*};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <MainLayout>
            <h1 class="text-4xl bg-yellow-500">"Welcome home!"</h1>
            <Button>"Count up"</Button>
        </MainLayout>
    }
}
