# Leptos Practice
This repo walks you through a bunch of both client-side and server-side features Leptos provides, such as signals, resources, and server functions.

## Tips
### Leptos
#### SSR + Hydration
- In the SSR mode, `cargo-leptos` helps coordinate the build process that requires two separate binaries: one compiled to native code and running the server, and the other compiled to WASM and running in the browser.
  - In my project `leptos-app`, they are `backend` and `frontend`, respectively.
  - It's a build tool that
  - It's not necessary, but eases the complexity of handling multiple binaries at one hand to a greater extent by giving you some special features:
    - Recompiling both the server and client upon changes made
    - Build-in support for TailwindCSS, SASS, and testing
  - I'm a big fan of TailwindCSS and the auto recompiling feature sounds fascinating to me, so I decided to use it.
- In leptos, page loading works in these orders:
  - 1. The browser makes a GET request for the current URL to the server
  - 2. The server receives the request and checks whether it has a way to handle it at that path
    - In `leptos_axum` and `leptos_actix`, such method as `leptos_routes()`, or `leptos_routes_with_handler()` takes route list, decides how to handle it, and generates a list of all possible routes your app can handle
  - 3. The server renders the root of your app (e.g., `<App/>`) with the URL being requested and other data such as the HTTP headers
  - 4. Your app runs once on the server at this timing, building up an HTML version of the component tree that'll be rendered at the current route
  - 5. The server returns this HTML page, injecting info on how to load the version of your app that has been compiled to WASM so it can run in the browser as well
    - At this stage, the HTML page is still at the state of "dehydrated", without any of the reactivity or event listeners added (In other words, it's waiting to be "hydrated")
  - 6. The browser receives this HTML page from the server
  - 7. It immediately goes back to the server to begin loading the JS and WASM necessary to run the interactive, client-side version of your app
  - 8. When the WASM version has reloaded, it does the same route-matching process as the server did at No.2
    - During this initial hydration phase, the WASM version of you app doesn't recreate the DOM nodes forming your app
    - Instead, it walks over the existing HTML DOM tree, picking up some elements and adding the necessary interactivity
    - Conversely, interacive devices such as a counter button cannot work before the WASM has loaded even though the page appears interactive
- Client-side navigation works effectively like this:
  - When the user clicks a link to navigate to another page, the browser won't make another round trip to the server
  - Instead of reloading the full page, the WASM version of your app will load the new page right there in the browser, without requesting it from the server
  - Essentially, your app upgrades itself from a server-loaded "multi-page app" into a browser-rendered "single-page app."
    - This has the best of both worlds: **a fast initial load time thanks to SSR, and fast secondary navigations thanks to the client-side routing**


### sea-orm-cli
- When you want to start database configurations from scratch, execute this command:
```
sea-orm-cli migrate init
```
  - It generates a start-up Rust lib dir `migration` at the working dir, and you can set up migration code there
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
- [Leptos Guide](https://leptos-rs.github.io/leptos/01_introduction.html)
- [Leptos-Use Guide](https://leptos-use.rs/introduction.html)
- [SeaORM/docs/next/migration/setting-up-migration/](https://www.sea-ql.org/SeaORM/docs/next/migration/setting-up-migration/)
- [vault81/todo-vault](https://github.com/vault81/todo-vault)
- [Module sqlx_postgres::types](https://docs.rs/sqlx-postgres/0.7.2/sqlx_postgres/types/index.html)
