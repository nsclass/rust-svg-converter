# Rust SVG image converter

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

IMAGE_DATA should be encoded with base64 for an image.

```
{
  "image_file_name" : "image_sample_base64.jpg",
  "number_of_colors" : 16,
  "image_base64_data": %{IMAGE_DATA}
}
```
