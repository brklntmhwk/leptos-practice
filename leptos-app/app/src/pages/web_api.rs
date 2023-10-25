// use derive_more::Display;
use async_trait::async_trait;
use leptos::{error::Result, *};
use leptos_struct_table::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use strum::{Display, EnumIter, EnumString, EnumProperty, IntoEnumIterator, IntoStaticStr};

use crate::{components::*, layouts::*, server_function::*};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, TableComponent)]
#[serde(rename_all = "camelCase")]
struct Post {
    #[table(key)]
    id: u32,
    user_id: u32,
    title: String,
    body: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, TableComponent)]
#[serde(rename_all = "camelCase")]
struct Comment {
    #[table(key)]
    id: u32,
    post_id: u32,
    name: String,
    email: String,
    body: String,
}

#[derive(
    Clone,
    // Copy,
    Debug,
    EnumProperty,
    // EnumString,
    Display,
    IntoStaticStr,
    // Hash,
    // EnumIter,
    // Eq,
    PartialEq,
    Deserialize,
    Serialize,
)]
enum DataType {
    #[strum(props(param = "comments"))]
    Comment,
    #[strum(props(param = "posts"))]
    Post,
    #[strum(props(param = "albums"))]
    Album,
    #[strum(props(param = "photos"))]
    Photo,
    #[strum(props(param = "users"))]
    User,
    #[strum(props(param = "todos"))]
    Todo,
    NoData,
}

// impl IntoView for DataType {
//     fn into_view(self) -> View {
//         view! {
//             <>{self}</>
//         }
//         .into_view()
//     }
// }

async fn fetch_data<T: for<'a> Deserialize<'a>>(data_type: DataType) -> Result<Vec<T>> {
    let res = Client::new()
        .get(format!(
            "https://jsonplaceholder.typicode.com/{}?_start=0&_end=10",
            data_type.get_str("param").unwrap()
        ))
        .send()
        .await?;
    let body = res.json::<Vec<T>>().await?;

    Ok(body)
}

#[component]
pub fn WebAPI() -> impl IntoView {
    let (data_type, set_data_type) = create_signal::<DataType>(DataType::NoData);
    let display_data = create_rw_signal(vec![]);

    let json_data_resource = create_local_resource(
        move || data_type.get(),
        move |d_type| async move {
          let data = fetch_data::<Post>(d_type).await.unwrap_or_default();
          // let data = match d_type {
          //   DataType::Comment => fetch_data::<Comment>(d_type).await.unwrap_or_default(),
          //   DataType::Post => fetch_data::<Post>(d_type).await.unwrap_or_default(),
          //   _ => vec![],
          //   // DataType::Album => fetch_data::<Post>(d_type).await.unwrap_or_default(),
          //   // DataType::Photo => fetch_data::<Post>(d_type).await.unwrap_or_default(),
          //   // DataType::Todo => fetch_data::<Post>(d_type).await.unwrap_or_default(),
          //   // DataType::User => fetch_data::<Post>(d_type).await.unwrap_or_default(),
          //   // DataType::NoData => fetch_data::<Post>(d_type).await.unwrap_or_default(),
          // };
          display_data.set(data);
        },
    );

    let change_data_type = move |d_type: DataType| {
        set_data_type.set(d_type);
    };

    // let no_data_row = move || {
    //     let data = display_data.get();
    //     // let data = json_data_resource.get();
    //     if data.is_none() || data.unwrap_or(vec![]).is_empty() {
    //         view! {
    //             <TableRow>
    //                 <TableCell colspan=2>
    //                     <div class="flex justify-center items-center">
    //                         <div class="flex text-gray-500 dark:text-gray-400">
    //                             <div class="w-6 h-6">{Svg::Logo}</div>
    //                             <span class="ml-2">"No data found."</span>
    //                         </div>
    //                     </div>
    //                 </TableCell>
    //             </TableRow>
    //         }
    //         .into_view()
    //     } else {
    //         view! { <></> }.into_view()
    //     }
    // };

    view! {
        <MainLayout>
            <h1>"Data Fetching from Web API"</h1>
            <p>
                "These are fetched from the most famous fake API, jsonplaceholder. Behind the scenes serves reqwest as an HTTP client."
            </p>
            // <Tabs tab_list=tab_list />
            // <Tabs tab_list=all_data_types.into() action= />
            <div class="w-full">
                <div class="relative right-0">
                    <ul
                        class="relative flex list-none flex-wrap rounded-lg bg-blue-gray-50/60 p-1"
                        data-tabs="tabs"
                        role="list"
                    >
                        <li class="z-30 flex-auto text-center">
                            <button
                                class="text-slate-700 z-30 mb-0 flex w-full items-center justify-center rounded-lg border-0 bg-inherit px-0 py-1 transition-all ease-in-out"
                                on:click=move |_| change_data_type(DataType::Album)
                            >
                                <span class="ml-1">"Album"</span>
                            </button>
                        </li>
                        <li class="z-30 flex-auto text-center">
                            <button
                                class="text-slate-700 z-30 mb-0 flex w-full items-center justify-center rounded-lg border-0 bg-inherit px-0 py-1 transition-all ease-in-out"
                                on:click=move |_| change_data_type(DataType::Comment)
                            >
                                <span class="ml-1">"Comment"</span>
                            </button>
                        </li>
                        <li class="z-30 flex-auto text-center">
                            <button
                                class="text-slate-700 z-30 mb-0 flex w-full items-center justify-center rounded-lg border-0 bg-inherit px-0 py-1 transition-all ease-in-out"
                                on:click=move |_| change_data_type(DataType::Photo)
                            >
                                <span class="ml-1">"Photo"</span>
                            </button>
                        </li>
                        <li class="z-30 flex-auto text-center">
                            <button
                                class="text-slate-700 z-30 mb-0 flex w-full items-center justify-center rounded-lg border-0 bg-inherit px-0 py-1 transition-all ease-in-out"
                                on:click=move |_| change_data_type(DataType::Post)
                            >
                                <span class="ml-1">"Post"</span>
                            </button>
                        </li>
                        <li class="z-30 flex-auto text-center">
                            <button
                                class="text-slate-700 z-30 mb-0 flex w-full items-center justify-center rounded-lg border-0 bg-inherit px-0 py-1 transition-all ease-in-out"
                                on:click=move |_| change_data_type(DataType::Todo)
                            >
                                <span class="ml-1">"Todo"</span>
                            </button>
                        </li>
                        <li class="z-30 flex-auto text-center">
                            <button
                                class="text-slate-700 z-30 mb-0 flex w-full items-center justify-center rounded-lg border-0 bg-inherit px-0 py-1 transition-all ease-in-out"
                                on:click=move |_| change_data_type(DataType::User)
                            >
                                <span class="ml-1">"User"</span>
                            </button>
                        </li>
                    </ul>
                </div>
            </div>
            {move || match data_type.get() {
                DataType::Album => {
                    view! { <PostTable items=display_data/> }
                }
                DataType::Comment => {
                    view! { <PostTable items=display_data/> }
                }
                DataType::Photo => {
                    view! { <PostTable items=display_data/> }
                }
                DataType::Post => {
                    view! { <PostTable items=display_data/> }
                }
                DataType::Todo => {
                    view! { <PostTable items=display_data/> }
                }
                DataType::User => {
                    view! { <PostTable items=display_data/> }
                }
                DataType::NoData => {
                    view! { <PostTable items=display_data/> }
                }
            }}

        // <Table column_headers=post_headers>
        // // <Table column_headers=move || display_headers.get()>
        // <Transition fallback=move || {
        // view! { <></> }
        // }>{no_data_row()}</Transition>
        // <For
        // each=move || json_data_resource.get().unwrap_or(vec![])
        // key=|post| post.id
        // children=move |post: Post| {
        // view! {
        // <TableRow class="border-b">
        // <TableCell>
        // <span>{post.user_id}</span>
        // </TableCell>
        // <TableCell>
        // <span>{post.id}</span>
        // </TableCell>
        // <TableCell>
        // <span>{post.title}</span>
        // </TableCell>
        // <TableCell>
        // <span>{post.body}</span>
        // </TableCell>
        // </TableRow>
        // }
        // }
        // />
        // </Table>
        </MainLayout>
    }
}
