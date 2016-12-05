

struct IndexList {
    items: Vec<usize>
}

impl IndexList {
    pub fn new() -> IndexList {
        IndexList {
            items: vec![]
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
