mod code11;
mod geom;
mod mesh_code;
mod mesh_type;

pub use geom::{Coordinates, Rect};
pub use mesh_code::JPMeshCode;
pub use mesh_type::JPMeshType;

#[cfg(test)]
const EPSILON: f64 = 1e-6;

#[cfg(test)]
#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {
        assert!(
            ($a - $b).abs() < crate::EPSILON,
            "assertion failed: `(left ≈ right)`\n  left: `{}`\n right: `{}`\n",
            $a,
            $b
        );
    };
}

#[cfg(test)]
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
