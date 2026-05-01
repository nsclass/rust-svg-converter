# Spec 1 — Baseline Architecture

> **Status:** Retroactive baseline. Documents the system as it exists at commit `499bf16` (post Vite/TypeScript migration). No new behavior is proposed here; this spec records WHAT the system does today so future codev work has a shared reference.

## Goal

`ns-svg-converter` is a web application that converts raster images (PNG, JPEG) to SVG vector paths. A user drops an image into the browser UI, the backend traces it through a multi-stage pipeline, and the resulting SVG is returned for preview and download.

## Users & Use Cases

- **End user (browser):** drag-and-drop a raster image, click *Convert*, preview the SVG, click *Download* to save the `.svg` file. Also browses sample conversions and SVG history content on the same page.
- **HTTP client (programmatic):** PUTs base64-encoded image data to `/svg/conversion` and receives the SVG string in JSON. Used by the BDD test suite and any external integrators.

## High-Level Components

The repo is one Rust workspace plus a separate React UI:

| Component | Path | Responsibility |
|-----------|------|----------------|
| Core converter | `svg-converter/` | Pure Rust library implementing the raster→SVG pipeline |
| Web service | `svg-web-service/` | Actix-Web HTTP server: REST API + static file hosting |
| Frontend | `react-ui/` | React 18 SPA built with Vite + TypeScript |
| Container | `Dockerfile` | Multi-stage build that compiles Rust release binary + React bundle into a single Ubuntu image |

The web service serves the compiled React bundle out of `./react-ui/build` at `/`, so the Docker image is the only deployable artifact in production.

## HTTP Surface

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/health` | Liveness probe — returns `{"status":"UP"}` |
| `PUT` | `/svg/conversion` | Convert a base64-encoded image to SVG (see payload below) |
| `GET` | `/` | Serves `react-ui/build/index.html` (SPA entry) |
| `GET` | `/*` | `actix-files` static handler for `react-ui/build/` |

### `PUT /svg/conversion` contract

**Request:**
```json
{
  "image_file_name": "spider.jpg",
  "image_base64_data": "data:image/jpeg;base64,...",
  "number_of_colors": 16
}
```

**Response (200):**
```json
{
  "image_file_name": "spider.jpg",
  "svg_string": "<svg ...>...</svg>"
}
```

**Errors:** `400 Bad Request` with the `svg_converter::Error` message (e.g. `not valid base64 image string`, `failed to convert an image`).

> ⚠️ Known gap: the `number_of_colors` field is part of the request schema but the handler currently ignores it — the converter always uses `ImageConvertOptions::default()` (16 colors). Documented here so it isn't mistaken for a missing feature.

## Conversion Pipeline (Core Library)

`svg_converter::svg_converted_str_from_base64_image` runs seven sequential stages via `OperationManager`, threading state through the `SvgConversionCtx` enum:

1. **Decode** — strip the `data:image/...;base64,` prefix, base64-decode, hand to the `image` crate, produce an RGBA `ImageColorData`.
2. **Palette + color quantization** — `exoquant` KMeans optimizer with Floyd-Steinberg dithering, default 16 colors, max 256.
3. **Layer + edge detection** — split the indexed image into per-color layers and detect edges.
4. **Scan paths** — walk edges into ordered path point lists.
5. **Batch interpolation** — smooth path points.
6. **Image path tracing** — fit linear/quadratic curves to each path.
7. **SVG string generation** — emit the final `<svg>` document.

Each stage reports progress through a caller-supplied `OperationProgressListener` (the web handler logs `desc, idx/total` via `tracing`).

## Frontend Behavior

- **Entry:** `react-ui/src/index.tsx` mounts `<App>` into `#root`.
- **Layout:** `App → ErrorBoundary → (MainNavBar, MainView)`.
- **`MainView`** composes three sections on one page:
  - `SVGConverterView` — title, drop zone, and the converter.
  - `SVGConvertExamples` — two static before/after sample pairs.
  - `SVGHistory` — Wikipedia-sourced timeline of the SVG standard.
- **Drop zone** (`useImageDropZone`, `react-dropzone`) reads the file as a base64 data URL and stores `(filename, fileContent)` in component state.
- **`SVGConverter`** posts `{image_file_name, image_base64_data, number_of_colors: 16}` to `/svg/conversion` via `axios.put`, shows a spinner while waiting, then renders the returned SVG (decoded as a `data:image/svg+xml;base64,...` `<img>` source) with a *Download* button that creates a Blob and triggers an anchor click.
- **Error handling:** an axios error message is split on `:` and displayed in a Bootstrap alert; a top-level `ErrorBoundary` catches render errors and shows a reload prompt.
- **Dev server:** Vite proxies `/svg` → `http://localhost:8080` (`vite.config.ts`), so the frontend can run standalone against a local backend.

## Configuration

`svg-web-service` reads two environment variables (via `dotenv` + `config` crate):

| Var | Default | Notes |
|-----|---------|-------|
| `HOST` | `localhost` | Bound interface |
| `PORT` | `8080` | Bound port |
| `RUST_LOG` | — | Standard `tracing-subscriber` env filter |

Local dev uses `.env` (`HOST=localhost`); the container uses `docker-env` (`HOST=0.0.0.0`, copied to `.env` inside the image).

Logging is structured JSON via `tracing-bunyan-formatter` and `tracing-actix-web`.

## Test Surface

- **Rust unit tests** — colocated `#[cfg(test)]` modules in `svg-converter` (e.g. `image_data`, `generate_palette_quantization`).
- **Rust integration test** — `svg-web-service/tests/health_check_test.rs` boots the server in-process and hits `/health`.
- **End-to-end pipeline test** — `svg-converter/src/svg_converter_utils.rs::tests::svg_conversion_success` runs the full pipeline against a fixture base64 image (`tests/bdd/features/samples/image_sample_base64.txt`) and asserts a non-empty SVG.
- **Cucumber BDD** — Ruby-based features in `svg-web-service/tests/bdd/features/` (`testcase_application_status.feature`, `testcase_svg_image_convert.feature`); requires the server to be running and Ruby + bundler installed.
- **Frontend tests** — Vitest + Testing Library in `react-ui/src/__tests__/` (`App.test.tsx`, `ImageDropZone.test.tsx`).
- **CI** — single GitHub Actions workflow at `.github/workflows/svg_convert_rust.yml`.

## Deployment

A multi-stage Dockerfile builds the React bundle (`yarn build` in `react-ui/`), then compiles the Rust release binary, then copies both into a runtime `ubuntu` image as a non-root `svg` user. The runtime container exposes `8080` and runs `./svg-web-service`, which serves both the API and the static UI.

## Non-Goals (for this baseline)

- No authentication, rate limiting, or per-user state.
- No persistence — every conversion is stateless and request-scoped.
- No exposure of `ImageConvertOptions` knobs beyond the (currently ignored) `number_of_colors` field.
- No streaming or chunked conversion — full payload is buffered in memory.
- No CDN / asset hashing concerns beyond Vite's defaults.

## Glossary

- **Color quantization** — reducing a true-color image to a fixed palette (here: KMeans + Floyd-Steinberg via `exoquant`).
- **Layer** — the binary mask of pixels assigned to a single palette color.
- **Scan path** — the ordered sequence of edge points around a layer region.
- **Path tracing** — fitting line and quadratic Bézier segments to a scan path to emit SVG `<path d="...">`.

## References

- `README.md` — user-facing usage & build instructions.
- `Cargo.toml` (workspace), `svg-converter/Cargo.toml`, `svg-web-service/Cargo.toml`.
- `react-ui/package.json`, `react-ui/vite.config.ts`.
- `Dockerfile`, `.env`, `docker-env`.
