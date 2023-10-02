#[cfg(feature = "ssr")]
use std::sync::{
    atomic::{AtomicI32, Ordering},
    Arc,
};

#[cfg(feature = "ssr")]
use entity::{
    actor, address, category, city, country, customer, film, film_actor, film_category, inventory,
    language, payment,
    prelude::*,
    rental,
    sea_orm::{
        entity::ActiveModelTrait, query::QueryOrder, ColumnTrait, EntityTrait, ModelTrait,
        QueryFilter, Select,
    },
    sea_orm_active_enums, staff, store,
};
use leptos::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// use of deprecated associated function `leptos::ServerFn::register`: Explicit server function registration is no longer required on most platforms (including Linux, macOS, iOS, FreeBSD, Android, and Windows). If you are on another platform and need to explicitly register server functions, call ServerFn::register_explicit() instead.
// #[cfg(feature = "ssr")]
// pub fn register_server_functions() -> Result<(), ServerFnError> {
//     FindCustomer::register()?;
//     Ok(())
// }

#[cfg(feature = "ssr")]
pub fn db(cx: Scope) -> Result<Arc<entity::db::DB>, ServerFnError> {
    use_context::<Arc<entity::db::DB>>(cx)
        .ok_or("Pool missing.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

/* Server Functions
 * They must be async, return Result<T, ServerFnError>
 * Return types must implement serde::Serialize since args have to be sent to the server after being serialized
 * Args must implement serde::Serialize and serde::de::DeserializeOwned
 */

#[server(FindCustomer, "/api")]
pub async fn find_customer(cx: Scope, customer_id: i32) -> Result<customer::Model, ServerFnError> {
    let db = db(cx)?;

    let customer = customer::Entity::find_by_id(customer_id)
        .one(db.conn())
        .await
        .map_err(|e| ServerFnError::ServerError(format!("{e}")))?;

    if customer.is_none() {
        return Err(ServerFnError::ServerError("No customer found".to_string()));
    }

    Ok(customer.unwrap())
}

#[server(FetchAllCustomers, "/api")]
pub async fn fetch_all_customers(cx: Scope) -> Result<Vec<customer::Model>, ServerFnError> {
    let db = db(cx)?;

    let customers = customer::Entity::find()
        .all(db.conn())
        .await
        .map_err(|e| ServerFnError::ServerError(format!("{e}")))?;

    if customers.is_empty() {
        return Err(ServerFnError::ServerError("No customers found".to_string()));
    }

    Ok(customers.into_iter().collect())
}
