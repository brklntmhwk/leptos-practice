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
                name: "Films".to_string(),
                active: route.path() == "/film-search",
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
          <ul class="flex gap-4 mt-2 bg-pink-300">
            {nav_items}
          </ul>
        </div>
      </nav>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
      <div class="">
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
      <div class="container mx-auto grid gap-5 overflow-y-auto">
        <Header/>
        <MainContainer>{children()}</MainContainer>
      </div>
    }
}
