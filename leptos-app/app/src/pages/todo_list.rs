// use entity::film::{self, Model};
use leptos::*;
use leptos_dom::*;
use leptos_router::*;

use crate::{components::*, layouts::*, server_function::*};

#[component]
fn TodoTable() -> impl IntoView {
    view! {
      <></>
    }
}

#[component]
pub fn TodoList() -> impl IntoView {
    tracing::info!("FilmSearch Page");

    let (search_keyword, set_search_keyword) = create_signal("".to_string());
    let foo_action = create_server_action::<Foo>();
    // let search_films_action = create_server_action::<SearchFilms>();
    // let search_films = create_server_multi_action::<SearchFilms>();

    // let films_resource = create_resource(
    //     move || (search_keyword.get(), search_films_action.version().get()),
    //     move |_| async move {
    //         search_films(Some(search_keyword.get_untracked()))
    //             .await
    //             .unwrap_or_default()
    //     },
    // );
    let foo_resource = create_resource(
        move || foo_action.version().get(),
        move |_| {
            tracing::info!("fetching foo_source...");
            foo()
        },
    );
    // let films_resource = create_resource(
    //     move || search_keyword.get(),
    //     move |_| async move {
    //         search_films(Some(search_keyword.get_untracked()))
    //             .await
    //             .unwrap_or_default()
    //     },
    // );

    // let no_films = move || {
    //     if films_resource.get().unwrap_or_default().is_empty() {
    //         view! {
    //           <span>
    //             "No films found for the keyword: "
    //             <span class="font-medium underline underline-offset-4">{search_keyword.get()}</span>
    //           </span>
    //         }
    //         .into_view()
    //     } else {
    //         view! { <></> }.into_view()
    //     }
    // };

    view! {
      <MainLayout>
        <h1>"Search Films"</h1>
        <div class="relative">
          <div class="absolute top-3.5 left-2 text-zinc-400 h-5 w-5">
            {Svg::MagnifyingGlass}
          </div>
          <input
              id="film-search"
              type="text"
              name="search"
              placeholder="Search films..."
              class="rounded-sm py-3 pl-9 border border-zinc-300 bg-zinc-100 text-zinc-700 focus:outline focus:outline-offset-2 focus:outline-blue-300 w-full appearance-none"
              on:input=move |e| {
                set_search_keyword.set(event_target_value(&e));
              }
              prop:value=search_keyword
          />
        </div>
        <Transition
          fallback=move || view! {
            <p>"Loading..."</p>
          }>
          // {no_films()}
          // {move || {
          //   films_resource.get().map(|films| {
          //     films.iter()
          //     .map(|film| view! {
          //         <div>{&film.title}</div>
          //       }
          //     )
          //     .collect_view()
          //   })
          // }}
          {foo_resource.get()}
        </Transition>
      </MainLayout>
    }
}
