#[repr(usize)]
pub enum Cell {
    Alive = 1_usize,
    Dead = 0_usize,
}

impl From<bool> for Cell {
    fn from(t: bool) -> Self {
        match t {
            true => Cell::Alive,
            false => Cell::Dead,
        }
    }
}

impl Into<bool> for Cell {
    fn into(self) -> bool {
        match self {
            Cell::Alive => true,
            Cell::Dead => false,
        }
    }
}
