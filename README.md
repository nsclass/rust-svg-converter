# Rust SVG Image Converter ![tests](https://github.com/nsclass/rust-svg-converter/workflows/tests/badge.svg)

The SVG converter written in Rust programming language which is ported from Java version of SVG converter.
[https://github.com/nsclass/ns-svg-converter](https://github.com/nsclass/ns-svg-converter)

# React Framework for UI(Frontend)

Frontend application is made of React framework.

## Build

The following command will build UI application.

```bash
$ cd react-ui
$ yarn build
```

## Deployment

HTTP server(backend) is expecting the build output of React application in `svg-web-service/react-ui/build`.
So we have to copy the `/react-ui/build` directory into target directory manually for debugging.
`svg-web-service` will looking for the `index.html` file and other static outputs from the `./react-ui/build` directory.
However, we don't need to copy files for building a docker image because Dockefile will handle this.


```
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

# Actix-Web Service(Backend)

This application is providing the following HTTP REST end points.

## Health Checking API

GET `http://localhost:8080/health`

```bash
{"status": "UP"}
```

## SVG Conversion API

PUT `http://localhost:8080/svg/conversion`

### Payload

IMAGE_DATA should be encoded with base64 as an image media type. The encoded string needs to start with `data:image/jpeg;base64`

- Example File

```bash
https://github.com/nsclass/rust-svg-converter/blob/master/svg-web-service/tests/bdd/features/samples/image_sample_base64.txt
```

- Payload Example

```bash
{
  "image_file_name" : "image_sample_base64.jpg",
  "number_of_colors" : 16,
  "image_base64_data": %{IMAGE_DATA}
}
```

## Running cucumber BDD tests

This project is integrated with cucumber testing framework.

Before running the following tests, svg-web-service should be launched first and system should have Ruby installed.

```bash
$ cd svg-web-service/tests/bdd
$ bundler exec cucumber feature/*.feature
```

# Docker Deployment

## Build an Image
The following command will build a docker image.
```bash
$ docker build -t [image-name:version] .
```

## Run the Image
You can run an application in a docker with the following command.
```bash
$ docker run -d --rm --name=[name] -p 8080:8080 [image-name]
```
