# news-extractor

## build

```bash
cargo build
```

## run


### server

```bash
cargo run server
```

### parser

```bash
cargo run parser --url http://example.com --stdin < render.html
# or
cargo run parser --url http://example.com --render-server http://localhost:3000/render
```
