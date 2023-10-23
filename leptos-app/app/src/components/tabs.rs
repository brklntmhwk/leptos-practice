use std::{fmt::Display, hash::Hash};

use leptos::{ev::MouseEvent, *};
use leptos_dom::*;
use leptos_router::*;

// #[derive(Clone, Debug)]
// pub struct TabItem {
//     pub name: String,
//     pub action: Fn(MouseEvent),
// }

// impl TabItem {
//     pub fn new(name: String, action: Fn(MouseEvent)) -> Self {
//         Self { name, action }
//     }
// }

// impl IntoView for TabItem {
//     fn into_view(self) -> View {
//         view! {
//           <li class="py-2 text-zinc-700 hover:text-zinc-500">
//             {self.name}
//           </li>
//         }
//         .into_view()
//         // match self.action {
//         //     Some(action) => view! {
//         //       <li class="py-2 text-zinc-700 hover:text-zinc-500">
//         //         <button on:click=action>{self.name}</button>
//         //       </li>
//         //     }
//         //     .into_view(),
//         //     None => view! {
//         //       <li class="py-2 text-zinc-700 hover:text-zinc-500">
//         //         {self.name}
//         //       </li>
//         //     }
//         //     .into_view(),
//         // }
//     }
// }

#[component]
pub fn Tabs<F, T>(tab_list: MaybeSignal<Vec<T>>, #[prop(optional)] on_click: F) -> impl IntoView
where
    F: Copy + Fn(MouseEvent) + 'static + Default,
    T: Clone + Copy + Display + Eq + PartialEq + Hash + IntoView + 'static,
{
    // let tab_list = move || {
    //     tab_list
    //         .iter()
    //         .map(|tab| TabItem {
    //             name: tab.to_string(),
    //             action: on_click,
    //         })
    //         .collect::<Vec<_>>()
    // };

    view! {
          <div class="w-full">
      <div class="relative right-0">
        <ul
          class="relative flex list-none flex-wrap rounded-lg bg-blue-gray-50/60 p-1"
          data-tabs="tabs"
          role="list"
        >
          <For
              each=move || tab_list.get()
              key=|tab| *tab
              children=move |tab: T| {
                  view! {
                      <li class="z-30 flex-auto text-center">
                        <button
                          class="text-slate-700 z-30 mb-0 flex w-full cursor-pointer items-center justify-center rounded-lg border-0 bg-inherit px-0 py-1 transition-all ease-in-out"
                          on:click=on_click
                        >
                          <span class="ml-1">{tab}</span>
                        </button>
                      </li>
                  }
              }
          />
        </ul>
      </div>
    </div>
        }
}
