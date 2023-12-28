use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};
use std::mem::size_of;

pub trait BitFieldRepresentation: BitAnd<Output = Self> + BitOr<Output = Self> + BitXor<Output = Self> + Not<Output = Self> + Shl<u8, Output = Self> + Shr<u8, Output = Self> + Copy + Sized + From<u8>
{}

impl BitFieldRepresentation for u8 {}
impl BitFieldRepresentation for u16 {}
impl BitFieldRepresentation for u32 {}
impl BitFieldRepresentation for u64 {}
impl BitFieldRepresentation for u128 {}

pub trait BitFieldCompatible<T>
where T: BitFieldRepresentation {
    fn from_type(value: T) -> Self;
    fn to_type(&self) -> T;
}

/// A BitsField is a field of bits.
/// Each element is represented by a certain number of bits.
pub struct BitsField<T>
where T: BitFieldRepresentation  {
    bits_per_element: usize,
    element_size: usize,
    elements: Vec<T>,
}

impl<T> BitsField<T>
where T: BitFieldRepresentation {

    pub fn default(bits_per_element: usize) -> Self {
        BitsField {
            bits_per_element: 1,
            element_size: size_of::<T>()*8,
            elements: Vec::new(),
        }
    }

    pub fn new(bits_per_element: usize, size: usize) -> Result<Self, &'static str> {
        match bits_per_element {
            0 => Err("bits_per_element must be greater than 0"),
            1..=8 => Ok(BitsField {
                bits_per_element,
                element_size: size_of::<T>()*8,
                elements: vec![T::from(0); (size as f64 * bits_per_element as f64 / (size_of::<T>()*8) as f64).ceil() as usize],
            }),
            _ => Err("bits_per_element must be less than 8"),
        }
    }

    pub fn clear(&mut self) {
        self.elements = Vec::new();
    }

    pub fn len(&self) -> usize {
        ((self.elements.len() * self.element_size) as f64 / self.bits_per_element as f64).floor() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn convert_index(&self, index: usize) -> Result<(usize, usize), &'static str> {
        if index >= self.len() {
            Err("index out of bounds")
        } else {
            let element_index = (index * self.bits_per_element as usize) / self.element_size;
            let bit_index = (index * self.bits_per_element as usize) % self.element_size;
            Ok((element_index, bit_index))
        }
    }

    pub fn get<Output>(&self, index: usize) -> Result<Output, &str> 
    where Output: BitFieldCompatible<T> {
        let (element_index, bit_index) = self.convert_index(index)?;
        // Maybe the data is overlapping on the next element.
        Ok(Output::from_type(
            if (self.element_size - bit_index) < self.bits_per_element {
                // We need to get some bits from the next element.
                // There is two masks to build.
                // The mask of the current element.
                // The mask of the next element.

                let left_index = self.element_size - bit_index;
                let right_index = self.bits_per_element - left_index;

                let mut current_mask = T::from(0);
                for i in 0..left_index {
                    current_mask = current_mask | (T::from(1) << (bit_index + i) as u8)
                }
                let mut next_mask = T::from(0);
                for i in 0..right_index {
                    next_mask = next_mask | (T::from(1) << i as u8)
                }
                ((self.elements[element_index] & current_mask) << right_index as u8) | ((self.elements[element_index + 1] & next_mask) >> (self.element_size - right_index) as u8)
            } else {
                // We can get all the bits from the current element.
                // We need to mask the bits we want.
                // Let's build the mask.
                let mut mask = T::from(0);
                for i in 0..self.bits_per_element {
                    mask = mask | (T::from(1) << (bit_index + i) as u8)
                }
                self.elements[element_index] & mask
            }
        ))
    }

    pub fn set<Output>(&mut self, index: usize, value: Output) -> Result<(), &'static str>
    where Output: BitFieldCompatible<T> {
        let (element_index, bit_index) = (&self).convert_index(index)?;
        
        // Maybe the data is overlapping on the next element.
        if (self.element_size - bit_index) < self.bits_per_element {
            // We need to set some bits on the next element.
            // There is two masks to build.
            // The mask of the current element.
            // The mask of the next element.

            let left_index = self.element_size - bit_index;
            let right_index = self.bits_per_element - left_index;

            let mut current_mask = T::from(0);
            for i in 0..left_index {
                current_mask = current_mask | (T::from(1) << (bit_index + i) as u8)
            }
            let mut next_mask = T::from(0);
            for i in 0..right_index {
                next_mask = next_mask | (T::from(1) << i as u8)
            }
            self.elements[element_index] = (self.elements[element_index] & !current_mask) | ((value.to_type() >> (right_index as u8) << (bit_index - left_index) as u8) & current_mask);
            self.elements[element_index + 1] = (self.elements[element_index + 1] & !next_mask) | ((value.to_type() << (self.element_size - right_index) as u8) & next_mask);
        } else {
            // We can set all the bits on the current element.
            // We need to mask the bits we want.
            // Let's build the mask.
            let mut mask = T::from(0);
            for i in 0..self.bits_per_element {
                mask = mask | (T::from(1) << (bit_index + i) as u8)
            }
            self.elements[element_index] = (self.elements[element_index] & !mask) | ((value.to_type() << (bit_index - self.bits_per_element) as u8) & mask);
        }
        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::BitFieldCompatible;


    #[derive(Debug, PartialEq)]
    enum TwoBitsState {
        Alive = 1,
        Dead = 0,
    }

    impl BitFieldCompatible<u8> for TwoBitsState {
        fn from_type(value: u8) -> Self {
            match value {
                0 => TwoBitsState::Dead,
                _ => TwoBitsState::Alive,
            }
        }

        fn to_type(&self) -> u8 {
            match self {
                TwoBitsState::Alive => 1,
                TwoBitsState::Dead => 0,
            }
        }
    }

    #[test]
    fn test_two_bits_states() {
        // Create a new BitsField with 2 bits per element. And 16 elements.
        let mut bits_field = super::BitsField::<u8>::new(1, 16).unwrap();
        // Check if the vec is sized correctly.
        assert_eq!(bits_field.elements.len(), 2);

        // Check if the bits_field is not empty.
        assert_eq!(bits_field.is_empty(), false);

        // Set the even elements to Alive.
        for i in 0..bits_field.len() {
            if i % 2 == 0 {
                bits_field.set(i, TwoBitsState::Alive).unwrap();
            }
        }

        // Check if the even elements are Alive.
        for i in 0..bits_field.len() {
            if i % 2 == 0 {
                assert_eq!(bits_field.get::<TwoBitsState>(i).unwrap(), TwoBitsState::Alive);
            } else {
                assert_eq!(bits_field.get::<TwoBitsState>(i).unwrap(), TwoBitsState::Dead);
            }
        }

    }

}