#![allow(non_snake_case)]
use std::{cell::RefCell, rc::Rc, sync::Arc};

use entity::{
    chrono,
    todos::{self, Model},
    uuid,
};
use js_sys::Function;
use leptos::{ev::MouseEvent, *};
use leptos_dom::*;
use leptos_router::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, DomRect, Element, Event, EventTarget, Window};

use super::add_to_local_storage;
use crate::{components::*, layouts::*, server_function::*};

#[component]
fn TodoRow(
    todo: todos::Model,
    delete_todo: Action<DeleteTodo, Result<(), ServerFnError>>,
    toggle_todo: Action<ToggleTodo, Result<(), ServerFnError>>,
    edit_todo: MultiAction<EditTodo, Result<(), ServerFnError>>,
) -> impl IntoView {
    let edit_todo_fields = vec![
        FormField {
            id: "id".to_string(),
            label: None,
            input_type: FormFieldInputType::Hidden,
            placeholder: "".to_string(),
            value: todo.id.to_string(),
            required: true,
        },
        FormField {
            id: "title".to_string(),
            label: Some("Title".to_string()),
            input_type: FormFieldInputType::Text,
            placeholder: "Do things".to_string(),
            value: todo.title.clone(),
            required: true,
        },
        FormField {
            id: "description".to_string(),
            label: Some("Description".to_string()),
            input_type: FormFieldInputType::TextArea,
            placeholder: "".to_string(),
            value: todo.description.clone().unwrap_or("".to_string()),
            required: false,
        },
        FormField {
            id: "due_date".to_string(),
            label: Some("Due Date".to_string()),
            input_type: FormFieldInputType::Date,
            placeholder: "".to_string(),
            value: todo
                .due_date
                .map(|dd| dd.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|| "".to_string()),
            required: false,
        },
    ];

    view! {
        <TableRow class="grid grid-cols-12 items-baseline rounded border md:table-row md:my-0 md:rounded-none md:border-b grid-rows-auto min-w-[20rem]">
            <TableCell
                on:click=move |_| toggle_todo.dispatch(ToggleTodo { id: todo.id.clone() })
                class="order-1 col-start-1 row-span-3 row-start-1 justify-self-center pointer-events-auto"
            >
                <div class="flex items-center">
                    <input
                        type="checkbox"
                        checked=todo.done
                        class="w-4 h-4 text-orange-600 bg-gray-100 rounded border-gray-300 dark:bg-gray-700 dark:border-gray-600 dark:ring-offset-gray-800 focus:ring-2 focus:ring-orange-500 dark:focus:ring-orange-600 dark:focus:ring-offset-gray-800"
                    />
                </div>
            </TableCell>
            <TableCell
                on:click=move |_| toggle_todo.dispatch(ToggleTodo { id: todo.id.clone() })
                class="order-2 col-span-8 col-start-2 row-start-1 p-4 min-w-0 text-lg font-medium text-gray-900 pointer-events-auto md:text-base dark:text-white truncate"
            >
                <span onclick="event.cancelBubble = true;">{todo.title.clone()}</span>
            </TableCell>
            <TableCell
                on:click=move |_| toggle_todo.dispatch(ToggleTodo { id: todo.id.clone() })
                class="overflow-y-auto overflow-x-hidden order-4 col-span-8 col-start-2 row-start-2 p-4 min-w-0 min-h-0 max-h-64 whitespace-pre pointer-events-auto text-ellipsis md:truncate"
            >
                <span onclick="event.cancelBubble = true;">{todo.description.clone()}</span>
            </TableCell>
            <TableCell
                on:click=move |_| toggle_todo.dispatch(ToggleTodo { id: todo.id.clone() })
                class="order-5 col-span-8 col-start-2 row-start-3 p-4 pointer-events-auto"
            >
                <span onclick="event.cancelBubble = true;">
                    <span class="md:hidden">"Due Date: "</span>
                    {todo
                        .due_date
                        .map(|dd| dd.format("%d.%m.%Y").to_string())
                        .unwrap_or_else(|| "".to_string())}
                </span>
            </TableCell>
            <TableCell class="grid order-3 grid-cols-1 col-span-2 col-end-12 grid-rows-2 row-span-3 row-start-1 gap-6 justify-items-center justify-self-end self-center p-4 pointer-events-auto md:flex-row md:grid-cols-2 md:grid-rows-1 md:px-2">
                <FormDrawerButton
                    button_class="border-none"
                    icon_class="w-6 h-6 fill-zinc-400"
                    action=edit_todo
                    title="Edit To-Do".to_string()
                    icon=Svg::Edit
                    fields=edit_todo_fields
                />
                <ActionForm action=delete_todo>
                    <input type="hidden" name="id" value=move || todo.id.to_string()/>
                    <Button class="border-none" button_type="submit">
                        <div class="w-6 h-6">{Svg::RubbishBin}</div>
                    </Button>
                </ActionForm>
            </TableCell>
        </TableRow>
    }
}

#[component]
fn AddTodoDrawer(
    list_id: uuid::Uuid,
    add_todo: MultiAction<AddTodo, Result<(), ServerFnError>>,
) -> impl IntoView {
    let add_todo_fields = vec![
        FormField {
            id: "list_id".to_string(),
            label: None,
            input_type: FormFieldInputType::Hidden,
            placeholder: "".to_string(),
            value: list_id.to_string(),
            required: true,
        },
        FormField {
            id: "title".to_string(),
            label: Some("Title".to_string()),
            input_type: FormFieldInputType::Text,
            placeholder: "Do things".to_string(),
            value: "".to_string(),
            required: true,
        },
        FormField {
            id: "description".to_string(),
            label: Some("Description".to_string()),
            input_type: FormFieldInputType::TextArea,
            placeholder: "".to_string(),
            value: "".to_string(),
            required: false,
        },
        FormField {
            id: "due_date".to_string(),
            label: Some("Due Date".to_string()),
            input_type: FormFieldInputType::Date,
            placeholder: "".to_string(),
            value: "".to_string(),
            required: false,
        },
    ];

    view! {
        <FormDrawerButton
            action=add_todo
            title="Add Todo".to_string()
            icon=Svg::AddSquare
            fields=add_todo_fields
            button_class=""
            icon_class="w-8 h-8 fill-zinc-600"
        />
    }
}

#[component]
fn TodoList(list_id: uuid::Uuid) -> impl IntoView {
    let add_todo = create_server_multi_action::<AddTodo>();
    let edit_todo = create_server_multi_action::<EditTodo>();
    let delete_todo = create_server_action::<DeleteTodo>();
    let toggle_todo = create_server_action::<ToggleTodo>();
    let list_resource = create_resource(
        move || (list_id,),
        move |_| async move { find_list(list_id).await },
    );

    let (search, set_search) = create_signal("".to_string());
    let list_todos_resource = create_resource(
        move || {
            (
                add_todo.version().get(),
                edit_todo.version().get(),
                delete_todo.version().get(),
                toggle_todo.version().get(),
                search.get(),
                list_id,
            )
        },
        move |_| async move {
            list_todos(list_id, Some(search.get()), None)
                .await
                .unwrap_or_default()
        },
    );

    let column_headers = vec![
        ColumnHeader {
            id: "checkbox".to_string(),
            label: "".to_string(),
            width: Some(4),
            center: false,
        },
        ColumnHeader {
            id: "title".to_string(),
            label: "Title".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "description".to_string(),
            label: "Description".to_string(),
            width: None,
            center: false,
        },
        ColumnHeader {
            id: "due_date".to_string(),
            label: "Due Date".to_string(),
            width: Some(16),
            center: false,
        },
        ColumnHeader {
            id: "action".to_string(),
            label: "Action".to_string(),
            width: Some(48),
            center: true,
        },
    ];

    let no_todos_row = move || {
        if list_todos_resource.get().unwrap_or_default().is_empty() {
            if search.get().is_empty() {
                view! {
                    <TableRow>
                        <TableCell colspan=5 class="col-span-5 text-center">
                            <div class="flex justify-center items-center">
                                <div class="flex text-gray-500 dark:text-gray-400">
                                    <div class="w-6 h-6">{Svg::Logo}</div>
                                    <span class="ml-2">
                                        "No to-dos found. Click the button on the top left to add a new to-do."
                                    </span>
                                </div>
                            </div>
                        </TableCell>
                    </TableRow>
                }.into_view()
            } else {
                view! {
                    <TableRow>
                        <TableCell colspan=5 class="col-span-5 text-center">
                            <div class="flex justify-center items-center">
                                <div class="flex text-gray-500 dark:text-gray-400">
                                    <div class="w-6 h-6">{Svg::Logo}</div>
                                    <span class="ml-2">
                                        "No to-dos found for the search term: "
                                        <span class="font-semibold">{search.get()}</span>
                                    </span>
                                </div>
                            </div>
                        </TableCell>
                    </TableRow>
                }
                .into_view()
            }
        } else {
            view! { <></> }.into_view()
        }
    };

    view! {
      <>
            <Transition fallback=move || {
                view! { <h1 class="bg-red-700">"Loading..."</h1> }
            }>
                <h1 class="mb-4 text-2xl font-semibold text-gray-900 dark:text-white">
                    {list_resource
                        .get()
                        .map(|list| list.expect("no list").title.clone())
                        .unwrap_or_else(|| "".to_string())}
                </h1>
            </Transition>
            <div class="overflow-x-auto relative border-0 border-gray-200 shadow-md md:rounded-lg md:border dark:border-gray-700">
                <div class="flex justify-between items-center p-2">
                    <AddTodoDrawer list_id=list_id add_todo=add_todo/>
                    <div class="relative">
                        <label for="table-search" class="sr-only">
                            "Search"
                        </label>
                        <div class="absolute left-0 top-2 items-center pl-3 text-gray-400 pointer-events-none">
                            <div class="w-6 h-6 fill-zinc-600 stroke-zinc-400">{Svg::MagnifyingGlass}</div>
                        </div>
                        <input
                            type="text"
                            id="table-search"
                            class="block p-2 pl-10 w-full text-sm text-gray-900 bg-gray-50 rounded-sm border border-gray-300 dark:placeholder-gray-400 dark:text-white dark:bg-gray-700 dark:border-gray-600 focus:border-orange-500 focus:ring-orange-500 min-w-[7em]"
                            placeholder="Search"
                            on:input=move |ev| {
                                set_search.set(event_target_value(&ev));
                            }
                            prop:value=search
                        />
                    </div>
                    <div class="relative">
                      <div class="absolute top-3.5 left-2 text-zinc-400 fill-zinc-600 stroke-zinc-400 h-6 w-6">
                        {Svg::MagnifyingGlass}
                      </div>
                      <input
                          id="search"
                          type="text"
                          name="search"
                          placeholder="Search todos.."
                          class="rounded-sm py-3 pl-9 border border-zinc-300 bg-zinc-100 text-zinc-700 focus:outline focus:outline-offset-2 focus:outline-blue-300 w-full appearance-none"
                          on:input=move |e| {
                            set_search.set(event_target_value(&e));
                          }
                          prop:value=search
                      />
                    </div>
                </div>
                <Transition fallback=move || {
                    view! { <tr class="bg-red-700">"Loading..."</tr> }
                }>
                    <Table column_headers=column_headers.clone()>
                        {no_todos_row()}
                        <For
                            each=move || list_todos_resource.get().unwrap_or(vec![])
                            key=|todo| todo.calc_hash()
                            children=move |todo: todos::Model| {
                                view! {
                                    <TodoRow todo=todo toggle_todo=toggle_todo delete_todo=delete_todo edit_todo=edit_todo/>
                                }
                            }
                        />
                    </Table>
                </Transition>
            </div>
        </>
    }
}

#[component]
pub fn TodoListPage() -> impl IntoView {
    let params = use_params_map();
    let list_id = move || {
        params.with(|params| {
            let str = params.get("list_id").cloned().unwrap_or_default().clone();

            uuid::Uuid::parse_str(&str).unwrap_or_default()
        })
    };

    create_effect(move |_| {
        let list_id = list_id();
        add_to_local_storage(list_id);
    });

    view! {
        <MainLayout>
            <TodoList list_id=list_id()/>
        </MainLayout>
    }
}
