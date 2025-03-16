/// 地域メッシュコードの種類
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

impl JPMeshType {
    pub(crate) const fn code_length(&self) -> usize {
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

    /// 緯度方向のメッシュ幅を取得します。(度)
    pub const fn lat_interval(&self) -> f64 {
        self.lat_interval_seconds() / 3600.0
    }

    /// 経度方向のメッシュ幅を取得します。(度)
    pub const fn lng_interval(&self) -> f64 {
        self.lng_interval_seconds() / 3600.0
    }
}
