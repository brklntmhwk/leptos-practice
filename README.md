# Leptos Practice
This repo walks you through a bunch of both client-side and server-side features Leptos provides, such as signals, resources, and server functions.

## Tips
These are some notes of what I've learnt through tinkering with this app.
### Leptos
- Leptos is a web frontend framework that is reactive primitive with signals like SolidJS
- Instead of using virtual DOM, it works with real DOM nodes and can observe and detect partial changes made inside a structure by signal
- This makes rendering to occur as less frequently as possible
#### SSR + Hydration
- In the SSR mode, `cargo-leptos` helps coordinate the build process that requires two separate binaries: one compiled to native code and running the server, and the other compiled to WASM and running in the browser.
  - In my project `leptos-app`, they are `backend` and `frontend`, respectively.
  - It's a build tool that enables parallel building of server and client in watch mode
    - Execute this command to start watching: `cargo leptos watch`
  - It's not necessary, but eases the complexity of handling multiple binaries at one hand to a greater extent by giving you some special features:
    - Recompiling both the server and client upon changes made
    - Build-in support for TailwindCSS, SASS, and testing
    - Optimization of the WASM with wasm-opt
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
#### Reactivity
- `create_signal` is similar to `useState` in React, and almost equivalent to `createSignal` in SolidJS
- `create_effect` is almost equivalent to `useEffect` in React, but there's a difference;
  - It doesn't take that dependency array
    - Leptos automatically tracks deps knowing which signals are accessed within the effect so you don't have to explictly add it
      - For example, if `use_last` is `true`, the effect will rerun whenever `first`, `last`, or `use_last`changes in this case
      - but if not, only will `first` be the tracking target
      ```rs
      let (first, set_first) = create_signal(String::new());
      let (last, set_last) = create_signal(String::new());
      let (use_last, set_use_last) = create_signal(true);

      create_effect(move |_| {
          log(
              if use_last.get() {
                  format!("{} {}", first.get(), last.get())
              } else {
                  first.get()
              },
          )
      });
      ```
#### Async
- `create_resource` allows you to integrate async into the sync reactive system
  - The word "resource" points to reactive data structure in this context
  - It lets the `Future` into a signal that returns `Some(T)` if it's resolved, and `None` if it's still pending, rather than waiting for its data to be loaded with `.async`
  - Just like `useSWR` or `useSWRInfinite` in swr, a React hooks library, it takes a source signal as its first arg, and a fetcher as its second one
  ```rs
  let (count, set_count) = create_signal(0);

  let async_data = create_resource(
      count, // a source signal
      |value| async move {
          logging::log!("loading data from API");
          load_data(value).await
      }, // a fetcher (every time `count` changes, this will run)
  );
  ```
  - To create a resource that runs only once, you can pass a non-reactive, empty source signal:
  ```rs
  let once = create_resource(|| (), |_| async move { load_data().await });
  ```
- Compared to `create_resource`, `create_action` is more suitable for a situation where an async function needs to run in response to a user's action such as clicking a button
  - e.g., `create_action` for `add_todo`, `create_resource` for `list_todos`
- Unlike `create_resource`, `create_local_resource` handles local resources that don't load on the server but in the browser
  - Since the `Future` that a fetcher will generate runs only on the local system, its data type doesn't need to be serializable
- Both `<Suspense>` and `<Transition>` take resource elements as their children, observing the status of them loading
  - If resources are still loading, the fallback component is displayed instead
- The difference between `<Suspense>` and `<Transition>` is the former one causes flickering every time the data reloaded whereas the latter one only shows the fallback the first time
- `<Async>` is the no fallback version of `<Suspense>` that helps omit a bit of boilerplates

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
  - In the `--watch` mode, changes will be reflected soon after you've made in `style/input.css`

## Reference
- [Leptos Guide](https://leptos-rs.github.io/leptos/01_introduction.html)
- [Leptos-Use Guide](https://leptos-use.rs/introduction.html)
- [SeaORM/docs/next/migration/setting-up-migration/](https://www.sea-ql.org/SeaORM/docs/next/migration/setting-up-migration/)
- [vault81/todo-vault](https://github.com/vault81/todo-vault)
- [Module sqlx_postgres::types](https://docs.rs/sqlx-postgres/0.7.2/sqlx_postgres/types/index.html)
- [JSON Placeholder](https://jsonplaceholder.typicode.com/)
- [Material Tailwind Tabs](https://www.material-tailwind.com/docs/html/tabs)
- [How to generate a vec of enum options in Rust [duplicate]](https://stackoverflow.com/questions/68427115/how-to-generate-a-vec-of-enum-options-in-rust)
