use leptos::*;
use leptos_dom::*;
use leptos_router::*;

use crate::{components::*, layouts::*, server_function::*};

#[component]
pub fn FilmSearchPage() -> impl IntoView {
    tracing::info!("FilmSearch Page");

    // let search_film = create_server_multi_action::<SearchFilms>();
    // let query = use_query_map();
    // let search = move || query().get("q").cloned().unwrap_or_default();
    // let search_results = create_resource(search, search_films);

    view! {
      <MainLayout>
        <h2>"Search Films"</h2>
        <Form method="GET" action="">
          <input type="search" name="search" value="search" />
        </Form>
        <Transition
          fallback=move || view! {
            <p>"Loading..."</p>
          }>
          <div>aaaa</div>
        </Transition>
      </MainLayout>
    }
}
