use crate::geom::{Coordinates, Rect};
use crate::mesh_seed::JPMeshCodeSeed;
use crate::mesh_type::JPMeshType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JPMeshCode {
    mesh_type: JPMeshType,
    seed: JPMeshCodeSeed,
}

impl JPMeshCode {
    pub fn new(coords: Coordinates, mesh_type: JPMeshType) -> Self {
        let seed = JPMeshCodeSeed::new(coords);
        JPMeshCode { mesh_type, seed }
    }

    pub fn from_number(mesh_code: u64, mesh_type: JPMeshType) -> Self {
        let mut code_2 = [0u8; 11];
        let mut mesh_code = mesh_code;
        let ifirst = 11 - mesh_type.code_length();
        for i in (0..11).rev() {
            let value = (mesh_code % 10) as u8;
            if i >= ifirst {
                code_2[i - ifirst] = value;
            }
            mesh_code /= 10;
        }

        JPMeshCode {
            mesh_type,
            seed: JPMeshCodeSeed { code_2 },
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

    pub fn downscale(&self) -> Vec<Self> {
        match self.mesh_type {
            JPMeshType::Mesh80km => (0..8)
                .flat_map(|i| {
                    (0..8)
                        .map(|j| {
                            Self::from_number(
                                self.to_number() * 100 + i * 10 + j,
                                JPMeshType::Mesh10km,
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .collect(),
            JPMeshType::Mesh10km => (0..10)
                .flat_map(|i| {
                    (0..10)
                        .map(|j| {
                            Self::from_number(
                                self.to_number() * 100 + i * 10 + j,
                                JPMeshType::Mesh1km,
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .collect(),
            JPMeshType::Mesh1km => (1..=4)
                .map(|i| Self::from_number(self.to_number() * 10 + i, JPMeshType::Mesh500m))
                .collect(),
            JPMeshType::Mesh500m => (1..=4)
                .map(|i| Self::from_number(self.to_number() * 10 + i, JPMeshType::Mesh250m))
                .collect(),
            JPMeshType::Mesh250m => (1..=4)
                .map(|i| Self::from_number(self.to_number() * 10 + i, JPMeshType::Mesh125m))
                .collect(),
            _ => Vec::new(),
        }
    }

    pub fn upscale(&self) -> Option<Self> {
        match self.mesh_type {
            JPMeshType::Mesh10km => Some(Self::from_number(
                self.to_number() / 100,
                JPMeshType::Mesh80km,
            )),
            JPMeshType::Mesh1km => Some(Self::from_number(
                self.to_number() / 100,
                JPMeshType::Mesh10km,
            )),
            JPMeshType::Mesh500m => Some(Self::from_number(
                self.to_number() / 10,
                JPMeshType::Mesh1km,
            )),
            JPMeshType::Mesh250m => Some(Self::from_number(
                self.to_number() / 10,
                JPMeshType::Mesh500m,
            )),
            JPMeshType::Mesh125m => Some(Self::from_number(
                self.to_number() / 10,
                JPMeshType::Mesh250m,
            )),
            _ => None,
        }
    }

    pub fn to_number(&self) -> u64 {
        let mut result = 0;
        for &digit in self.to_slice() {
            result = result * 10 + digit as u64;
        }
        result
    }

    pub fn to_slice(&self) -> &[u8] {
        match self.mesh_type {
            JPMeshType::Mesh80km => &self.seed.code_2[..JPMeshType::Mesh80km.code_length()],
            JPMeshType::Mesh10km => &self.seed.code_2[..JPMeshType::Mesh10km.code_length()],
            JPMeshType::Mesh1km => &self.seed.code_2[..JPMeshType::Mesh1km.code_length()],
            JPMeshType::Mesh500m => &self.seed.code_2[..JPMeshType::Mesh500m.code_length()],
            JPMeshType::Mesh250m => &self.seed.code_2[..JPMeshType::Mesh250m.code_length()],
            JPMeshType::Mesh125m => &self.seed.code_2[..JPMeshType::Mesh125m.code_length()],
        }
    }

    pub fn bounds(&self) -> Rect {
        self.seed.bounds(self.mesh_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assert_approx_eq, assert_mesh_size_correct, assert_rect_includes, assert_rect_not_includes,
    };

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

    #[test]
    fn test_mesh_code_upscale() {
        // Create larger scale meshes by truncating digits from the dataset's mesh_code,
        // and verify that the dataset's inner coordinates are contained within these mesh boundaries
        for test_case in get_test_cases() {
            let mesh_code =
                JPMeshCode::from_number(test_case.mesh_code_number, test_case.mesh_type);

            // 1km -> 10km
            let mesh_code_10km = mesh_code.upscale().unwrap();
            let bounds_10km = mesh_code_10km.bounds();

            // verify that inner coordinates are contained within the mesh boundaries
            let inner_coord = test_case.inner_coord();
            assert_rect_includes!(bounds_10km, inner_coord);

            // verify that mesh size is correct
            assert_mesh_size_correct!(bounds_10km, 450.0, 300.0);

            // 1km -> 80km
            let mesh_code_80km = mesh_code_10km.upscale().unwrap();
            let bounds_80km = mesh_code_80km.bounds();

            // check if the inner coordinate is included in the mesh
            assert_rect_includes!(bounds_80km, inner_coord);

            // check if the size of the area is correct
            assert_mesh_size_correct!(bounds_80km, 3600.0, 2400.0);
        }
    }

    #[test]
    fn test_mesh_code_downscale() {
        // Create smaller scale meshes by adding digits to the dataset's mesh_code,
        // and verify that the dataset's inner coordinates are contained within these mesh boundaries
        for test_case in get_test_cases() {
            // the mesh code will be (test_case.mesh_code_number * 1000 + 111)
            let inner_coord = test_case.inner_coord();
            let mesh_codes =
                JPMeshCode::from_number(test_case.mesh_code_number, test_case.mesh_type);

            // 1km -> 500m
            let mesh_codes_500m = mesh_codes.downscale();
            for (i, mesh_code_500m) in mesh_codes_500m.iter().enumerate() {
                let bounds_500m = mesh_code_500m.bounds();

                assert_mesh_size_correct!(bounds_500m, 22.5, 15.0);

                if i == 0 {
                    assert_rect_includes!(bounds_500m, inner_coord);
                } else {
                    assert_rect_not_includes!(bounds_500m, inner_coord);
                }
            }

            // 1km -> 250m
            let mesh_codes_250m = mesh_codes_500m.first().unwrap().downscale();
            for (i, mesh_code_250m) in mesh_codes_250m.iter().enumerate() {
                let bounds_250m = mesh_code_250m.bounds();

                assert_mesh_size_correct!(bounds_250m, 11.25, 7.5);

                if i == 0 {
                    assert_rect_includes!(bounds_250m, inner_coord);
                } else {
                    assert_rect_not_includes!(bounds_250m, inner_coord);
                }
            }

            // 1km -> 125m
            let mesh_codes_125m = mesh_codes_250m.first().unwrap().downscale();
            for (i, mesh_code_125m) in mesh_codes_125m.iter().enumerate() {
                let bounds_125m = mesh_code_125m.bounds();

                assert_mesh_size_correct!(bounds_125m, 5.625, 3.75);

                if i == 0 {
                    assert_rect_includes!(bounds_125m, inner_coord);
                } else {
                    assert_rect_not_includes!(bounds_125m, inner_coord);
                }
            }
        }
    }
}
