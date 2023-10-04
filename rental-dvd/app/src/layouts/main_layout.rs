use leptos::*;
use leptos_dom::*;
use leptos_router::*;

use super::*;
use crate::{components::*, server_function::*};

#[derive(Clone, Debug)]
struct NavItem {
    path: String,
    name: String,
    active: bool,
}

impl NavItem {
    fn new(path: String, name: String, active: bool) -> Self {
        Self { path, name, active }
    }

    fn href(&self) -> String {
        if !self.path.starts_with('/') {
            let mut href = self.path.clone();
            href.insert(0, '/');
            href
        } else {
            self.path.clone()
        }
    }
}

impl IntoView for NavItem {
    fn into_view(self) -> View {
        if self.active {
            view! {
              <li>
                <A
                    href=self.href()
                    class="block py-2 pr-4 pl-3 text-white bg-orange-500 rounded md:p-0 md:text-orange-500 md:bg-transparent dark:text-white"
                >
                    {self.name}
                </A>
              </li>
            }.into_view()
        } else {
            view! {
              <li>
                <A
                    href=self.href()
                    class="block py-2 pr-4 pl-3 text-gray-700 rounded md:p-0 md:border-0 dark:text-gray-400 hover:bg-gray-100 md:hover:bg-transparent md:hover:text-blue-700 md:dark:hover:text-white md:dark:hover:bg-transparent dark:hover:bg-gray-700 dark:hover:text-white"
                >
                    {self.name}
                </A>
              </li>
            }.into_view()
        }.into_view()
    }
}

#[component]
fn Navbar() -> impl IntoView {
    let route = use_route();
    let nav_items = move || {
        vec![
            NavItem {
                path: "".to_string(),
                name: "Home".to_string(),
                active: route.path() == "",
            },
            NavItem {
                path: "/film-search".to_string(),
                name: "Film Search".to_string(),
                active: route.path().starts_with("/film-search"),
            },
        ]
    };

    view! {
      <nav class="py-2.5 px-2 bg-purple-300 rounded border-gray-200">
        <ul class="flex gap-4 mt-4 bg-pink-300 border border-gray-100">
          {nav_items}
        </ul>
      </nav>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
      <div class="container">
        <Navbar/>
      </div>
    }
}

#[component]
fn MainContainer(children: Children) -> impl IntoView {
    view! {
      <div class="container grid gap-8 mx-auto p-3">
        {children()}
      </div>
    }
}

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    view! {
      <div class="grid gap-5 overflow-y-auto">
        <Header/>
        <MainContainer>{children()}</MainContainer>
      </div>
    }
}
