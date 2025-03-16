use japan_mesh_rs::{Coordinates, JPMeshCode, JPMeshType, Rect};
use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct GeoJsonFeatureCollection {
    #[serde(rename = "type")]
    feature_type: String,
    features: Vec<GeoJsonFeature>,
}

#[derive(Serialize)]
struct GeoJsonFeature {
    #[serde(rename = "type")]
    feature_type: String,
    properties: GeoJsonProperties,
    geometry: GeoJsonGeometry,
}

#[derive(Serialize)]
struct GeoJsonProperties {
    mesh_code: u64,
}

#[derive(Serialize)]
struct GeoJsonGeometry {
    #[serde(rename = "type")]
    geometry_type: String,
    coordinates: Vec<Vec<Vec<f64>>>,
}

fn rect_to_polygon(rect: Rect) -> Vec<Vec<Vec<f64>>> {
    let min = rect.min();
    let max = rect.max();
    
    // GeoJSONのポリゴン座標は[経度, 緯度]の順で、反時計回りに定義
    // 最初と最後の座標は同じである必要がある
    vec![vec![
        vec![min.lng, min.lat],
        vec![max.lng, min.lat],
        vec![max.lng, max.lat],
        vec![min.lng, max.lat],
        vec![min.lng, min.lat],
    ]]
}

fn create_geojson_for_mesh_type(bounds: Rect, mesh_type: JPMeshType) -> GeoJsonFeatureCollection {
    // 指定された範囲内のメッシュコードを取得
    let mesh_codes = JPMeshCode::from_on_bounds(bounds, mesh_type);
    
    // 各メッシュコードをGeoJSONのフィーチャーに変換
    let features = mesh_codes
        .into_iter()
        .map(|mesh_code| {
            let bounds = mesh_code.to_bounds();
            let mesh_code_number = mesh_code.to_number();
            
            GeoJsonFeature {
                feature_type: "Feature".to_string(),
                properties: GeoJsonProperties {
                    mesh_code: mesh_code_number,
                },
                geometry: GeoJsonGeometry {
                    geometry_type: "Polygon".to_string(),
                    coordinates: rect_to_polygon(bounds),
                },
            }
        })
        .collect();
    
    GeoJsonFeatureCollection {
        feature_type: "FeatureCollection".to_string(),
        features,
    }
}

fn save_geojson(geojson: &GeoJsonFeatureCollection, filename: &str) -> std::io::Result<()> {
    let json_string = serde_json::to_string_pretty(geojson)?;
    let mut file = File::create(filename)?;
    file.write_all(json_string.as_bytes())?;
    println!("GeoJSONファイルを保存しました: {}", filename);
    Ok(())
}

fn main() -> std::io::Result<()> {
    // 各メッシュタイプと範囲の組み合わせを定義
    let mesh_configs = vec![
        (
            "out/mesh80km.geojson",
            Rect::new(
                Coordinates::new(138.0, 39.0),
                Coordinates::new(144.0, 45.0),
            ),
            JPMeshType::Mesh80km,
        ),
        (
            "out/mesh10km.geojson",
            Rect::new(
                Coordinates::new(140.0, 41.0),
                Coordinates::new(142.0, 43.0),
            ),
            JPMeshType::Mesh10km,
        ),
        (
            "out/mesh1km.geojson",
            Rect::new(
                Coordinates::new(140.55, 41.60),
                Coordinates::new(140.90, 41.95),
            ),
            JPMeshType::Mesh1km,
        ),
        (
            "out/mesh500m.geojson",
            Rect::new(
                Coordinates::new(140.65, 41.70),
                Coordinates::new(140.80, 41.85),
            ),
            JPMeshType::Mesh500m,
        ),
        (
            "out/mesh250m.geojson",
            Rect::new(
                Coordinates::new(140.70, 41.75),
                Coordinates::new(140.75, 41.80),
            ),
            JPMeshType::Mesh250m,
        ),
        (
            "out/mesh125m.geojson",
            Rect::new(
                Coordinates::new(140.715, 41.765),
                Coordinates::new(140.735, 41.785),
            ),
            JPMeshType::Mesh125m,
        ),
    ];

    // 各設定に対してGeoJSONを生成して保存
    for (filename, bounds, mesh_type) in mesh_configs {
        println!("{}のGeoJSONを生成中...", filename);
        let geojson = create_geojson_for_mesh_type(bounds, mesh_type);
        save_geojson(&geojson, filename)?;
    }

    println!("すべてのGeoJSONファイルの生成が完了しました。");
    Ok(())
}
