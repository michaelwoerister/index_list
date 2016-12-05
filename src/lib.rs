

pub struct IndexList {
    items: Vec<usize>
}

impl IndexList {
    pub fn new() -> IndexList {
        IndexList {
            items: vec![]
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn push(&mut self, item: usize) {
        self.items.push(item);
    }

    pub fn get(&self, index: usize) -> Option<usize> {
        self.items.get(index).map(|x| *x)
    }

    pub fn set(&mut self, index: usize, value: usize) {
        self.items[index] = value;
    }
}

impl std::ops::Index<usize> for IndexList {
    type Output: usize;
    fn index(&self, index: usize) -> &Self::Output {
        self.items.get(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::IndexList;

    #[test]
    fn len() {
        let list = IndexList::new();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn get() {
        let mut list = IndexList::new();
        list.push(1);
        list.push(2);
        list.push(1);
        assert_eq!(list.len(), 3);
        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(1), Some(2));
        assert_eq!(list.get(2), Some(1));
        assert_eq!(list.get(3), None);
    }

     #[test]
    fn set() {
        let mut list = IndexList::new();
        list.push(1);
        list.push(2);
        assert_eq!(list.len(), 2);
        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(1), Some(2));
        list.set(0, 3);
        assert_eq!(list.get(0), Some(3));
        assert_eq!(list.get(1), Some(2));
        list.set(1, 4);
        assert_eq!(list.get(0), Some(3));
        assert_eq!(list.get(1), Some(4));
    }
}
