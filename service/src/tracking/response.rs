#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Cords {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Shape {
    pub k: String,
    pub v: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TrackingResponse {
    pub timestamp: u64,
    pub hotkey: i16,
    pub face_found: bool,
    pub rotation: Cords,
    pub position: Cords,
    pub eye_left: Cords,
    pub blend_shapes: Vec<Shape>,
}
