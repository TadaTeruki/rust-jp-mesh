use crate::{Coordinates, JPMeshType, Rect, code_num::CodeNum};

pub type CodeTo5km = CodeNum<7, 1>;

impl CodeTo5km {
    pub fn from_coordinates(coords: Coordinates, mesh_type: JPMeshType) -> Self {
        // latitude / interval (Mesh80km) = p % a
        let p = (coords.lat / JPMeshType::Mesh80km.lat_interval()).floor() as u8;
        let a = coords.lat % JPMeshType::Mesh80km.lat_interval();

        // longitude - 100 degrees = u % f
        let u = (coords.lng - 100.0).floor() as u8;
        let f = coords.lng - 100.0 - u as f64;

        let p1 = (p / 10) % 10;
        let p2 = p % 10;
        let u1 = (u / 10) % 10;
        let u2 = u % 10;

        if mesh_type == JPMeshType::Mesh80km {
            return CodeNum::new(&[p1, p2, u1, u2]);
        }

        // a / lat_interval (Mesh10km) = q % b
        let q = (a / JPMeshType::Mesh10km.lat_interval()).floor() as u8;
        let b = a % JPMeshType::Mesh10km.lat_interval();

        // f / lng_interval (Mesh10km) = v % g
        let v = (f / JPMeshType::Mesh10km.lng_interval()).floor() as u8;
        let g = f % JPMeshType::Mesh10km.lng_interval();

        if mesh_type == JPMeshType::Mesh10km {
            return CodeNum::new(&[p1, p2, u1, u2, q, v]);
        }

        // b / lat_interval (Mesh5km) = r % c
        let r = (b / JPMeshType::Mesh5km.lat_interval()).floor() as u8;

        // g / lng_interval (Mesh5km) = w % h
        let w = (g / JPMeshType::Mesh5km.lng_interval()).floor() as u8;

        let m = (r * 2) + (w + 1);

        CodeNum::new(&[p1, p2, u1, u2, q, v, m])
    }

    pub fn to_bounds(self, mesh_type: JPMeshType) -> Rect {
        let code_array = self.to_array();

        let p = (code_array[0] * 10 + code_array[1]) as f64;
        let u = (code_array[2] * 10 + code_array[3]) as f64;
        let q = code_array[4] as f64;
        let v = code_array[5] as f64;
        let m = code_array[6] as f64;

        // Calculate latitude (southwest corner)
        let lat_base = p * JPMeshType::Mesh80km.lat_interval();
        let lat_q = q * JPMeshType::Mesh10km.lat_interval();
        let lat_r = ((m - 1.0) / 2.0).floor() * JPMeshType::Mesh5km.lat_interval();

        // Calculate longitude (southwest corner)
        let lng_base = 100.0 + u;
        let lng_v = v * JPMeshType::Mesh10km.lng_interval();
        let lng_w = ((m - 1.0) % 2.0) * JPMeshType::Mesh5km.lng_interval();

        // Coordinates of southwest corner
        let min_lat = lat_base + lat_q + lat_r;
        let min_lng = lng_base + lng_v + lng_w;

        // Coordinates of northeast corner
        let max_lat = min_lat + mesh_type.lat_interval();
        let max_lng = min_lng + mesh_type.lng_interval();

        Rect::new(
            Coordinates::new(min_lng, min_lat),
            Coordinates::new(max_lng, max_lat),
        )
    }
}
