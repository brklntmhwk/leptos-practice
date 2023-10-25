use leptos::*;
use leptos_dom::*;
use leptos_router::*;

use super::*;
use crate::components::*;

#[component]
fn MainContainer(children: Children) -> impl IntoView {
    view! { <div class="grid gap-8 p-3">{children()}</div> }
}

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    view! {
        <div class="container mx-auto flex flex-col gap-5 overflow-y-auto">
            <Navbar/>
            <MainContainer>{children()}</MainContainer>
        </div>
    }
}
