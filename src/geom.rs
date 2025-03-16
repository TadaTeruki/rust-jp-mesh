/// 座標を表す構造体
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinates {
    /// 経度
    pub lng: f64,
    /// 緯度
    pub lat: f64,
}

impl Coordinates {
    /// 指定された経度と緯度を持つ座標を生成します。
    pub fn new(lng: f64, lat: f64) -> Self {
        Self { lng, lat }
    }
}

/// 矩形を表す構造体
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    min_coord: Coordinates,
    max_coord: Coordinates,
}

impl Rect {
    /// 指定された座標を含む矩形を生成します。
    pub fn new(min_coord: Coordinates, max_coord: Coordinates) -> Self {
        Self {
            min_coord,
            max_coord,
        }
    }

    /// 矩形の最小座標を取得します。
    pub fn min(&self) -> Coordinates {
        self.min_coord
    }

    /// 矩形の最大座標を取得します。
    pub fn max(&self) -> Coordinates {
        self.max_coord
    }

    /// 矩形の中心座標を取得します。
    pub fn center(&self) -> Coordinates {
        Coordinates::new(
            (self.min_coord.lng + self.max_coord.lng) / 2.0,
            (self.min_coord.lat + self.max_coord.lat) / 2.0,
        )
    }

    /// 指定された座標が矩形に含まれるかどうかを判定します。
    pub fn includes(&self, coords: Coordinates) -> bool {
        let min = self.min();
        let max = self.max();

        coords.lat >= min.lat
            && coords.lat < max.lat
            && coords.lng >= min.lng
            && coords.lng < max.lng
    }
}
