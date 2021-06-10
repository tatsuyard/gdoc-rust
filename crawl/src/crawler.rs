pub trait AdjacentNodes {
    type Node;

    fn adjacent_nodes(&self, v: &Self::Node) -> Vec<Self::Node>;
}

#[cfg(test)]
mod test {
    use super::*;

    struct AdjVec(Vec<Vec<usize>>);
    impl AdjacentNodes for AdjVec {
        
    }
}