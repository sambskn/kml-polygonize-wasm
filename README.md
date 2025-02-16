# kml-polygonize-wasm

A JS package written in Rust & compiled to WASM, with the goal of making it easy to quickly parse out polygons from a given KML file (provided as an ArrayBuffer).

In progress, currently only good for extracting regular polygons, with the goal of expanding to be able to massage polygon-like polylines or points into a polygons.


## Building

`wasm-pack build`

## Usage in JS

1. Import as a package through package.json
2. Import function `polygonizeKml` and pass an object with the KML file as an array buffer in the format:
```js
  {
    kml: new ArrayBuffer(); //replace with actual file content 
  }
```
