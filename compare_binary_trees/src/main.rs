mod trees;

use std::rc::Rc;

#[derive(PartialEq)]
pub struct BinaryNode<T> {
    value: T,
    left: Option<Rc<BinaryNode<T>>>,
    right: Option<Rc<BinaryNode<T>>>,
}

fn compare<T: PartialEq>(a: &Option<Rc<BinaryNode<T>>>, b: &Option<Rc<BinaryNode<T>>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a_rc), Some(b_rc)) => {
            let a_value = a_rc.as_ref();
            let b_value = b_rc.as_ref();

            if a_value.value != b_value.value {
                return false;
            }

            return compare(&a_value.left, &b_value.left)
                && compare(&a_value.right, &b_value.right);
        }
        _ => false,
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use crate::{compare, trees::trees};

    #[test]
    fn compare_tree() {
        assert!(compare(
            &Some(Rc::new(trees().0)),
            &Some(Rc::new(trees().0))
        ));
        assert!(!compare(
            &Some(Rc::new(trees().0)),
            &Some(Rc::new(trees().1))
        ));
    }
}
