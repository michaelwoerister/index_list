
// Bits

// 0                                     31
// .... .... .... .... .... .... .... ....
//
// ILLL 0000 1111 2222 3333 4444 5555 6666

pub struct IndexList {
    ptr_or_list: *const Vec<usize>
}

impl IndexList {
    pub fn from_slice(values: &[usize]) -> IndexList {
        let x = Box::new(values.to_owned());
        let ptr = &*x as *const Vec<usize>;
        ::std::mem::forget(x);
        IndexList {
            ptr_or_list: ptr
        }
    }

    pub fn len(&self) -> usize {
        if self.is_immediate() {
            self.immediate_len()
        } else {
            self.get_vec().len()
        }

    }

    pub fn get(&self, index: usize) -> Option<usize> {
        if self.is_immediate() {
            if index > self.immediate_len() {
                None
            } else {
                Some(self.get_immediate_value(index))
            }
        } else {
            self.get_vec().get(index).map(|x| *x)
        }
    }

    fn get_vec(&self) -> &Vec<usize> {
        debug_assert!(!self.is_immediate());
        unsafe {
            &*self.ptr_or_list
        }
    }

    fn is_immediate(&self) -> bool {
        (self.ptr_or_list as usize) & 1 == 1
    }

    fn immediate_len(&self) -> usize {
        debug_assert!(self.is_immediate());
        ((self.ptr_or_list as usize) >> 1) & 0b111
    }

    fn set_immediate_value(&mut self, index: usize, value: usize) {
        debug_assert!(self.is_immediate());
        debug_assert!(value == value & 0b1111);
        let bit_offset = (index + 1) * 4;
        *self.ptr_as_bits_mut() &= !(0b1111 << bit_offset);
        *self.ptr_as_bits_mut() |= value << bit_offset;
    }

    fn get_immediate_value(&self, index: usize) -> usize {
        debug_assert!(self.is_immediate());
        let bit_offset = (index + 1) * 4;
        (self.ptr_as_bits() >> bit_offset) & 0b1111
    }

    fn ptr_as_bits_mut(&mut self) -> &mut usize {
        debug_assert!(self.is_immediate());
        unsafe {
            ::std::mem::transmute(&mut self.ptr_or_list)
        }
    }

    fn ptr_as_bits(&self) -> usize {
        debug_assert!(self.is_immediate());
        self.ptr_or_list as usize
    }

    fn can_be_immediate(values: &[usize]) -> bool {
        values.len() < 8 && values.iter().all(|val| val & 0b1111 == val)
    }
}

#[cfg(test)]
mod tests {
    use super::IndexList;

    #[test]
    fn len() {
        let list = IndexList::from_slice(&[]);
        assert_eq!(list.len(), 0);

        let list = IndexList::from_slice(&[1]);
        assert_eq!(list.len(), 1);

        let list = IndexList::from_slice(&[2]);
        assert_eq!(list.len(), 1);

        let list = IndexList::from_slice(&[1, 2]);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn get() {
        let list = IndexList::from_slice(&[1, 2, 1]);
        assert_eq!(list.len(), 3);
        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(1), Some(2));
        assert_eq!(list.get(2), Some(1));
        assert_eq!(list.get(3), None);
    }


}
