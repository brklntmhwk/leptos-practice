// use entity::film::{self, Model};
use leptos::*;
use leptos_router::*;

use crate::{components::*, layouts::*, server_function::*};

#[component]
pub fn TodoSingle() -> impl IntoView {
    let params = use_params_map();
    // let film_resource = create_resource(
    //     move || params.with(|p| p.get("id").cloned()),
    //     move |id| async move {
    //         if let Some(id) = id {
    //             fetch_film(id).await
    //         } else {
    //             Err(ServerFnError::MissingArg("No id provided..".to_string()))
    //         }
    //     },
    // );
    // let film = film_resource.get().unwrap().unwrap();

    view! {
        <div>"todo"</div>
    }
}

#[component]
pub fn NoTodo() -> impl IntoView {
    view! {
      <div>"No todo with the id found.."</div>
    }
}
