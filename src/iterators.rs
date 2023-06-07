use super::ListNode;

pub struct Iter<'a, T> {
    //struct for mutable iteration over the
    //items of the LinkedList
    //exhausted: a boolean indicating whether or not the iterator is finished.
    exhausted: bool,
    //current_node, a reference to the current ListNode.
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
        //if the iterator is exhausted, end the iteration.
        if self.exhausted {
            return None;
        }
        let ref current_value = self.current_node.value;
        if let Some(next_node) = self.current_node.next.as_ref() {
            //if there is a next node, set the current_node to the next_node.
            self.current_node = &**next_node;
        } else {
            //otherwise, this is the end of the LinkedList. Set exhausted to true.
            self.exhausted = true;
        }
        Some(current_value)
    }
}

pub struct IntoIter<T> {
    //Iterator that consumes the LinkedList.
    current_node: Option<ListNode<T>>,
}

impl<T> IntoIter<T> {
    pub fn new(head_node: ListNode<T>) -> Self {
        IntoIter {
            current_node: Some(head_node),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_node.is_none() {
            return None;
        }
        //the unwrap is safe because otherwise the function already would have
        //returned.
        let mut unwrapped_current_node = self.current_node.take().unwrap();
        let next_node = unwrapped_current_node.next.take();

        // If the next_node is none, then the iteration is complete. Set the current_node to None.
        //The next call to the iterator will return None.
        if next_node.is_none() {
            self.current_node = None;
        } else {
            //Otherwise, set the current_node to a Some of the next_node.
            self.current_node = Some(*next_node.unwrap());
        }
        return Some(unwrapped_current_node.value);
    }
}
pub struct IterMut<'a, T> {
    current_node: Option<&'a mut ListNode<T>>,
}

impl<'a, T> IterMut<'a, T> {
    pub fn new(head: &'a mut ListNode<T>) -> Self {
        IterMut {
            current_node: Some(head),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_node.is_none() {
            return None;
        }

        //Almost the same logic as the IntoIter implementation.
        let unwrapped_current_node = self.current_node.take().unwrap();

        //Obtain a mutable reference to the next node.
        let next_node = unwrapped_current_node.next.as_mut();
        if next_node.is_none() {
            //If the next_node is None, set self.current_node to None.
            //The next call to the Iterator will terminate the Iterator.
            self.current_node = None;
        } else {
            self.current_node = Some(&mut **next_node.unwrap());
        };
        let ref mut current_node_value = unwrapped_current_node.value;
        return Some(current_node_value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_iter() {
        // test_iter
        let list_head = ListNode::from_iter(vec![1, 2, 3, 4]);
        let list_copy: Vec<_> = list_head.iter().collect();
        assert_eq!(list_copy, vec![&1, &2, &3, &4]);
        //Mutate the linkedList, and iterate again.
        let new_head = list_head.prepend(0);
        let list_copy2: Vec<_> = new_head.iter().collect();
        assert_eq!(list_copy2, vec![&0, &1, &2, &3, &4]);
    }

    #[test]
    fn test_into_iter() {
        //test_into_iter, which consumes the LinkedList
        let list_head = ListNode::from_iter(vec![1, 2, 3, 4]);
        let list_copy: Vec<_> = list_head.into_iter().collect();
        assert_eq!(list_copy, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_iter_mut() {
        //test_iter_mut, which returns a mutable reference to each item
        // in the LinkedList.
        let mut list_head = ListNode::from_iter(vec![1, 2, 3, 4]);
        for item in list_head.iter_mut() {
            //Double each value
            *item *= 2;
        }
        let collected_list: Vec<_> = list_head.iter().collect();
        //Check that each value was indeed doubled.
        assert_eq!(collected_list, vec![&2, &4, &6, &8]);
    }
}
