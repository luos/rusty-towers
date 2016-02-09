
#[derive(Clone,Copy)]
pub struct MapElement {
	pub x: u32,
	pub y: u32,
    pub color: [f32; 4]
}

pub struct Map {
	pub width: u32,
	pub height: u32,
    pub elements: Vec<Vec<MapElement>>
}

