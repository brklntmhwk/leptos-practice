use app::*;
use common::*;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let db_url = dotenvy::var("DATABASE_URL").expect(".env file not found");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Unable to connect to Postgres..");

    let mut tx = pool.begin().await.expect("Unable to begin transaction..");

    println!("---Category---");

    let categories: Vec<Category> = sqlx::query_as!(Category, "SELECT * FROM category")
        .fetch_all(&pool)
        .await
        .expect("Unable to query category table..");

    for cat in categories.iter() {
        println!("{:?}", cat);
    }

    println!("---City---");

    let cities: Vec<City> = sqlx::query_as!(City, "SELECT * FROM city LIMIT 10")
        .fetch_all(&pool)
        .await
        .expect("Unable to query city table..");

    for city in cities.iter() {
        println!("{:?}", city);
    }

    println!("---Actor---");

    let actors: Vec<Actor> = sqlx::query_as!(Actor, "SELECT * FROM actor LIMIT 10")
        .fetch_all(&pool)
        .await
        .expect("Unable to query actor table..");

    for actor in actors.iter() {
        println!("{:?}", actor);
    }

    println!("---FilmActor---");

    let film_actors: Vec<FilmActor> =
        sqlx::query_as!(FilmActor, "SELECT * FROM film_actor LIMIT 10")
            .fetch_all(&pool)
            .await
            .expect("Unable to query film_actor table..");

    for film_actor in film_actors.iter() {
        println!("{:?}", film_actor);
    }

    println!("---Payment---");

    let payments: Vec<Payment> = sqlx::query_as!(
        Payment,
        r#"
            SELECT
                payment_id, customer_id, staff_id, rental_id, amount as "amount: i32", payment_date
            FROM payment LIMIT 10
            "#
    )
    .fetch_all(&pool)
    .await
    .expect("Unable to query payment table..");

    for payment in payments.iter() {
        println!("{:?}", payment);
    }

    println!("---Staff---");

    let staffs: Vec<Staff> = sqlx::query_as!(Staff, "SELECT * FROM staff LIMIT 10")
        .fetch_all(&pool)
        .await
        .expect("Unable to query staff table..");

    for staff in staffs.iter() {
        println!("{:?}", staff);
    }

    println!("--- Staff, Payment, Customer INNER JOIN ---");

    let customers_with_payment_details: Vec<CustomerData> = sqlx::query_as!(
        CustomerData,
        r#"
            SELECT
                c.customer_id,
                c.first_name customer_first_name,
                c.last_name customer_last_name,
                s.first_name staff_first_name,
                s.last_name staff_last_name,
                amount as "amount: i32",
                payment_date
            FROM
                customer c
            INNER JOIN payment p
                USING (customer_id)
            INNER JOIN staff s
                ON p.staff_id = s.staff_id
            ORDER BY payment_date
            LIMIT 10
        "#
    )
    .fetch_all(&pool)
    .await
    .expect("Unable to query staff table..");

    for customer_data in customers_with_payment_details.iter() {
        println!("{:?}", customer_data);
    }

    println!("---Film---");

    let films: Vec<Film> = sqlx::query_as!(
        Film,
        r#"
        SELECT
            film_id, title, description, release_year, language_id, rental_duration, rental_rate as "rental_rate: i32", length, replacement_cost as "replacement_cost: i32", rating as "rating: String", last_update, special_features, fulltext as "fulltext: String"
        FROM film LIMIT 10
        "#
    )
    .fetch_all(&pool)
    .await
    .expect("Unable to query film table..");

    for film in films.iter() {
        println!("{:?}", film);
    }
}