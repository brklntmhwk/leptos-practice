# Leptos Practice
This repo walks you through a bunch of both client-side and server-side features Leptos provides, such as signals, resources, and server functions.

## Tips
### sea-orm-cli
- When you want to start database configurations from scratch, executing this command creates migration dir
```
sea-orm-cli migrate init
```

### TailwindCSS standalone CLI
```
tailwindcss -i style/input.css -o style/output.css --watch
```

## Reference
- [SeaORM/docs/next/migration/setting-up-migration/](https://www.sea-ql.org/SeaORM/docs/next/migration/setting-up-migration/)
- [vault81/todo-vault](https://github.com/vault81/todo-vault)

appに足りてないcrateをaddする
とりあえずpagesの中身を揃える必要があるものだけ揃える
→これでうまくいけばtodoのCRUD、DBからの反映ができるようになっているはず。

あとは・・
leptos-useの使用を検討
画面からReactみたいに色々ヌルヌル動かせたりできたら楽しい
