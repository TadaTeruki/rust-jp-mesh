use crate::{
    Coordinates, JPMeshType, Rect,
    calcs::{to_5km::CodeTo5km, to_125m::CodeTo125m},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JPMeshCode {
    To125m {
        code: CodeTo125m,
        mesh_type: JPMeshType,
    },
    To5km {
        code: CodeTo5km,
        mesh_type: JPMeshType,
    },
}

impl JPMeshCode {
    pub fn new(coords: Coordinates, mesh_type: JPMeshType) -> Self {
        match mesh_type {
            JPMeshType::Mesh1km
            | JPMeshType::Mesh500m
            | JPMeshType::Mesh250m
            | JPMeshType::Mesh125m => {
                let code = CodeTo125m::from_coordinates(coords, mesh_type);
                JPMeshCode::To125m { code, mesh_type }
            }
            JPMeshType::Mesh80km | JPMeshType::Mesh10km | JPMeshType::Mesh5km => {
                let code = CodeTo5km::from_coordinates(coords, mesh_type);
                JPMeshCode::To5km { code, mesh_type }
            }
        }
    }

    pub fn from_number(mesh_code: u64, mesh_type: JPMeshType) -> Self {
        match mesh_type {
            JPMeshType::Mesh1km
            | JPMeshType::Mesh500m
            | JPMeshType::Mesh250m
            | JPMeshType::Mesh125m => {
                let code = CodeTo125m::from_number(mesh_code, mesh_type.code_length());
                JPMeshCode::To125m { code, mesh_type }
            }
            JPMeshType::Mesh80km | JPMeshType::Mesh10km | JPMeshType::Mesh5km => {
                let code = CodeTo5km::from_number(mesh_code, mesh_type.code_length());
                JPMeshCode::To5km { code, mesh_type }
            }
        }
    }

    pub fn to_bounds(&self) -> Rect {
        match self {
            Self::To125m { code, mesh_type } => code.to_bounds(*mesh_type),
            Self::To5km { code, mesh_type } => code.to_bounds(*mesh_type),
        }
    }

    pub fn is_inside(&self, coords: Coordinates) -> bool {
        let bounds = self.to_bounds();
        let min = bounds.min();
        let max = bounds.max();

        coords.lat >= min.lat
            && coords.lat < max.lat
            && coords.lng >= min.lng
            && coords.lng < max.lng
    }

    pub fn to_number(self) -> u64 {
        match self {
            Self::To125m { code, mesh_type } => code.to_number(mesh_type.code_length()),
            Self::To5km { code, mesh_type } => code.to_number(mesh_type.code_length()),
        }
    }

    pub fn mesh_type(&self) -> JPMeshType {
        match self {
            Self::To125m { mesh_type, .. } => *mesh_type,
            Self::To5km { mesh_type, .. } => *mesh_type,
        }
    }

    pub fn from_on_bounds(bounds: Rect, mesh_type: JPMeshType) -> Vec<Self> {
        let mut mesh_bins = vec![];
        let min = bounds.min();
        let max = bounds.max();
        let lat_len = ((max.lat - min.lat) / mesh_type.lat_interval()).ceil() as u64;
        let lng_len = ((max.lng - min.lng) / mesh_type.lng_interval()).ceil() as u64;

        let start_coords = JPMeshCode::new(min, mesh_type).to_bounds().center();

        for i in 0..=lat_len {
            for j in 0..=lng_len {
                let coords = Coordinates::new(
                    start_coords.lng + j as f64 * mesh_type.lng_interval(),
                    start_coords.lat + i as f64 * mesh_type.lat_interval(),
                );
                mesh_bins.push(JPMeshCode::new(coords, mesh_type));
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
        mesh_code_number: u64,
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
                mesh_code_number: 64414277,
                mesh_type: JPMeshType::Mesh1km,
                left_bottom: Coordinates::new(141.3375, 43.058333),
            },
            TestCase {
                mesh_code_number: 61401589,
                mesh_type: JPMeshType::Mesh1km,
                left_bottom: Coordinates::new(140.7375, 40.816667),
            },
            TestCase {
                mesh_code_number: 59414142,
                mesh_type: JPMeshType::Mesh1km,
                left_bottom: Coordinates::new(141.15, 39.7),
            },
            TestCase {
                mesh_code_number: 57403629,
                mesh_type: JPMeshType::Mesh1km,
                left_bottom: Coordinates::new(140.8625, 38.266667),
            },
        ];
    }

    #[test]
    fn test_mesh_code_generation() {
        for test_case in get_test_cases() {
            let inner_coord = test_case.inner_coord();
            let mesh_code = JPMeshCode::new(inner_coord, test_case.mesh_type);

            let actual_number = mesh_code.to_number();
            assert_eq!(actual_number, test_case.mesh_code_number);
        }
    }

    #[test]
    fn test_mesh_code_bounds() {
        for test_case in get_test_cases() {
            let inner_coord = test_case.inner_coord();
            let mesh_code = JPMeshCode::new(inner_coord, test_case.mesh_type);

            let bounds = mesh_code.to_bounds();
            let min_coord = bounds.min();

            // check if the bottom left coordinate is correct
            assert_approx_eq!(min_coord.lng, test_case.left_bottom.lng);
            assert_approx_eq!(min_coord.lat, test_case.left_bottom.lat);

            // check if the size of the area is correct
            assert_mesh_size_correct!(bounds, 45.0, 30.0);
        }
    }

    #[test]
    fn test_mesh_code_from_number_to_number() {
        for test_case in get_test_cases() {
            let mesh_code =
                JPMeshCode::from_number(test_case.mesh_code_number, test_case.mesh_type);
            let number = mesh_code.to_number();
            assert_eq!(number, test_case.mesh_code_number);
        }
    }
}
