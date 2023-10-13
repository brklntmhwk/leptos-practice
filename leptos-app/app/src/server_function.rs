#[cfg(feature = "ssr")]
use std::sync::Arc;

use entity::{
    chrono::{NaiveDate, ParseError},
    lists,
    prelude::*,
    sea_orm::{
        entity::ActiveModelTrait, query::QueryOrder, ColumnTrait, EntityTrait, ModelTrait,
        QueryFilter, Select,
    },
    todos, uuid,
};
use leptos::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(feature = "ssr")]
pub fn db() -> Result<entity::db::DB, ServerFnError> {
    use_context::<entity::db::DB>()
        .ok_or("Pool missing.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

/* # Server Functions
 * They must be async, return Result<T, ServerFnError>
 * Return types must implement serde::Serialize since args have to be sent to the server after being serialized
 * Args must implement serde::Serialize and serde::de::DeserializeOwned
 */

#[server(Foo, "/api")]
pub async fn foo() -> Result<String, ServerFnError> {
    Ok(String::from("Bar!"))
}

#[server(AddList, "/api")]
pub async fn add_list(title: String) -> Result<lists::Model, ServerFnError> {
    let db = db()?;

    let list = lists::ActiveModel::new(title)
        .insert(db.conn())
        .await
        .map_err(|e| {
            let str = format!("{e}");
            ServerFnError::ServerError(str)
        })?;

    Ok(list)
}

#[server(FindList, "/api")]
pub async fn find_list(list_id: uuid::Uuid) -> Result<lists::Model, ServerFnError> {
    let db = db()?;

    let list = lists::Entity::find_by_id(list_id)
        .one(db.conn())
        .await
        .map_err(|e| ServerFnError::ServerError(format!("{e}")))?;

    if list.is_none() {
        return Err(ServerFnError::ServerError("No list found".to_string()));
    }

    Ok(list.unwrap())
}

#[server(DeleteList, "/api")]
pub async fn delete_list(list_id: uuid::Uuid) -> Result<(), ServerFnError> {
    let db = db()?;

    let list = lists::Entity::find_by_id(list_id)
        .one(db.conn())
        .await
        .map_err(|e| ServerFnError::ServerError(format!("{e}")))?;

    if list.is_none() {
        return Err(ServerFnError::ServerError("No list found".to_string()));
    }

    list.unwrap()
        .delete(db.conn())
        .await
        .map_err(|_| ServerFnError::ServerError("No list deleted".to_string()))?;

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Order {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TodoOrderBy {
    Title(Order),
    DueDate(Order),
}

#[server(ListTodos, "/api")]
pub async fn list_todos(
    list_id: uuid::Uuid,
    search: Option<String>,
    order_by: Option<TodoOrderBy>,
) -> Result<Vec<todos::Model>, ServerFnError> {
    let db = db()?;

    let list = lists::Entity::find_by_id(list_id)
        .one(db.conn())
        .await
        .map_err(|e| ServerFnError::ServerError(format!("{e}")))?;

    if list.is_none() {
        return Err(ServerFnError::ServerError("No list found".to_string()));
    }

    let todos = list.unwrap().find_related(todos::Entity);

    let todos = if let Some(search) = search {
        let filter = entity::sea_orm::Condition::any()
            .add(todos::Column::Title.contains(&search))
            .add(todos::Column::Description.contains(&search));

        todos.filter(filter)
    } else {
        todos
    };

    let todos = todos
        .order_by_asc(todos::Column::CreatedAt)
        .all(db.conn())
        .await
        .map_err(|err| {
            tracing::error!("Failed to list todos: {}", err);
            ServerFnError::ServerError("No todos found".to_string())
        })?;

    Ok(todos)
}

#[server(AddTodo, "/api")]
pub async fn add_todo(
    list_id: uuid::Uuid,
    title: String,
    description: Option<String>,
    due_date: Option<String>,
) -> Result<(), ServerFnError> {
    let db = db()?;

    let due_date = due_date
        .and_then(|str| if str.is_empty() { None } else { Some(str) })
        .map(|string| {
            let naive_date = NaiveDate::parse_from_str(&string, "%Y-%m-%d")
                .map_err(|op| ServerFnError::ServerError(format!("{}", op)))?;

            // type annotation needed here
            Ok::<NaiveDate, ServerFnError>(naive_date)
        })
        .transpose()?;

    todos::ActiveModel::new(list_id, title, description, due_date)
        .insert(db.conn())
        .await
        .map_err(|e| {
            let str = format!("{e}");
            ServerFnError::ServerError(str)
        })?;

    Ok(())
}

#[server(DeleteTodo, "/api")]
pub async fn delete_todo(id: uuid::Uuid) -> Result<(), ServerFnError> {
    let db = db()?;

    let todo = todos::Entity::find_by_id(id)
        .one(db.conn())
        .await
        .map_err(|_| ServerFnError::ServerError("No todo found".to_string()))?
        .expect("should be unreachable #160");

    todo.delete(db.conn())
        .await
        .map_err(|_| ServerFnError::ServerError("No todo deleted".to_string()))?;
    Ok(())
}

#[server(EditTodo, "/api")]
pub async fn edit_todo(
    id: uuid::Uuid,
    title: String,
    description: Option<String>,
    due_date: Option<String>,
) -> Result<(), ServerFnError> {
    let db = db()?;

    let due_date = due_date
        .and_then(|str| if str.is_empty() { None } else { Some(str) })
        .map(|string| {
            let naive_date = NaiveDate::parse_from_str(&string, "%Y-%m-%d")
                .map_err(|op| ServerFnError::ServerError(format!("{}", op)))?;

            // type annotation needed here
            Ok::<NaiveDate, ServerFnError>(naive_date)
        })
        .transpose()?;

    let mut updated: todos::ActiveModel = todos::Entity::find_by_id(id)
        .one(db.conn())
        .await
        .map_err(|_| ServerFnError::ServerError("No todo found".to_string()))?
        .expect("should be unreachable #183")
        .into();

    updated.title = entity::sea_orm::Set(title);
    updated.description = entity::sea_orm::Set(description);
    updated.due_date = entity::sea_orm::Set(due_date);

    updated
        .update(db.conn())
        .await
        .map_err(|_| ServerFnError::ServerError("No to-do updated".to_string()))?;

    Ok(())
}

#[server(ToggleTodo, "/api")]
pub async fn toggle_todo(id: uuid::Uuid) -> Result<(), ServerFnError> {
    let db = db()?;
    let mut updated: todos::ActiveModel = todos::Entity::find_by_id(id)
        .one(db.conn())
        .await
        .map_err(|_| ServerFnError::ServerError("No to-do found".to_string()))?
        .unwrap()
        .into();

    updated.done = entity::sea_orm::Set(!updated.done.unwrap());

    updated
        .update(db.conn())
        .await
        .map_err(|_| ServerFnError::ServerError("No to-do updated".to_string()))?;

    Ok(())
}
