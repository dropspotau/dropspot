### Running the local setup
```
bacon build-frontend
bacon run-server
bacon build-web
```


## Building WebAssembly
Navigate to the core directory
```
cd core
wasm-pack build --target web
cd ../static/dropspot
rm -rf node_modules
pnpm i
pnpm build
```

