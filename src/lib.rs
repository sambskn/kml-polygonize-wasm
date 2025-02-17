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
            //log!("unhandled kml enum: {:?}", kml);
        }
    }
    output
}

fn extract_data_from_geometry(geom: &Geometry) -> Vec<Polygon> {
    let mut polygons = vec![];
    let mut polygon_points = vec![];
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
            polygons.push(Polygon { points });
        }
        Geometry::MultiGeometry(multi_geom) => {
            // MultiGeometry handling strat: if it's polygon, it's polygon, & if it's a buncha linestrings, maybe also polygon?
            // one temp array for building a polygon (e.g. from LineStrings)
            let mut temp_polygon_points = vec![];
            for inner_geom in &multi_geom.geometries {
                match inner_geom {
                    Geometry::LineString(linsestr) => {
                        // add linestring points to temp_polygon_points
                        // this is assuming this is an autocad like export where a polygon gets split into a million linestrings
                        for coord in &linsestr.coords {
                            temp_polygon_points.push(vec![coord.x, coord.y])
                        }
                    }
                    Geometry::Polygon(_polygon) => {
                        // use our main function to grab the polygon data
                        polygons.append(&mut extract_data_from_geometry(inner_geom));
                    }
                    _ => {
                        log!(
                            "yo idk what we do with this inner geom in the multigeometry:  {:?}",
                            inner_geom
                        )
                    }
                }
            }
            // check if we got a polygon from our temp points
            let temp_point_count = temp_polygon_points.len();
            if temp_point_count > 2 {
                // check if last point == first point, as is custom in these parts
                if (temp_polygon_points[temp_point_count - 1][0] != temp_polygon_points[0][0]
                    && temp_polygon_points[temp_point_count - 1][1] != temp_polygon_points[0][1])
                {
                    // add in first point at the end if its not there
                    temp_polygon_points
                        .push(vec![temp_polygon_points[0][0], temp_polygon_points[0][1]]);
                }
                let new_polygon = Polygon {
                    points: vec![temp_polygon_points],
                };
                polygons.push(new_polygon);
            }
        }
        Geometry::LineString(linestr) => {
            for coord in &linestr.coords {
                polygon_points.push(vec![coord.x, coord.y])
            }
        }
        _ => {
            log!("unhandled geometry type: {:?}", geom);
        }
    }
    // check if we need to build any top level polygons from line strings
    // check if we got a polygon from our temp points
    let temp_point_count = polygon_points.len();
    if temp_point_count > 2 {
        // check if last point == first point, as is custom in these parts
        if (polygon_points[temp_point_count - 1][0] != polygon_points[0][0]
            && polygon_points[temp_point_count - 1][1] != polygon_points[0][1])
        {
            // add in first point at the end if its not there
            polygon_points.push(vec![polygon_points[0][0], polygon_points[0][1]]);
        }
        let new_polygon = Polygon {
            points: vec![polygon_points],
        };
        polygons.push(new_polygon);
    }
    // return array of polys
    polygons
}
