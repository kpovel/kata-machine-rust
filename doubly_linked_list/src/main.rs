mod doubly_linked_list;
use doubly_linked_list::DoublyLinkedList;

fn main() {
    let mut doubly_linked_list = DoublyLinkedList::<u32>::new();
    doubly_linked_list.prepend(1);
    doubly_linked_list.prepend(2);
    dbg!(doubly_linked_list);
}
