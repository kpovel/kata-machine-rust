use std::rc::Rc;

struct BinaryNode<T> {
    value: T,
    left: Option<Rc<BinaryNode<T>>>,
    right: Option<Rc<BinaryNode<T>>>,
}

fn walk<T: Clone>(curr: &Option<Rc<BinaryNode<T>>>, path: &mut Vec<T>) -> Vec<T> {
    match curr {
        Some(curr) => {
            path.push(curr.value.to_owned());

            walk(&curr.left, path);
            walk(&curr.right, path);

            return path.to_vec();
        }
        None => path.to_vec(),
    }
}

fn pre_ord_search<T: Clone>(head: &Option<Rc<BinaryNode<T>>>) -> Vec<T> {
    return walk(head, &mut vec![]);
}

fn main() {
    let tree = BinaryNode {
        value: 20,
        right: Some(Rc::new(BinaryNode {
            value: 50,
            right: Some(Rc::new(BinaryNode {
                value: 100,
                right: None,
                left: None,
            })),
            left: Some(Rc::new(BinaryNode {
                value: 30,
                right: Some(Rc::new(BinaryNode {
                    value: 45,
                    right: None,
                    left: None,
                })),
                left: Some(Rc::new(BinaryNode {
                    value: 29,
                    right: None,
                    left: None,
                })),
            })),
        })),
        left: Some(Rc::new(BinaryNode {
            value: 10,
            right: Some(Rc::new(BinaryNode {
                value: 15,
                right: None,
                left: None,
            })),
            left: Some(Rc::new(BinaryNode {
                value: 5,
                right: Some(Rc::new(BinaryNode {
                    value: 7,
                    right: None,
                    left: None,
                })),
                left: None,
            })),
        })),
    };

    let path = pre_ord_search(&Some(Rc::new(tree)));
    dbg!(&path);
}
