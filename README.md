# kml-polygonize-wasm

A JS package written in Rust & compiled to WASM, with the goal of making it easy to quickly parse out polygons from a given KML file (provided as an ArrayBuffer).

In progress, currently only good for extracting regular polygons, with the goal of expanding to be able to massage polygon-like polylines or points into a polygons.

## Building
`wasm-pack build` baby that's all - that makes a npm package in the `pkg` folder

## Usage in JS
See example in the `www` folder
