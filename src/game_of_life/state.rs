
pub enum State {
    Alive = 1,
    Dead = 0,
}

impl State {
    fn is_alive(&self) -> bool {
        match self {
            State::Alive => true,
            State::Dead => false,
        }
    }

    fn to_bit(&self, bit_index: u8) -> u8 {
        match self {
            State::Alive => 1 << bit_index,
            State::Dead => 0,
        }
    }

    fn from_bit(bit: u8, bit_index: u8) -> State {
        match bit & (1 << bit_index) {
            0 => State::Dead,
            _ => State::Alive,
        }
    }
}