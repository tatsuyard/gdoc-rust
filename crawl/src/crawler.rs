pub trait AdjacentNodes {
    type Node;

    fn adjacent_nodes(&self, v: &Self::Node) -> Vec<Self::Node>;
}