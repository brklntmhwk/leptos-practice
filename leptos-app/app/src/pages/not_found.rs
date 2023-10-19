use leptos::*;

use crate::{components::*, layouts::*, server_function::*};

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <MainLayout>
            <h1 class="text-4xl">"Page Not Found"</h1>
        </MainLayout>
    }
}
