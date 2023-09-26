# Rental DVD Shop

[PostgreSQL Tutorial](https://www.postgresqltutorial.com/) thankfully demonstrates how to get along with PostgreSQL by giving you a great example of real world database: Rental DVD shop.

This project uses the example database in combination with Rust (mainly SQLx) inside a Docker container. You can get hands-on experience in PostgreSQL, and by extension, handling database relations quickly thanks to the out-of-the-box Docker container. All the steps you need to take beforehand is the followings:

##

## Notes
- In [the original tutorial](https://www.postgresqltutorial.com/postgresql-getting-started/install-postgresql-linux/), How to download the sample db zip file is described as:
```
curl -O https://sp.postgresqltutorial.com/wp-content/uploads/2019/05/dvdrental.zip
```
but this leads you to the error: ```Could not resolve host: sp.postgresqltutorial.com```
To avoid it, execute the curl command below instead:
```
curl -O https://www.postgresqltutorial.com/wp-content/uploads/2019/05/dvdrental.zip
```

```
pg_restore -h localhost -U postgres -d dvdrental ./dvdrental.tar
```

---
- A shell script including the below command is set to ```postCreateCommand``` in ```devcontainer.json``` to enable the non-root user vscode to execute commands like ```cargo build``` or ```cargo run```
  ```
  sudo chown -R vscode /usr/local/cargo/registry
  ```
  - This is necessary because it throws the below error When you set it in Dockerfile instead:
```chown: cannot access no such file or directory```

---
- Whenever you want to check database in CLI, execute the commmand:
  - \* You need to specify which host as ```-h localhost```
```
psql -h localhost -d postgres -U postgres
```

---
- When it comes to the SQLx crate of Rust, it doesn't support such unique types in Postgres as tsvector, year, and mpaa_rating, therefore Rust compiler throws an error when you try to query them.
- On top of that, BigDecimal involves some trait bound problems that keep you from querying data
  - To avoid this, there are two options you can take: Change database or type assertion.

#### Change database
execute these SQL queries as a workaround
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

```
ALTER TABLE film
```

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

## Reference

### Type conversions between Rust and Postgres
- [Module sqlx_postgres::types](https://docs.rs/sqlx-postgres/0.7.1/sqlx_postgres/types/index.html)

### Project's directory structure
- [Full-stack Rust: A complete tutorial with examples](https://blog.logrocket.com/full-stack-rust-a-complete-tutorial-with-examples/)

## Tips

- Whenever a certain field is nullable, you need to wrap its type as: ```Option<T>```
  - [Trait is not implemented for std::string::String](https://stackoverflow.com/questions/76699657/trait-is-not-implemented-for-stdstringstring)

- [Installing PostgreSQL within a docker container](https://stackoverflow.com/questions/23724713/installing-postgresql-within-a-docker-container/52442893#52442893)
- [PostgreSQLへの接続と切断](https://www.javadrive.jp/postgresql/connect/index2.html#section1)
- [【Docker】 Rails + PostgreSQLの開発環境構築時に「psql: could not connect to server: No such file or directory」](https://qiita.com/rebi/items/e9625cedf0d41d1cfa28)

- [Rust cannot represent Postgres numeric type as BigDecimal type](https://stackoverflow.com/questions/76477527/rust-cannot-represent-postgres-numeric-type-as-bigdecimal-type)

Elm をフロントエンドに使うか、ReactやElmに影響を受けたフロントエンドﾌﾚｰﾑﾜｰｸYewを使うか、要検討
RustとElmで囲碁対戦サービスを作る(サーバにAxum使ってる)
https://zenn.dev/tkzwhr/books/go-game-service
→Elm魅力的だけど、これはこれでクローンして別の機会にいじろうかな。。ということでYew??
このレポジトリ参考になりそう
https://github.com/JoeyMckenzie/realworld-rust-axum-sqlx/tree/main

Yew or leptos
https://www.reddit.com/r/rust/comments/1526qo3/yew_or_leptos/
leptosパフォーマンス面とかバイナリサイズとかでYewに勝ってるみたい

型互換性がないいくつかのカラムについては今回は諦めて削除しようと思う。そうしないと先へ進めないし。
SQLxととりあえず繋がったから、今度はAxum とフロントエンドフレームワークの組み合わせでブラウザに表示する。データベースとのやり取りをAPI化するなど。

なんだか知らんが、Leptos + Axum + SQLx + SQLite + TailwindCSSのレポ発見！
https://github.com/benwis/leptos-heavy-metal-stack
Leptosにはサーバーファンクションなるものがあるらしく、そこからSQLxを呼んでいるとのこと
