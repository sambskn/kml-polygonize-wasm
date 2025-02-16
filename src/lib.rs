use kml::{types::Geometry, Kml};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

macro_rules! log { // PLEASE no logs in the final thing, big slowdown
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    };
}

#[derive(Serialize, Deserialize)]
pub struct KMLInput {
    #[serde(with = "serde_bytes")]
    pub kml: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct PolygonizeOutput {
    pub polygons: Vec<Polygon>,
}

#[derive(Serialize, Deserialize)]
pub struct Polygon {
    pub points: Vec<Vec<Vec<f64>>>,
}

#[wasm_bindgen(js_name = polygonizeKML)]
pub fn polygonize_kml(input: JsValue) -> Result<JsValue, JsValue> {
    let kml_input: KMLInput = serde_wasm_bindgen::from_value(input)?;
    let kml_string = String::from_utf8(kml_input.kml).unwrap();
    // log!("Yo heres a str of that kml: {}", kml_string);

    let kml: Kml = kml_string.parse().unwrap();

    let polygons = extract_data_from_kml(&kml);
    let output = PolygonizeOutput { polygons };
    Ok(serde_wasm_bindgen::to_value(&output)?)
}

fn extract_data_from_kml(kml: &Kml) -> Vec<Polygon> {
    let mut output = vec![];
    match kml {
        Kml::KmlDocument(doc) => {
            // loop through entities
            for element in &doc.elements {
                output.append(&mut extract_data_from_kml(&element));
            }
        }
        Kml::Document { attrs: _, elements } => {
            for element in elements {
                output.append(&mut extract_data_from_kml(&element));
            }
        }
        Kml::Folder { attrs: _, elements } => {
            for element in elements {
                output.append(&mut extract_data_from_kml(&element));
            }
        }
        Kml::Placemark(placemark) => match &placemark.geometry {
            Some(geom) => {
                output.append(&mut extract_data_from_geometry(geom));
            }
            None => {
                log!("no content in placemark geoms")
            }
        },
        _ => {
            log!("unhandled kml enum: {:?}", kml);
        }
    }
    output
}

fn extract_data_from_geometry(geom: &Geometry) -> Vec<Polygon> {
    match geom {
        Geometry::Polygon(polygon) => {
            // initialize polygon points vec
            let mut points = vec![];
            // add points from outer polygon ring
            let outer_ring = &polygon.outer;
            let mut first_polygon_ring = vec![];
            for coord in &outer_ring.coords {
                first_polygon_ring.push(vec![coord.x, coord.y]);
            }
            points.push(first_polygon_ring);
            // add inner rings
            let inner_ring_set = &polygon.inner;
            for linear_ring in inner_ring_set {
                let mut point_ring_inner = vec![];
                for coord in &linear_ring.coords {
                    point_ring_inner.push(vec![coord.x, coord.y])
                }
                points.push(point_ring_inner);
            }
            vec![Polygon { points }]
        }
        Geometry::MultiGeometry(multi_geom) => {
            let mut polygons = vec![];
            for inner_geom in &multi_geom.geometries {
                polygons.append(&mut extract_data_from_geometry(inner_geom));
            }
            polygons
        }
        _ => {
            log!("unhandled geometry type: {:?}", geom);
            vec![]
        }
    }
}
