# red-siren

## Development
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -c ./red-siren-lib/tailwind.config.js -i ./red-siren-lib/input.css -o ./public/tailwind.css --watch
```

Run the following command in the root of the project to start the Dioxus dev server:

```bash
cd red-siren-web
dx build --features web
dx serve --features ssr --hot-reload --platform desktop
```

```bash
cargo watch -s "dx build --features web 
dx serve --features ssr --platform desktop --bin red-siren-web"
```

- Open the browser to http://localhost:8080

