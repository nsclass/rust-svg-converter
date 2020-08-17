# Rust SVG Image Converter ![tests](https://github.com/nsclass/rust-svg-converter/workflows/tests/badge.svg)

The SVG converter written in Rust programming language which is ported from Java version of SVG converter.
[https://github.com/nsclass/ns-svg-converter](https://github.com/nsclass/ns-svg-converter)

# React UI(Frontend)

Frontend application is made of React framework.

## Build

```
$ cd react-ui
$ yarn build
```

## Deployment

HTTP server(Backend) is expecting the build output of React application in the following path. `svg-web-service` will looking for the `index.html` file and other static outputs in the `./react-ui/build` directory.

```
svg-web-service
./react-ui/build
├── asset-manifest.json
├── favicon.ico
├── index.html
├── logo192.png
├── logo512.png
├── manifest.json
├── precache-manifest.ad541c35361b2eb36268f67025f065f2.js
├── robots.txt
├── service-worker.js
└── static
    ├── css
    │   ├── main.677fbce2.chunk.css
    │   └── main.677fbce2.chunk.css.map
    ├── js
    │   ├── 2.6773a66d.chunk.js
    │   ├── 2.6773a66d.chunk.js.LICENSE.txt
    │   ├── 2.6773a66d.chunk.js.map
    │   ├── main.cc1ff848.chunk.js
    │   ├── main.cc1ff848.chunk.js.map
    │   ├── runtime-main.d70f4e48.js
    │   └── runtime-main.d70f4e48.js.map
    └── media
        ├── 2.a90b0f86.jpg
        ├── SVG_logo.bb4f03ed.png
        ├── SVG_logo.e1954262.svg
        ├── header-bg.e2d2a51c.jpg
        ├── spider.6cedddb4.jpg
        └── spider.jpg.65dacd47.svg
```

# HTTP End Points(Backend)

This application is exposing the following HTTP end points.

## Health Checking API

GET `http://localhost:8080/health`

```
{"status": "UP"}
```

## SVG Conversion API

PUT `http://localhost:8080/svg/conversion`

### Payload

IMAGE_DATA should be encoded with base64 as an image media type. The encoded string needs to start with `data:image/jpeg;base64`

- Example File

```
https://github.com/nsclass/rust-svg-converter/blob/master/svg-web-service/tests/bdd/features/samples/image_sample_base64.txt
```

- Payload Example

```
{
  "image_file_name" : "image_sample_base64.jpg",
  "number_of_colors" : 16,
  "image_base64_data": %{IMAGE_DATA}
}
```
