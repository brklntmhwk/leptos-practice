# Leptos Practice
This repo walks you through a bunch of both client-side and server-side features Leptos provides, such as signals, resources, and server functions.

## Tips
### sea-orm-cli
- When you want to start database configurations from scratch, execute this command:
```
sea-orm-cli migrate init
```
  - It generates a start-up Rust lib dir ```migration``` at the working dir, and you can set up migration code there
  - Refer to [SeaORM/docs/next/migration/setting-up-migration/](https://www.sea-ql.org/SeaORM/docs/next/migration/setting-up-migration/)

- After you've done setting it up, execute this command:
```
sea-orm-cli generate entity -u postgres://postgres:postgres@localhost:5432/postgres -o entity/src
```
  - It generates entity files based on your migration settings

### TailwindCSS standalone CLI
- Reflect the styles in `style/input.css` on `style/output.css` by executing this command:
```
tailwindcss -i style/input.css -o style/output.css --watch
```
  - In the `--watch` mode, changes will be reflected as soon as you've done so in `style/input.css`

## Reference
- [SeaORM/docs/next/migration/setting-up-migration/](https://www.sea-ql.org/SeaORM/docs/next/migration/setting-up-migration/)
- [vault81/todo-vault](https://github.com/vault81/todo-vault)
- [Module sqlx_postgres::types](https://docs.rs/sqlx-postgres/0.7.2/sqlx_postgres/types/index.html)
