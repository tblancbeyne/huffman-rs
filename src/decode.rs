use crate::Node;

pub fn decode_tree(encoded_tree: &[bool]) -> Node {
    decode_tree_aux(encoded_tree).0
}

fn decode_tree_aux(encoded_tree: &[bool]) -> (Node, &[bool]){
    let mut tree = Node {
        symbol: None,
        frequency: 0,
        left: None,
        right: None,
    };

    let encoded_tree = if ! encoded_tree[0]  {
        let (left, encoded_tree_left) = decode_tree_aux(&encoded_tree[1..encoded_tree.len()]);
        tree.left = Some(Box::new(left));
        let (right, encoded_tree_right) = decode_tree_aux(encoded_tree_left);
        tree.right = Some(Box::new(right));
        encoded_tree_right
    } else {
        &encoded_tree[1..encoded_tree.len()]
    };

    (tree, encoded_tree)
}
