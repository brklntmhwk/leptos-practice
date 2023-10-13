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
              <li class="py-2 text-zinc-300">
                {self.name}
              </li>
            }
            .into_view()
        } else {
            view! {
              <li class="py-2 text-zinc-700 hover:text-zinc-500">
                <A href=self.href()>{self.name}</A>
              </li>
            }
            .into_view()
        }
        .into_view()
    }
}

#[component]
fn Navbar() -> impl IntoView {
    let (menu_open, set_menu_open) = create_signal(false);
    let toggle_menu = move || set_menu_open.update(|curr| *curr = !*curr);
    let route = use_route();
    let nav_items = move || {
        vec![
            NavItem {
                path: "".to_string(),
                name: "Home".to_string(),
                active: route.path() == "",
            },
            NavItem {
                path: "/todo".to_string(),
                name: "Todo".to_string(),
                active: route.path().starts_with("/todo"),
            },
        ]
    };

    view! {
      <nav class="py-2.5 px-2 bg-purple-300 rounded-sm border-gray-200">
        <div class="flex flex-wrap justify-between items-center mx-auto">
          <A href="/" class="flex gap-3 items-center">
            <div class="h-8 w-8">
              {Svg::Logo}
            </div>
            <span class="text-base md:text-lg xl:text-xl font-semibold">"Rental DVD Shop"</span>
          </A>
          <button
              type="button"
              on:click=move |_| toggle_menu()
              class="inline-flex items-center p-2 ml-3 text-sm rounded-lg md:hidden focus:ring-2 focus:ring-gray-200 focus:outline-none"
          >
              <span class="sr-only">"Open main menu"</span>
              <div class="w-6 h-6 stroke-zinc-800">{Svg::HamburgerMenu}</div>
          </button>
          <div
              class="w-full md:block md:w-auto"
              class:hidden=move || !menu_open.get()
              id="navbar-default"
          >
              <ul class="flex flex-col p-4 mt-4 bg-gray-50 rounded-lg border border-gray-100 md:flex-row md:mt-0 md:space-x-8 md:text-sm md:font-medium md:bg-white md:border-0 dark:bg-gray-800 dark:border-gray-700 md:dark:bg-gray-900">
                  {nav_items}
              </ul>
          </div>
        </div>
      </nav>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
      <div>
        <Navbar/>
      </div>
    }
}

#[component]
fn MainContainer(children: Children) -> impl IntoView {
    view! {
      <div class="grid gap-8 p-3">
        {children()}
      </div>
    }
}

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    view! {
      <div class="container mx-auto flex flex-col gap-5 overflow-y-auto">
        <Header/>
        <MainContainer>{children()}</MainContainer>
      </div>
    }
}
