#[cfg(feature = "ssr")]
use std::sync::Arc;

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
//     let _ = FindCustomer::register_explicit()?;
//     let _ = FetchAllCustomers::register_explicit()?;
//     let _ = Foo::register_explicit()?;
//     Ok(())
// }

#[cfg(feature = "ssr")]
pub fn db() -> Result<Arc<entity::db::DB>, ServerFnError> {
    use_context::<Arc<entity::db::DB>>()
        .ok_or("Pool missing.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

/* # Server Functions
 * They must be async, return Result<T, ServerFnError>
 * Return types must implement serde::Serialize since args have to be sent to the server after being serialized
 * Args must implement serde::Serialize and serde::de::DeserializeOwned
 */

#[server(SearchFilms, "/api", "GetJson")]
pub async fn search_films(keyword: Option<String>) -> Result<Vec<film::Model>, ServerFnError> {
    let db = db()?;

    // Then filter them by the keyword input on frontend if it's not empty
    let films = if let Some(keyword) = keyword {
        let filter = entity::sea_orm::Condition::any()
            .add(film::Column::Title.contains(&keyword))
            .add(film::Column::Description.contains(&keyword));

        film::Entity::find()
            .filter(filter)
            .all(db.conn())
            .await
            .map_err(|e| ServerFnError::ServerError(format!("Inside if let:{e}")))?
    } else {
        film::Entity::find()
            .all(db.conn())
            .await
            .map_err(|e| ServerFnError::ServerError(format!("Inside if let else {e}")))?
    };

    if films.is_empty() {
        return Err(ServerFnError::ServerError("No films found".to_string()));
    }

    Ok(films)
}

#[server(FindCustomer, "/api")]
pub async fn find_customer(customer_id: i32) -> Result<customer::Model, ServerFnError> {
    let db = db()?;

    let customer: Option<customer::Model> = customer::Entity::find_by_id(customer_id)
        .one(db.conn())
        .await
        .map_err(|e| ServerFnError::ServerError(format!("{e}")))?;

    if customer.is_none() {
        return Err(ServerFnError::ServerError("No customer found".to_string()));
    }

    Ok(customer.unwrap())
}

#[server(FetchAllCustomers, "/api")]
pub async fn fetch_all_customers() -> Result<Vec<customer::Model>, ServerFnError> {
    let db = db()?;

    let customers: Vec<customer::Model> = customer::Entity::find()
        .all(db.conn())
        .await
        .map_err(|e| ServerFnError::ServerError(format!("{e}")))?;

    if customers.is_empty() {
        return Err(ServerFnError::ServerError("No customers found".to_string()));
    }

    Ok(customers)
}

#[server(Foo, "/api")]
pub async fn foo() -> Result<String, ServerFnError> {
    Ok(String::from("Bar!"))
}

#[server(LoginToStaff, "/api")]
pub async fn login_to_staff(staff_id: i32, username: String) -> Result<(), ServerFnError> {
    let db = db()?;

    let staff = staff::Entity::find_by_id(staff_id)
        .filter(staff::Column::Username.eq(username))
        .one(db.conn())
        .await
        .map_err(|e| ServerFnError::ServerError(format!("{e}")))?;

    // Verification logic here...
    Ok(())
}

#[server(DeleteRental, "/api")]
pub async fn delete_rental(rental_id: i32) -> Result<(), ServerFnError> {
    let db = db()?;

    let rental = rental::Entity::find_by_id(rental_id)
        .one(db.conn())
        .await
        .map_err(|e| ServerFnError::ServerError(format!("{e}")))?
        .unwrap();

    rental
        .delete(db.conn())
        .await
        .map_err(|_| ServerFnError::ServerError("Failed to delete rental".to_string()))?;

    Ok(())
}

/* At a DVD rental store...
 * - customers should be able to rental and return DVDs with a machine or at the casher
 *  - Upon rentals and returns,
 * - staffs should be able to
 *
 * what customers are permitted to do are:
 * - AddRental
 * what staffs are permitted to do are:
 * - LoginToStaff authenticates the input info and redirects to the staff's dashboard
 * - AddCustomer creates a new member of the store when requested by a customer
 * - DeleteCustomer deletes a member when requested by a customer
 * what a DVD rental machine does are:
 * -
 * -
 */
