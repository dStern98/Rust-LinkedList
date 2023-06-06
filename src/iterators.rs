use super::ListNode;

pub struct Iter<'a, T> {
    exhausted: bool,
    current_node: &'a ListNode<T>,
}

impl<'a, T> Iter<'a, T> {
    pub fn new(head: &'a ListNode<T>) -> Self {
        Iter {
            exhausted: false,
            current_node: head,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }
        let ref current_value = self.current_node.value;
        if let Some(next_node) = self.current_node.next.as_ref() {
            self.current_node = &**next_node;
        } else {
            self.exhausted = true;
        }
        Some(current_value)
    }
}

// pub struct IterMutTest<'a, T> {
//     pub exhausted: bool,
//     pub current_node: &'a mut ListNode<T>,
// }

// impl<'a, T> Iterator for IterMut<'a, T> {
//     type Item = &'a mut T;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.exhausted {
//             return None;
//         }
//         let ref mut current_value = self.current_node.value;
//         if let Some(next_node) = self.current_node.next.as_mut() {
//             self.current_node = &mut **next_node;
//         } else {
//             self.exhausted = true;
//         }
//         Some(current_value)
//     }
// }

// impl<'a, T> IterMutTest<'a, T> {
//     pub fn next<'b>(&'b mut self) -> Option<&'a mut T>
//     where
//         'b: 'a,
//     {
//         if self.exhausted {
//             return None;
//         }
//         let ref mut current_value = self.current_node.value;
//         if let Some(next_node) = self.current_node.next.as_mut() {
//             self.current_node = &mut **next_node;
//         } else {
//             self.exhausted = true;
//         }
//         Some(current_value)
//     }
// }
