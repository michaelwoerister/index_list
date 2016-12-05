
// Bits

// 0                                     31
// .... .... .... .... .... .... .... ....
//
// ILLL 0000 1111 2222 3333 4444 5555 6666

pub struct IndexList {
    // items: Vec<usize>
    ptr_or_list: *mut Vec<usize>
}

impl IndexList {
    pub fn from_slice(values: &[usize]) -> IndexList {
        let mut x = Box::new(values.to_owned());
        let ptr = &mut *x as *mut Vec<usize>;
        ::std::mem::forget(x);
        IndexList {
            ptr_or_list: ptr
        }
    }

    pub fn len(&self) -> usize {
        self.get_vec().len()
    }

    pub fn get(&self, index: usize) -> Option<usize> {
        self.get_vec().get(index).map(|x| *x)
    }

    fn get_vec(&self) -> &Vec<usize> {
        assert!((self.ptr_or_list as usize) & 1 == 0);
        unsafe {
            &*self.ptr_or_list
        }
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
