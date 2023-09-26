use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
// use sqlx::types::BigDecimal;

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Category {
    pub category_id: i32,
    pub name: String,
    pub last_update: NaiveDateTime,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct FilmCategory {
    pub film_id: i16,
    pub category_id: i16,
    pub last_update: NaiveDateTime,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Inventory {
    pub inventory_id: i32,
    pub film_id: i16,
    pub store_id: i16,
    pub last_update: NaiveDateTime,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Rental {
    pub rental_id: i32,
    pub rental_date: NaiveDateTime,
    pub inventory_id: i32,
    pub customer_id: i16,
    pub return_date: NaiveDateTime,
    pub staff_id: i16,
    pub last_update: NaiveDateTime,
}

#[derive(Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct Payment {
    pub payment_id: i32,
    pub customer_id: i16,
    pub staff_id: i16,
    pub rental_id: i32,
    pub amount: BigDecimal,
    pub payment_date: NaiveDateTime,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Staff {
    pub staff_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub address_id: i16,
    pub email: Option<String>,
    pub store_id: i16,
    pub active: bool,
    pub username: String,
    pub password: Option<String>,
    pub last_update: NaiveDateTime,
    pub picture: Option<Vec<u8>>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct City {
    pub city_id: i32,
    pub city: String,
    pub country_id: i16,
    pub last_update: NaiveDateTime,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Country {
    pub country_id: i32,
    pub country: String,
    pub last_update: NaiveDateTime,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Address {
    pub address_id: i32,
    pub address: String,
    pub address2: Option<String>,
    pub district: String,
    pub city_id: i16,
    pub postal_code: Option<String>,
    pub phone: String,
    pub last_update: NaiveDateTime,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Customer {
    pub customer_id: i32,
    pub store_id: i16,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub address_id: i16,
    pub activebool: bool,
    pub create_date: NaiveDate,
    pub last_update: NaiveDateTime,
    pub active: Option<i32>,
}

#[derive(Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct CustomerData {
    pub customer_id: i32,
    pub customer_first_name: String,
    pub customer_last_name: String,
    pub staff_first_name: String,
    pub staff_last_name: String,
    pub amount: BigDecimal,
    pub payment_date: NaiveDateTime,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Store {
    pub store_id: i32,
    pub manager_staff_id: i16,
    pub address_id: i16,
    pub last_update: NaiveDateTime,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Actor {
    pub actor_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub last_update: NaiveDateTime,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct FilmActor {
    pub actor_id: i16,
    pub film_id: i16,
    pub last_update: NaiveDateTime,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Language {
    pub language_id: i32,
    pub name: String,
    pub last_update: NaiveDateTime,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub enum Rating {
    #[serde(rename = "kebab-case")]
    NC17,
    G,
    PG,
    #[serde(rename = "kebab-case")]
    PG13,
    R,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Film {
    pub film_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub release_year: Option<i32>, // used to be year type
    pub language_id: i16,
    pub rental_duration: i16,
    pub rental_rate: BigDecimal,
    pub length: Option<i16>,
    pub replacement_cost: BigDecimal,
    // pub rating: Option<Rating>,
    pub rating: Option<String>, // used to be mpaa_rating
    pub last_update: NaiveDateTime,
    pub special_features: Option<Vec<String>>,
    pub fulltext: String, // used to be tsvector
}

// #[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
// pub enum Primitive {
//     Decimal(BigDecimal),
// }

// impl From<BigDecimal> for Primitive {
//     fn from(decimal: BigDecimal) -> Self {
//         Primitive::Decimal(decimal)
//     }
// }
// impl From<i32> for Primitive {
//     fn from(int: i32) -> Self {
//         Primitive::Decimal(BigDecimal::from_i32(int).unwrap())
//     }
// }
// impl From<i64> for Primitive {
//     fn from(int: i64) -> Self {
//         Primitive::Decimal(BigDecimal::from_i64(int).unwrap())
//     }
// }

// impl From<f64> for Primitive {
//     fn from(float: f64) -> Self {
//         Primitive::Decimal(BigDecimal::from_f64(float).unwrap())
//     }
// }

// #[sqlx(transparent)]
// #[derive(sqlx::Type, Debug, Default, Copy, Clone, PartialEq, PartialOrd, Deserialize)]
// pub struct CustomBigDecimal(f64);

// impl From<BigDecimal> for CustomBigDecimal {
//     fn from(value: BigDecimal) -> Self {
//         use bigdecimal::FromPrimitive;
//         Self(value.)
//         // Self(value.to_f64().unwrap_or_default())
//     }
// }

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
