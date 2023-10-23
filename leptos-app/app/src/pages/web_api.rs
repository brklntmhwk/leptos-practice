// use derive_more::Display;
use leptos::{error::Result, *};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};

use crate::{components::*, layouts::*, server_function::*};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Post {
    pub user_id: u32,
    pub id: u32,
    pub title: String,
    pub body: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Comment {
    pub post_id: u32,
    pub id: u32,
    pub name: String,
    pub email: String,
    pub body: String,
}

#[derive(
    Clone,
    Copy,
    Debug,
    EnumString,
    Display,
    IntoStaticStr,
    Hash,
    EnumIter,
    Eq,
    PartialEq,
    Deserialize,
    Serialize,
)]
enum DataType {
    #[strum(serialize = "comments")]
    Comment,
    #[strum(serialize = "posts")]
    Post,
    #[strum(serialize = "albums")]
    Album,
    #[strum(serialize = "photos")]
    Photo,
    #[strum(serialize = "users")]
    User,
    #[strum(serialize = "todos")]
    Todo,
}

// impl IntoView for DataType {
//     fn into_view(self) -> View {
//         view! {
//             <>{self}</>
//         }
//         .into_view()
//     }
// }

async fn fetch_posts(data_type: DataType) -> Result<Vec<Post>> {
    let res = Client::new()
        .get(format!(
            "https://jsonplaceholder.typicode.com/{}?_start=0&_end=10",
            data_type
        ))
        .send()
        .await?;
    let body = res.json::<Vec<Post>>().await?;

    Ok(body)
}

#[component]
pub fn WebAPI() -> impl IntoView {
    let (data_type, set_data_type) = create_signal::<DataType>(DataType::Post);
    // let (display_headers, set_display_headers) = create_signal::<Vec<ColumnHeader>>(post_headers);
    // let (all_data_types, _) = create_signal::<Vec<DataType>>(DataType::iter().collect::<Vec<_>>());
    let json_data_resource = create_local_resource(
        move || data_type.get(),
        |d_type| async move { fetch_posts(d_type).await.unwrap_or_default() },
    );

    let change_data_type = move |d_type: DataType| {
        set_data_type.set(d_type);
        // match d_type {
        //     DataType::Album => set_display_headers.set(album_headers),
        //     DataType::Comment => set_display_headers.set(comment_headers),
        //     DataType::Photo => set_display_headers.set(photo_headers),
        //     DataType::Post => set_display_headers.set(post_headers),
        //     DataType::Todo => set_display_headers.set(todo_headers),
        //     DataType::User => set_display_headers.set(user_headers),
        // };
    };

    let album_headers = vec![
        ColumnHeader {
            id: "user_id".to_string(),
            label: "UserId".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "id".to_string(),
            label: "Id".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "title".to_string(),
            label: "Title".to_string(),
            width: None,
            center: false,
        },
    ];

    let comment_headers = vec![
        ColumnHeader {
            id: "post_id".to_string(),
            label: "postId".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "id".to_string(),
            label: "Id".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "name".to_string(),
            label: "Name".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "email".to_string(),
            label: "Email".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "body".to_string(),
            label: "Body".to_string(),
            width: None,
            center: false,
        },
    ];

    let photo_headers = vec![
        ColumnHeader {
            id: "album_id".to_string(),
            label: "AlbumId".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "id".to_string(),
            label: "Id".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "title".to_string(),
            label: "Title".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "url".to_string(),
            label: "Url".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "thumbnail_url".to_string(),
            label: "ThumbnailUrl".to_string(),
            width: None,
            center: false,
        },
    ];

    let post_headers = vec![
        ColumnHeader {
            id: "user_id".to_string(),
            label: "UserId".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "id".to_string(),
            label: "Id".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "title".to_string(),
            label: "Title".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "body".to_string(),
            label: "Body".to_string(),
            width: None,
            center: false,
        },
    ];

    let todo_headers = vec![
        ColumnHeader {
            id: "user_id".to_string(),
            label: "UserId".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "id".to_string(),
            label: "Id".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "title".to_string(),
            label: "Title".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "completed".to_string(),
            label: "Completed".to_string(),
            width: None,
            center: false,
        },
    ];

    let user_headers = vec![
        ColumnHeader {
            id: "id".to_string(),
            label: "Id".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "name".to_string(),
            label: "Name".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "username".to_string(),
            label: "Username".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "email".to_string(),
            label: "Email".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "address".to_string(),
            label: "Address".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "phone".to_string(),
            label: "Phone".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "website".to_string(),
            label: "Website".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "company".to_string(),
            label: "Company".to_string(),
            width: None,
            center: false,
        },
    ];

    let no_data_row = move || {
        let data = json_data_resource.get();
        if data.is_none() || data.unwrap_or(vec![]).is_empty() {
            view! {
                <TableRow>
                    <TableCell colspan=2>
                        <div class="flex justify-center items-center">
                            <div class="flex text-gray-500 dark:text-gray-400">
                                <div class="w-6 h-6">{Svg::Logo}</div>
                                <span class="ml-2">
                                    "No data found."
                                </span>
                            </div>
                        </div>
                    </TableCell>
                </TableRow>
            }
            .into_view()
        } else {
            view! { <></> }.into_view()
        }
    };

    // let tab_list = DataType::iter().collect::<Vec<_>>();

    view! {
        <MainLayout>
            <h1>"Data Fetching from Web API"</h1>
            <p>"These are fetched from the most famous fake API, jsonplaceholder. Behind the scenes serves reqwest as an HTTP client."</p>
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
            <Table column_headers=post_headers>
            // <Table column_headers=move || display_headers.get()>
            <Transition fallback=move || {
                view! { <></> }
            }>{no_data_row()}</Transition>
                    <For
                        each=move || json_data_resource.get().unwrap_or(vec![])
                        key=|post| post.id
                        children=move |post: Post| {
                            view! {
                                <TableRow class="border-b">
                                    <TableCell>
                                        <span>{post.user_id}</span>
                                    </TableCell>
                                    <TableCell>
                                        <span>{post.id}</span>
                                    </TableCell>
                                    <TableCell>
                                        <span>{post.title}</span>
                                    </TableCell>
                                    <TableCell>
                                        <span>{post.body}</span>
                                    </TableCell>
                                </TableRow>
                            }
                        }
                    />
            </Table>
        </MainLayout>
    }
}
