

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

    pub fn push()
}


#[cfg(test)]
mod tests {
    use super::IndexList;

    #[test]
    fn len() {
        let list = IndexList::new();
        assert_eq!(list.len(), 0);
    }
}
