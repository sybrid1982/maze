pub struct PathfindingNode<'a, T> {
    node: &'a T,
    g: usize,
    h: usize,
    previous: &'a T
}

impl <'a, T> PathfindingNode<'a, T> {
    pub fn new(
        node: &'a T,
        g: usize,
        h: usize,
        previous: &'a T
    ) -> Self {
        PathfindingNode {
            node, g, h, previous
        }
    }
}