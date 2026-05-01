# Plan 1 — Baseline Architecture

> **Status:** Retroactive plan paired with `codev/specs/1-baseline-architecture.md`. There is nothing to implement — the system already exists. This document records HOW the code is laid out so future SPIR/AIR/BUGFIX work has an accurate map of the moving parts.

## Workspace Layout

```
ns-svg-converter/
├── Cargo.toml                  # workspace root: members = [svg-web-service, svg-converter]
├── Dockerfile                  # multi-stage: node-builder + cargo --release → ubuntu runtime
├── docker-env / .env           # HOST, PORT, RUST_LOG
├── svg-converter/              # core library crate
├── svg-web-service/            # Actix-Web binary crate (lib + bin)
├── react-ui/                   # React 18 + Vite + TS frontend
├── codev/                      # specs / plans / reviews / projects
└── .github/workflows/          # CI
```

## Crate: `svg-converter`

Pure pipeline library. No I/O, no async, no web concerns.

```
svg-converter/src/
├── lib.rs                      # re-exports {domain, operation, utils, svg_converter_utils}
├── svg_converter_utils.rs      # public entry: svg_converted_str_from_base64_image
├── domain/                     # data types threaded through the pipeline
│   ├── error.rs                # thiserror enum (Error::*)
│   ├── image_data.rs           # ImageData (raw u8) + ImageColorData (Vec<exoquant::Color>)
│   ├── image_array.rs
│   ├── image_convert_options.rs # ImageConvertOptions + Default (16 colors, etc.)
│   ├── image_interpolation.rs
│   ├── image_path_trace.rs
│   ├── image_paths.rs
│   └── svg_conversion_ctx.rs   # SvgConversionCtx enum — state machine variants
├── operation/                  # one file per pipeline stage
│   ├── generate_palette_quantization.rs
│   ├── generate_layer_edge_detection.rs
│   ├── generate_paths.rs       # scan paths
│   ├── batch_interpolation.rs
│   ├── image_path_tracing.rs
│   └── generate_svg_string.rs
└── utils/
    ├── operation_manager.rs    # OperationManager<T>: ordered Vec<Box<dyn Fn(T)->Result<T,Error>>>
    └── operation_progress_listener.rs # OperationProgressListener trait
```

### Key types & flow

- **`SvgConversionCtx`** is the carrier — each pipeline stage matches the variant it expects and produces the next variant. Sequence: `Base64Image → ImageData → ColorQuantization → Layers → ScanPaths → BatchInterpolation → ImagePathTraceLayers → SvgString`.
- **`OperationManager<T>`** owns a `Vec<OperationItem<T>>` and runs them in order in `execute(ctx)`, calling `on_progress(name, idx, total)` after each step. Stages are added with closures, so adding a stage is one `add_operation` call in `svg_converter_utils.rs`.
- **`Error`** is a `thiserror` enum covering each stage's failure mode plus `ImageError(image::error::ImageError)` for decode failures.

### Notable dependencies

- `image = "0.25"` — raster decode.
- `exoquant = "0.2"` — KMeans palette + Floyd-Steinberg dither.
- `ndarray`, `rayon`, `rand` — used inside pipeline stages.
- `rustc-serialize` — base64 (legacy; could be replaced with `base64` crate in future maintenance).

## Crate: `svg-web-service`

```
svg-web-service/src/
├── main.rs                     # actix_web::main → Config::from_env → app_run
├── lib.rs                      # app_run(Config) → HttpServer::new(...).bind().run()
├── config/mod.rs               # Config { host, port } + tracing-bunyan subscriber init
└── web_handler/
    ├── mod.rs                  # app_config: routes + actix_files::Files mount
    ├── health.rs               # GET  /health      → {"status":"UP"}
    ├── svg_conversion.rs       # PUT  /svg/conversion (streams payload → JSON parse → pipeline)
    └── index.rs                # GET  /            → react-ui/build/index.html
```

### Routing

```rust
// web_handler/mod.rs
config
  .route("/svg/conversion", web::put().to(svg_convert))
  .route("/health",         web::get().to(health))
  .route("/",               web::get().to(single_page_app))
  .service(Files::new("/", "./react-ui/build").index_file("index.html"));
```

Order matters: explicit routes match before `actix_files::Files` falls through to static assets.

### Request handling

`svg_convert` reads the streamed `web::Payload` into a `BytesMut`, deserializes `SvgConvertRequest`, calls `svg_converted_str_from_base64_image`, and returns either `200 SvgConvertResponse` or `400` with the error message. There is **no payload size cap** (the `MAX_SIZE` check is commented out) — flagged in the spec as a known limit, not a bug to fix in this baseline.

### Tests

- `tests/health_check_test.rs` — spawns `app_run` on a tokio task and curls `/health`.
- `tests/bdd/` — Cucumber/Ruby features; requires server running.

## Frontend: `react-ui`

```
react-ui/
├── package.json                # vite, react 18, react-dropzone, axios
├── vite.config.ts              # @vitejs/plugin-react-swc, vitest, /svg dev proxy
├── tsconfig.json
├── index.html                  # Vite entry
└── src/
    ├── index.tsx               # createRoot → <App/>
    ├── App.tsx                 # <ErrorBoundary><MainNavBar/><MainView/></ErrorBoundary>
    ├── component/
    │   ├── MainNavBar.tsx
    │   ├── MainView.tsx        # composes drop zone + converter + samples + history
    │   ├── SVGConverter.tsx    # axios.put('/svg/conversion'), spinner, download
    │   ├── SVGConvertExamples.tsx
    │   ├── SVGHistory.tsx
    │   └── ErrorBoundary.tsx
    ├── hooks/
    │   └── useImageDropZone.tsx # react-dropzone + FileReader.readAsDataURL
    ├── assets/{css,images,public}/
    └── __tests__/{App,ImageDropZone}.test.tsx
```

### Build & dev

- `yarn start` → Vite dev server (proxies `/svg` to `localhost:8080`).
- `yarn build` → `tsc && vite build`, output goes to `react-ui/build/` (legacy CRA path retained because the Rust server hard-codes `./react-ui/build` in two places: `web_handler/mod.rs` and `web_handler/index.rs`).
- `yarn test` → Vitest with jsdom.

### Frontend → backend contract

`SVGConverter.tsx` is the only place that talks to the backend:

```ts
axios.put("/svg/conversion", {
  image_file_name: filename,
  image_base64_data: dataUrl,   // includes "data:image/...;base64," prefix
  number_of_colors: 16,
})
```

Response shape: `{image_file_name, svg_string}`. Errors are surfaced as Bootstrap alerts.

## Build & Deploy Pipeline

### Local

1. `cd react-ui && yarn install && yarn build`  — produces `react-ui/build/`.
2. `cargo run -p svg-web-service` — serves API + UI on `http://localhost:8080`.

### Docker (production)

`Dockerfile`:
1. **Builder stage** (`node:12-stretch`) — installs Rust toolchain, runs `yarn build` in `react-ui/`, then `cargo build --release` at the workspace root.
2. **Runtime stage** (`ubuntu`) — non-root `svg` user, copies `target/release/svg-web-service`, `react-ui/build/`, and `docker-env` (as `.env`), exposes `8080`.

> Builder image is `node:12-stretch` (Debian 9, EOL). A future maintenance task should bump this to a current LTS Node image.

## Configuration & Environment

| Source | Loaded by | Purpose |
|--------|-----------|---------|
| `.env` | `dotenv` in `Config::from_env` | Local dev defaults (`HOST=localhost`) |
| `docker-env` | Copied to `.env` in the container | Container defaults (`HOST=0.0.0.0`) |
| Process env | `config::Environment::default()` | Overrides `HOST`, `PORT` |
| `RUST_LOG` | `tracing_subscriber::EnvFilter` | Log level (defaults to `info` if unset) |

## Observability

- **Logs:** `tracing` + `tracing-bunyan-formatter` (JSON to stdout) + `tracing-actix-web::TracingLogger` (per-request span).
- **Pipeline progress:** `OperationProgressListener` impl in `svg_conversion.rs` logs each stage as `desc, cur/total` at `info`.
- **Metrics / tracing exporter:** none.

## Known Constraints & Future-Work Hooks

These are observations from the current code, not action items. They exist so future specs can reference them by name.

1. **`number_of_colors` is unused** — `svg_conversion.rs` constructs the pipeline with `ImageConvertOptions::default()`; the request field is parsed but never read. (Future spec: thread it through.)
2. **No payload size limit** — the `MAX_SIZE` guard is commented out in `svg_convert`. Large uploads can OOM the process.
3. **Hard-coded static path** — `./react-ui/build` appears in both `web_handler/mod.rs` and `web_handler/index.rs`. Any path change must update both.
4. **`rustc-serialize` is deprecated** — used only for base64 in `svg_converter_utils::from_base64`.
5. **Dockerfile uses EOL Node 12 base.**
6. **Dependency churn:** much of the recent git history is dependabot bumps to `react-ui` (webpack, ws, braces, ejs) — those crates are no longer in the dependency graph after the Vite migration; only `vite`/`vitest` matter going forward.

## Acceptance (for the baseline)

This plan is "accepted" if a reader unfamiliar with the repo can:

- Trace a request from `SVGConverter.tsx` → `svg_convert` handler → pipeline stages → response.
- Find every place the SVG pipeline reads configuration.
- Locate the entry point for any of: HTTP routes, pipeline stages, frontend components, build output paths, env vars, tests.

If any of those are unclear, the plan is incomplete and should be amended in place.
