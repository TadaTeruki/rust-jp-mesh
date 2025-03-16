use crate::{
    Coordinates, JPMeshType, Rect,
    calcs::{to_2km::CodeTo2km, to_5km::CodeTo5km, to_125m::CodeTo125m},
};

/// 地域メッシュを表現します。
///
/// # サンプル
/// ```
/// use rust_jp_mesh::{Coordinates, JPMesh, JPMeshType};
///
/// let coords = Coordinates::new(139.767125, 35.681236);
/// let mesh = JPMesh::new(coords, JPMeshType::Mesh1km);
/// assert_eq!(mesh.to_number(), 53394611);
/// assert!(mesh.to_bounds().includes(coords));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JPMesh {
    To125m {
        code: CodeTo125m,
        mesh_type: JPMeshType,
    },
    To2km {
        code: CodeTo2km,
        mesh_type: JPMeshType,
    },
    To5km {
        code: CodeTo5km,
        mesh_type: JPMeshType,
    },
}

impl JPMesh {
    /// 指定された座標から地域メッシュを生成します。
    pub fn new(coords: Coordinates, mesh_type: JPMeshType) -> Self {
        match mesh_type {
            JPMeshType::Mesh1km
            | JPMeshType::Mesh500m
            | JPMeshType::Mesh250m
            | JPMeshType::Mesh125m => {
                let code = CodeTo125m::from_coordinates(coords, mesh_type);
                JPMesh::To125m { code, mesh_type }
            }
            JPMeshType::Mesh2km => {
                let code = CodeTo2km::from_coordinates(coords, mesh_type);
                JPMesh::To2km { code, mesh_type }
            }
            JPMeshType::Mesh80km | JPMeshType::Mesh10km | JPMeshType::Mesh5km => {
                let code = CodeTo5km::from_coordinates(coords, mesh_type);
                JPMesh::To5km { code, mesh_type }
            }
        }
    }

    /// 指定された地域メッシュコードと種類から地域メッシュを生成します。
    pub fn from_number(mesh: u64, mesh_type: JPMeshType) -> Self {
        match mesh_type {
            JPMeshType::Mesh1km
            | JPMeshType::Mesh500m
            | JPMeshType::Mesh250m
            | JPMeshType::Mesh125m => {
                let code = CodeTo125m::from_number(mesh);
                JPMesh::To125m { code, mesh_type }
            }
            JPMeshType::Mesh2km => {
                let code = CodeTo2km::from_number(mesh);
                JPMesh::To2km { code, mesh_type }
            }
            JPMeshType::Mesh80km | JPMeshType::Mesh10km | JPMeshType::Mesh5km => {
                let code = CodeTo5km::from_number(mesh);
                JPMesh::To5km { code, mesh_type }
            }
        }
    }

    /// 地域メッシュの範囲を表す矩形を取得します。
    pub fn to_bounds(&self) -> Rect {
        match self {
            Self::To125m { code, mesh_type } => code.to_bounds(*mesh_type),
            Self::To2km { code, mesh_type } => code.to_bounds(*mesh_type),
            Self::To5km { code, mesh_type } => code.to_bounds(*mesh_type),
        }
    }

    /// 地域メッシュコードを取得します。
    pub fn to_number(self) -> u64 {
        match self {
            Self::To125m { code, mesh_type } => code.to_number(mesh_type.code_length()),
            Self::To2km { code, mesh_type } => code.to_number(mesh_type.code_length()),
            Self::To5km { code, mesh_type } => code.to_number(mesh_type.code_length()),
        }
    }

    /// 地域メッシュの種類を取得します。
    pub fn mesh_type(&self) -> JPMeshType {
        match self {
            Self::To125m { mesh_type, .. } => *mesh_type,
            Self::To2km { mesh_type, .. } => *mesh_type,
            Self::To5km { mesh_type, .. } => *mesh_type,
        }
    }

    /// 指定された矩形範囲に含まれる地域メッシュを取得します。
    pub fn from_on_bounds(bounds: Rect, mesh_type: JPMeshType) -> Vec<Self> {
        let mut mesh_bins = vec![];
        let min = bounds.min();
        let max = bounds.max();
        let lat_len = ((max.lat - min.lat) / mesh_type.lat_interval()).ceil() as u64;
        let lng_len = ((max.lng - min.lng) / mesh_type.lng_interval()).ceil() as u64;

        let start = JPMesh::new(min, mesh_type).to_bounds().center();

        for i in 0..=lat_len {
            for j in 0..=lng_len {
                let coords = Coordinates::new(
                    start.lng + j as f64 * mesh_type.lng_interval(),
                    start.lat + i as f64 * mesh_type.lat_interval(),
                );
                mesh_bins.push(JPMesh::new(coords, mesh_type));
            }
        }

        mesh_bins
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = 1e-6;

    #[macro_export]
    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr) => {
            assert!(
                ($a - $b).abs() < EPSILON,
                "assertion failed: `(left ≈ right)`\n  left: `{}`\n right: `{}`\n",
                $a,
                $b
            );
        };
    }

    #[macro_export]
    macro_rules! assert_mesh_size_correct {
        ($bounds:expr, $lng_interval_seconds:expr, $lat_interval_seconds:expr) => {
            let min_coord = $bounds.min();
            let max_coord = $bounds.max();
            assert_approx_eq!(
                max_coord.lng - min_coord.lng,
                $lng_interval_seconds / 3600.0
            );
            assert_approx_eq!(
                max_coord.lat - min_coord.lat,
                $lat_interval_seconds / 3600.0
            );
        };
    }

    // small offset for checking coordinate inside the mesh
    const INNER_OFFSET: f64 = 0.000003;

    #[derive(Debug)]
    struct TestCase {
        mesh_number: u64,
        mesh_type: JPMeshType,
        left_bottom: Coordinates,
    }

    impl TestCase {
        fn inner_coord(&self) -> Coordinates {
            Coordinates::new(
                self.left_bottom.lng + INNER_OFFSET,
                self.left_bottom.lat + INNER_OFFSET,
            )
        }
    }

    fn get_test_cases() -> Vec<TestCase> {
        return vec![
            TestCase {
                mesh_number: 64414277,
                mesh_type: JPMeshType::Mesh1km,
                left_bottom: Coordinates::new(141.3375, 43.058333),
            },
            TestCase {
                mesh_number: 61401589,
                mesh_type: JPMeshType::Mesh1km,
                left_bottom: Coordinates::new(140.7375, 40.816667),
            },
            TestCase {
                mesh_number: 59414142,
                mesh_type: JPMeshType::Mesh1km,
                left_bottom: Coordinates::new(141.15, 39.7),
            },
            TestCase {
                mesh_number: 57403629,
                mesh_type: JPMeshType::Mesh1km,
                left_bottom: Coordinates::new(140.8625, 38.266667),
            },
        ];
    }

    #[test]
    fn test_mesh_generation() {
        for test_case in get_test_cases() {
            let inner_coord = test_case.inner_coord();
            let mesh = JPMesh::new(inner_coord, test_case.mesh_type);

            let actual_number = mesh.to_number();
            assert_eq!(actual_number, test_case.mesh_number);
        }
    }

    #[test]
    fn test_mesh_bounds() {
        for test_case in get_test_cases() {
            let inner_coord = test_case.inner_coord();
            let mesh = JPMesh::new(inner_coord, test_case.mesh_type);

            let bounds = mesh.to_bounds();
            let min_coord = bounds.min();

            // check if the bottom left coordinate is correct
            assert_approx_eq!(min_coord.lng, test_case.left_bottom.lng);
            assert_approx_eq!(min_coord.lat, test_case.left_bottom.lat);

            // check if the size of the area is correct
            assert_mesh_size_correct!(bounds, 45.0, 30.0);
        }
    }

    #[test]
    fn test_mesh_from_number_to_number() {
        for test_case in get_test_cases() {
            let mesh = JPMesh::from_number(test_case.mesh_number, test_case.mesh_type);
            let number = mesh.to_number();
            assert_eq!(number, test_case.mesh_number);
        }
    }

    #[test]
    fn test_mesh_corner() {
        let mesh = JPMesh::new(Coordinates::new(141.15, 39.7), JPMeshType::Mesh1km);
        let bounds = mesh.to_bounds();
        let min_coord = bounds.min();

        assert_approx_eq!(min_coord.lng, 141.15);
        assert_approx_eq!(min_coord.lat, 39.7);
    }
}
