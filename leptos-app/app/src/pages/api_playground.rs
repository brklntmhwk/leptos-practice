use leptos::*;

use crate::{components::*, layouts::*, server_function::*};

#[component]
pub fn APIPlayground() -> impl IntoView {
    let json_post_data_resource = create_resource(
        || (),
        move |_| async move { fetch_posts().await.unwrap_or_default() },
    );

    view! {
        <MainLayout>
            <h1 class="text-4xl">"Request to Web API and get some responses!"</h1>
            <Transition fallback=move || {
                view! { <tr class="bg-red-700">"Loading..."</tr> }
            }>
              <For
                  each=move || json_post_data_resource.get().unwrap_or(vec![])
                  key=|post| post.user_id
                  children=move |post: Post| {
                      view! {
                        <div>{post.title}</div>
                      }
                  }
              />
            </Transition>
        </MainLayout>
    }
}
