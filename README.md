# Rental DVD Shop

![Rental DVD shop database diagram](screenshots/dvd-rental-sample-database-diagram.png)

[PostgreSQL Tutorial](https://www.postgresqltutorial.com/) thankfully demonstrates how to get along with PostgreSQL by giving you a great example of real world database: Rental DVD shop.

This project uses the example database in combination with Rust (mainly SeaORM(SQLx) and Axum) inside a Docker container. You can get hands-on experience in PostgreSQL, and by extension, handling database relations quickly thanks to the out-of-the-box Docker container.

## Prerequisites

- You have installed:

  - [Docker Desktop](https://www.docker.com/products/docker-desktop/)
  - [VS Code](https://code.visualstudio.com/)

on your local machine.

- You have added the following VS Code extensions:
  - [Dev Containers](https://code.visualstudio.com/docs/devcontainers/tutorial) ([Remote Development](https://code.visualstudio.com/docs/remote/remote-overview) for Windows users instead)

## How to use

All steps you need to take beforehand are the followings:



## Notes
### Specify which host to look up for database
In [the original tutorial](https://www.postgresqltutorial.com/postgresql-getting-started/install-postgresql-linux/), you need to modify some commands: Specify which host to look up for database. These are the list of it.
-  How to download the sample db zip file is described as:
```
curl -O https://sp.postgresqltutorial.com/wp-content/uploads/2019/05/dvdrental.zip
```
but this leads you to the error: ```Could not resolve host: sp.postgresqltutorial.com```
To avoid it, execute the curl command below instead:
```
curl -O https://www.postgresqltutorial.com/wp-content/uploads/2019/05/dvdrental.zip
```
- How to restore the database out of the tar archive is described as:
```
pg_restore --dbname=dvdrental --verbose dvdrental.tar
```
but this leads you to the error: ```could not connect to server: No such file or directory...```
To avoid it, execute the command below instead:
```
pg_restore -h localhost -U postgres -d dvdrental ./dvdrental.tar
```
- How to check database in CLI is described as:
```
psql
```
but this leads you to the error: ```could not connect to server: No such file or directory...```
To avoid it, execute the command below instead:
\* If you execute this as postgres, you can omit ```-U postgres```
```
psql -h localhost -d postgres -U postgres
```
---
### Some unique types in Postgres aren't supported in SQLx
- When it comes to the SQLx crate of Rust, it doesn't support such unique types in Postgres as tsvector, year, and mpaa_rating, therefore Rust compiler throws an error when you try to query them.
- On top of that, BigDecimal involves some trait bound problems that keep you from querying data
  - To avoid this, there are two options you can take: Change database or type assertion.
```
the trait bound `bigdecimal::BigDecimal: From<BigDecimal>` is not satisfied
the following other types implement trait `From<T>`:
  <bigdecimal::BigDecimal as From<&i128>>
  <bigdecimal::BigDecimal as From<&i16>>
  <bigdecimal::BigDecimal as From<&i32>>
  <bigdecimal::BigDecimal as From<&i64>>
  <bigdecimal::BigDecimal as From<&i8>>
  <bigdecimal::BigDecimal as From<&u128>>
  <bigdecimal::BigDecimal as From<&u16>>
  <bigdecimal::BigDecimal as From<&u32>>
and 14 others
required for `BigDecimal` to implement `Into<bigdecimal::BigDecimal>`
```

#### Change database
Enter into the database with the psql command, and execute these SQL queries in your terminal.
```
DROP VIEW film_list, nicer_but_slower_film_list;
```
and then,
```
ALTER TABLE film
ALTER COLUMN release_year TYPE INT,
ALTER COLUMN rating TYPE VARCHAR(8),
ALTER COLUMN fulltext TYPE VARCHAR(255);
```
📝\* The first one deletes views (like rules) predefined so that the second one works without an error. The second one changes the incompatible types in Postgres into those compatible.

## Reference

### Type conversions between Rust and Postgres
- [Module sqlx_postgres::types](https://docs.rs/sqlx-postgres/0.7.1/sqlx_postgres/types/index.html)

### Project's directory structure
- [vault81/todo-vault](https://github.com/vault81/todo-vault)
- [Full-stack Rust: A complete tutorial with examples](https://blog.logrocket.com/full-stack-rust-a-complete-tutorial-with-examples/)
- [leptos-rs/cargo-leptos](https://github.com/leptos-rs/cargo-leptos)

### Docker image
- [cargo-chef](https://github.com/LukeMathWalker/cargo-chef)
- [clux/muslrust](https://github.com/clux/muslrust)

### Rust in general
- [RustのフロントエンドフレームワークLeptosでWASM+SSR/Hydrationするアプリをつくる](https://nulab.com/ja/blog/nulab/rust-leptos-ssr-hydration/)

### Error fix
- [【Docker】 Rails + PostgreSQLの開発環境構築時に「psql: could not connect to server: No such file or directory」](https://qiita.com/rebi/items/e9625cedf0d41d1cfa28)
- [Rust cannot represent Postgres numeric type as BigDecimal type](https://stackoverflow.com/questions/76477527/rust-cannot-represent-postgres-numeric-type-as-bigdecimal-type)
- [PostgreSQLへの接続と切断](https://www.javadrive.jp/postgresql/connect/index2.html#section1)
- [Installing PostgreSQL within a docker container](https://stackoverflow.com/questions/23724713/installing-postgresql-within-a-docker-container/52442893#52442893)
- [Linux上でのCargoビルドで`Could not find directory of OpenSSL installation`と言われたときの対処法
](https://qiita.com/yoshrc/items/8464f3b5eb346cc91d7b)
- [Rustで”HelloWorld!”しようとしたら「error: linker `cc` not found」と表示された時の対処方法
](https://qiita.com/ismt7/items/a6b17b01b56f098b2dd5)
- [open ssl 不在によるcargo packageインストールエラー メモ](https://murakamipeipei.com/2022/12/31/open-ssl-%E4%B8%8D%E5%9C%A8%E3%81%AB%E3%82%88%E3%82%8Bcargo-package%E3%82%A4%E3%83%B3%E3%82%B9%E3%83%88%E3%83%BC%E3%83%AB%E3%82%A8%E3%83%A9%E3%83%BC-%E3%83%A1%E3%83%A2/)
- [Google Git chromium/chrmiumos/platform/crosvm](https://chromium.googlesource.com/chromiumos/platform/crosvm/+/7a2592a2cb70d74cc9e71f3dd2329e6379c9e6c1/kokoro/Dockerfile)
- [Warning: some crates are on edition 2021 which defaults to resolver = "2"](https://substrate.stackexchange.com/questions/9011/warning-some-crates-are-on-edition-2021-which-defaults-to-resolver-2)

## Tips

- Whenever a certain field of a certain table is nullable, you need to make it optional as: ```Option<T>```
  - [Trait is not implemented for std::string::String](https://stackoverflow.com/questions/76699657/trait-is-not-implemented-for-stdstringstring)
- SeaORM offers a great CLI tool sea-orm-cli, with which automatic generation of entities out of existing database relations is possible in a heartbeat. You only need to execute this:
```
sea-orm-cli generate entity -u postgres://postgres:postgres@localhost:5432/dvdrental -o entity/src
```


## Backlog

- [x] add comments to all line blocks in Dockerfile and tweak some existing ones
- [] introduce TailwindCSS and arrage the ecosystem
- [x] introduce SeaORM
