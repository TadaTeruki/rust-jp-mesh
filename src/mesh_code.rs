use crate::JPMeshType;
use crate::code2::Code2;
use crate::geom::{Coordinates, Rect};
use crate::geom_code::{code2_from_coordinates, code2_to_bounds};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JPMeshCode {
    mesh_type: JPMeshType,
    code_2: Code2,
}

impl JPMeshCode {
    pub fn new(coords: Coordinates, mesh_type: JPMeshType) -> Self {
        JPMeshCode {
            mesh_type,
            code_2: code2_from_coordinates(coords, mesh_type),
        }
    }

    pub fn from_number(mesh_code: u64, mesh_type: JPMeshType) -> Self {
        JPMeshCode {
            mesh_type,
            code_2: Code2::from_number(mesh_code, mesh_type.code_length()),
        }
    }

    pub fn from_on_bounds(bounds: Rect, mesh_type: JPMeshType) -> Vec<Self> {
        let mut mesh_codes = vec![];
        let min = bounds.min();
        let max = bounds.max();
        let lat_len = ((max.lat - min.lat) / mesh_type.lat_interval()).ceil() as u64;
        let lng_len = ((max.lng - min.lng) / mesh_type.lng_interval()).ceil() as u64;

        for i in 0..=lat_len {
            for j in 0..=lng_len {
                let coords = Coordinates::new_(
                    min.lng + j as f64 * mesh_type.lng_interval(),
                    min.lat + i as f64 * mesh_type.lat_interval(),
                );
                mesh_codes.push(JPMeshCode::new(coords, mesh_type));
            }
        }

        mesh_codes
    }

    pub fn to_number(&self) -> u64 {
        self.code_2.to_number(self.mesh_type.code_length())
    }

    pub fn bounds(&self) -> Rect {
        code2_to_bounds(self.code_2, self.mesh_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_approx_eq, assert_mesh_size_correct};

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
            Coordinates::new_(
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
                left_bottom: Coordinates::new_(141.3375, 43.058333),
            },
            TestCase {
                mesh_code_number: 61401589,
                mesh_type: JPMeshType::Mesh1km,
                left_bottom: Coordinates::new_(140.7375, 40.816667),
            },
            TestCase {
                mesh_code_number: 59414142,
                mesh_type: JPMeshType::Mesh1km,
                left_bottom: Coordinates::new_(141.15, 39.7),
            },
            TestCase {
                mesh_code_number: 57403629,
                mesh_type: JPMeshType::Mesh1km,
                left_bottom: Coordinates::new_(140.8625, 38.266667),
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

            let bounds = mesh_code.bounds();
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
