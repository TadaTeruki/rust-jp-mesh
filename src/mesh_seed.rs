use crate::geom::{Coordinates, Rect};
use crate::mesh_type::JPMeshType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct JPMeshCodeSeed {
    // mesh code for 80km, 10km, 1km, 500m, 250m, 125m
    pub(crate) code_2: [u8; 11],
}

impl JPMeshCodeSeed {
    pub fn new(coords: Coordinates) -> Self {
        // latitude / interval (Mesh80km) = p % a
        let p = (coords.lat / JPMeshType::Mesh80km.lat_interval()).floor() as u8;
        let a = coords.lat % JPMeshType::Mesh80km.lat_interval();

        // a / lat_interval (Mesh10km) = q % b
        let q = (a / JPMeshType::Mesh10km.lat_interval()).floor() as u8;
        let b = a % JPMeshType::Mesh10km.lat_interval();

        // b / lat_interval (Mesh1km) = r % c
        let r = (b / JPMeshType::Mesh1km.lat_interval()).floor() as u8;
        let c = b % JPMeshType::Mesh1km.lat_interval();

        // c / lat_interval (Mesh500m) = s % d
        let s = (c / JPMeshType::Mesh500m.lat_interval()).floor() as u8;
        let d = c % JPMeshType::Mesh500m.lat_interval();

        // d / lat_interval (Mesh250m) = t % e
        let t = (d / JPMeshType::Mesh250m.lat_interval()).floor() as u8;
        let e = d % JPMeshType::Mesh250m.lat_interval();

        // e / lat_interval (Mesh125m) = tt
        let tt = (e / JPMeshType::Mesh125m.lat_interval()).floor() as u8;

        // longitude - 100 degrees = u % f
        let u = (coords.lng - 100.0).floor() as u8;
        let f = coords.lng - 100.0 - u as f64;

        // f / lng_interval (Mesh10km) = v % g
        let v = (f / JPMeshType::Mesh10km.lng_interval()).floor() as u8;
        let g = f % JPMeshType::Mesh10km.lng_interval();

        // g / lng_interval (Mesh1km) = w % h
        let w = (g / JPMeshType::Mesh1km.lng_interval()).floor() as u8;
        let h = g % JPMeshType::Mesh1km.lng_interval();

        // h / lng_interval (Mesh500m) = x % i
        let x = (h / JPMeshType::Mesh500m.lng_interval()).floor() as u8;
        let i = h % JPMeshType::Mesh500m.lng_interval();

        // i / lng_interval (Mesh250m) = y % j
        let y = (i / JPMeshType::Mesh250m.lng_interval()).floor() as u8;
        let j = i % JPMeshType::Mesh250m.lng_interval();

        // j / lng_interval (Mesh125m) = yy
        let yy = (j / JPMeshType::Mesh125m.lng_interval()).floor() as u8;

        // (s * 2)+(x + 1)= m
        let m = (s * 2) + (x + 1);

        // (t * 2)+(y + 1)= n
        let n = (t * 2) + (y + 1);

        // (tt * 2)+(yy + 1)= nn
        let nn = (tt * 2) + (yy + 1);

        // First 6 digits
        let head = {
            let p1 = (p / 10) % 10;
            let p2 = p % 10;
            let u1 = (u / 10) % 10;
            let u2 = u % 10;
            [p1, p2, u1, u2, q, v]
        };

        // Last 5 digits
        let tail_bin = { [r, w, m, n, nn] };

        let mut code_2 = [0u8; 11];
        code_2[..6].copy_from_slice(&head);
        code_2[6..11].copy_from_slice(&tail_bin);

        JPMeshCodeSeed { code_2 }
    }

    pub fn bounds(&self, mesh_type: JPMeshType) -> Rect {
        let mut code_2 = self.code_2;

        for (i, code) in code_2.iter_mut().enumerate().skip(mesh_type.code_length()) {
            *code = if i >= 8 { 1 } else { 0 };
        }

        let p = (code_2[0] * 10 + code_2[1]) as f64;
        let u = (code_2[2] * 10 + code_2[3]) as f64;
        let q = code_2[4] as f64;
        let v = code_2[5] as f64;
        let r = code_2[6] as f64;
        let w = code_2[7] as f64;
        let m = code_2[8] as f64;
        let n = code_2[9] as f64;
        let nn = code_2[10] as f64;

        // Calculate latitude (southwest corner)
        let lat_base = p * JPMeshType::Mesh80km.lat_interval();
        let lat_q = q * JPMeshType::Mesh10km.lat_interval();
        let lat_r = r * JPMeshType::Mesh1km.lat_interval();
        let lat_s = ((m - 1.0) / 2.0).floor() * JPMeshType::Mesh500m.lat_interval();
        let lat_t = ((n - 1.0) / 2.0).floor() * JPMeshType::Mesh250m.lat_interval();
        let lat_tt = ((nn - 1.0) / 2.0).floor() * JPMeshType::Mesh125m.lat_interval();

        // Calculate longitude (southwest corner)
        let lng_base = 100.0 + u;
        let lng_v = v * JPMeshType::Mesh10km.lng_interval();
        let lng_w = w * JPMeshType::Mesh1km.lng_interval();
        let lng_x = ((m - 1.0) % 2.0) * JPMeshType::Mesh500m.lng_interval();
        let lng_y = ((n - 1.0) % 2.0) * JPMeshType::Mesh250m.lng_interval();
        let lng_yy = ((nn - 1.0) % 2.0) * JPMeshType::Mesh125m.lng_interval();

        // Coordinates of southwest corner
        let min_lat = lat_base + lat_q + lat_r + lat_s + lat_t + lat_tt;
        let min_lng = lng_base + lng_v + lng_w + lng_x + lng_y + lng_yy;

        // Coordinates of northeast corner
        let max_lat = min_lat + mesh_type.lat_interval();
        let max_lng = min_lng + mesh_type.lng_interval();

        Rect::new(
            Coordinates::new_(min_lng, min_lat),
            Coordinates::new_(max_lng, max_lat),
        )
    }
}
