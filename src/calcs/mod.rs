pub mod mesh_to125m;
pub mod mesh_to50m;
pub mod mesh_tosquared;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JPMeshCalcType {
    To125m,
    To50m,
    ToSquared,
}
