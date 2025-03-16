#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JPMeshType {
    /// 第1次地域区画
    Mesh80km,
    /// 第2次地域区画
    Mesh10km,
    /// 基準地域メッシュ
    Mesh1km,
    /// 2分の1地域メッシュ
    Mesh500m,
    /// 4分の1地域メッシュ
    Mesh250m,
    /// 8分の1地域メッシュ
    Mesh125m,
    /// 5倍地域メッシュ
    Mesh5km,
}

impl Ord for JPMeshType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.code_length().cmp(&other.code_length()).reverse()
    }
}

impl PartialOrd for JPMeshType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl JPMeshType {
    pub const fn code_length(&self) -> usize {
        match self {
            JPMeshType::Mesh80km => 4,
            JPMeshType::Mesh10km => 6,
            JPMeshType::Mesh1km => 8,
            JPMeshType::Mesh500m => 9,
            JPMeshType::Mesh250m => 10,
            JPMeshType::Mesh125m => 11,
            JPMeshType::Mesh5km => 7,
        }
    }

    const fn lat_interval_seconds(&self) -> f64 {
        match self {
            JPMeshType::Mesh80km => 2400.0,
            JPMeshType::Mesh10km => 300.0,
            JPMeshType::Mesh1km => 30.0,
            JPMeshType::Mesh500m => 15.0,
            JPMeshType::Mesh250m => 7.5,
            JPMeshType::Mesh125m => 3.75,
            JPMeshType::Mesh5km => 150.0,
        }
    }

    const fn lng_interval_seconds(&self) -> f64 {
        match self {
            JPMeshType::Mesh80km => 3600.0,
            JPMeshType::Mesh10km => 450.0,
            JPMeshType::Mesh1km => 45.0,
            JPMeshType::Mesh500m => 22.5,
            JPMeshType::Mesh250m => 11.25,
            JPMeshType::Mesh125m => 5.625,
            JPMeshType::Mesh5km => 225.0,
        }
    }

    pub const fn lat_interval(&self) -> f64 {
        self.lat_interval_seconds() / 3600.0
    }

    pub const fn lng_interval(&self) -> f64 {
        self.lng_interval_seconds() / 3600.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_type_order() {
        let mesh_types = vec![
            JPMeshType::Mesh80km,
            JPMeshType::Mesh10km,
            JPMeshType::Mesh1km,
            JPMeshType::Mesh500m,
            JPMeshType::Mesh250m,
            JPMeshType::Mesh125m,
        ];

        for i in 1..mesh_types.len() {
            assert!(mesh_types[i - 1] > mesh_types[i]);
        }
    }
}
