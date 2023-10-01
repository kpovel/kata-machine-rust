use crate::BinaryNode;
use std::rc::Rc;

pub fn trees() -> (BinaryNode<u32>, BinaryNode<u32>) {
    let tree: BinaryNode<u32> = BinaryNode {
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

    let tree2: BinaryNode<u32> = BinaryNode {
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
                    value: 47,
                    right: None,
                    left: None,
                })),
                left: None,
            })),
        })),
    };

    return (tree, tree2);
}
