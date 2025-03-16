pub mod to125m;
pub mod to50m;
pub mod tosquared;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JPMeshCalcType {
    To125m,
    To50m,
    ToSquared,
}
