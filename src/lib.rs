
// Bits

// 0                                     31
// .... .... .... .... .... .... .... ....
//
// ILLL 0000 1111 2222 3333 4444 5555 6666

// 0                                       31                                     63
// . .... .... .... .... .... .... .... .... .... .... .... .... .... .... .... ...
//
// I LLLL 0000 1111 2222 3333 4444 5555 6666 7777 8888 9999 1010 1111 1212 1313

#[cfg(target_pointer_width = "32")]
const MAX_LENGTH: usize = 7;
#[cfg(target_pointer_width = "32")]
const LENGTH_BITS: usize = 3;

#[cfg(target_pointer_width = "64")]
const MAX_LENGTH: usize = 13;
#[cfg(target_pointer_width = "64")]
const LENGTH_BITS: usize = 4;


pub struct IndexList {
    ptr_or_list: *const Vec<usize>
}

impl IndexList {
    pub fn from_slice(values: &[usize]) -> IndexList {
        if IndexList::can_be_immediate(values) {
            let mut index_list = IndexList {
                ptr_or_list: (0b1 | (values.len() << 1)) as *const Vec<usize>
            };

            for (index, &value) in values.iter().enumerate() {
                index_list.set_immediate_value(index, value)
            }

            index_list
        } else {
            let ptr = Box::into_raw(Box::new(values.to_owned()));
            IndexList {
                ptr_or_list: ptr
            }
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
            if index >= self.immediate_len() {
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

    pub fn is_immediate(&self) -> bool {
        (self.ptr_or_list as usize) & 1 == 1
    }

    fn immediate_len(&self) -> usize {
        debug_assert!(self.is_immediate());
        ((self.ptr_or_list as usize) >> 1) & ((1 << LENGTH_BITS)-1)
    }

    fn set_immediate_value(&mut self, index: usize, value: usize) {
        debug_assert!(self.is_immediate());
        debug_assert!(value == value & 0b1111);
        debug_assert!(index < self.immediate_len());
        let bit_offset = index * 4 + LENGTH_BITS + 1;
        *self.ptr_as_bits_mut() &= !(0b1111 << bit_offset);
        *self.ptr_as_bits_mut() |= value << bit_offset;
    }

    fn get_immediate_value(&self, index: usize) -> usize {
        debug_assert!(self.is_immediate());
        let bit_offset = index * 4 + LENGTH_BITS + 1;
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
        values.len() <= MAX_LENGTH && values.iter().all(|&val| (val & 0b1111) == val)
    }
}

impl Drop for IndexList {
    fn drop(&mut self) {
        if !self.is_immediate() {
            unsafe {
                // Drop the boxed vec
                let _ = Box::from_raw(self.ptr_or_list as *mut Vec<usize>);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IndexList;
    use super::MAX_LENGTH;

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

    #[test]
    fn non_immediate_because_of_len() {
        for len in 0 .. 20 {
            let reference: Vec<usize> = (0..len).map(|val| val % 8).collect();
            let index_list = IndexList::from_slice(&reference);
            assert_eq!(index_list.is_immediate(), len <= MAX_LENGTH);

            for (index, &val) in reference.iter().enumerate() {
                assert_eq!(val, index_list.get(index).unwrap());
            }
        }
    }

    #[test]
    fn non_immediate_because_of_val() {
        for len in 1 .. 20 {
            let reference: Vec<usize> = (110..110+len).collect();
            let index_list = IndexList::from_slice(&reference);
            assert!(!index_list.is_immediate());

            for (index, &val) in reference.iter().enumerate() {
                assert_eq!(val, index_list.get(index).unwrap());
            }
        }
    }

}
