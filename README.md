# Rust SVG Image Converter ![tests](https://github.com/nsclass/rust-svg-converter/workflows/tests/badge.svg)

The SVG converter written in Rust programming language which is ported from Java version of SVG converter.
[https://github.com/nsclass/ns-svg-converter](https://github.com/nsclass/ns-svg-converter)

# HTTP End Points

This application is exposing the following HTTP end points

## Health checking API

GET `http://localhost:8080/health`

```
{"status": "UP"}
```

## SVG conversion API

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
