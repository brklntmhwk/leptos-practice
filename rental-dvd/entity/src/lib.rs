#![forbid(unsafe_code)]
pub extern crate chrono;
pub extern crate sea_orm;

pub mod prelude;

#[cfg(feature = "server-side")]
pub mod db;

pub mod actor;
pub mod address;
pub mod category;
pub mod city;
pub mod country;
pub mod customer;
pub mod film;
pub mod film_actor;
pub mod film_category;
pub mod inventory;
pub mod language;
pub mod payment;
pub mod rental;
pub mod sea_orm_active_enums;
pub mod staff;
pub mod store;