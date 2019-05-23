use im::HashMap as ImHashMap;

pub type Change = (Coord, Color);
pub type Diff = Vec<Coord>;
pub type Map = ImHashMap<Coord, Color>;
pub type Coord = (usize, usize);
pub type Region = (Coord, Coord);
pub type Color = (u8, u8, u8);

