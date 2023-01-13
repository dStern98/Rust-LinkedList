use std::collections::VecDeque;

pub struct ListNode<T> {
    //Single Node in a Linked List. Contains a Value T,
    //and a next field that potentially points to the next Node.
    pub value: T,
    pub next: Option<Box<ListNode<T>>>,
}

//An Enum that Handles Errors in LinkedList Operations
#[derive(Debug)]
enum OperationsError {
    ListNotLongEnough,
    CannotPerformOnHead,
}

impl<T> IntoIterator for ListNode<T> {
    type Item = T;
    type IntoIter = ListNodeIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        let mut outbound_vec = VecDeque::new();
        let mut current_node = self;
        while let Some(next_node) = current_node.next {
            outbound_vec.push_back(current_node.value);
            current_node = *next_node;
        }
        outbound_vec.push_back(current_node.value);

        ListNodeIterator {
            items: outbound_vec,
        }
    }
}

impl<T> FromIterator<T> for ListNode<T> {
    fn from_iter<I>(iter: I) -> ListNode<T>
    where
        I: IntoIterator<Item = T>,
    {
        //Convert an Iterator into a Linked List. We have to treat the head of the list
        //differently than the rest of the insertions. We create the head, and then append the rest of
        //the items.
        let mut initial_head: Option<ListNode<T>> = None;
        for value in iter {
            if initial_head.is_none() {
                initial_head = Some(ListNode::new(value));
            } else {
                if let Some(ref mut head_node) = initial_head {
                    head_node.append(value);
                }
            }
        }
        //This will panic if the passed in iterator containers no elements,
        //but since under my implementation one cannot produce a ListNode<T> with no value,
        //the panic is acceptable under the circumstances
        return initial_head.unwrap();
    }
}

impl<T> ListNode<T> {
    fn iter_mut(&mut self) -> ListNodeMutableIterator<T> {
        //Iterate over the Linked List, inserting &mut T into the VecDeque
        let mut outbound_vec = VecDeque::new();
        let mut current_node = self;
        while let Some(ref mut next_node) = current_node.next {
            outbound_vec.push_back(&mut current_node.value);
            current_node = &mut *next_node;
        }
        outbound_vec.push_back(&mut current_node.value);

        ListNodeMutableIterator {
            items: outbound_vec,
        }
    }

    fn remove(&mut self, position_to_remove: usize) -> Result<T, OperationsError> {
        //This method cannot remove the head node, as that would require ownership to drop
        //if position_to_remove is 0, just return an Err
        if position_to_remove == 0 {
            return Err(OperationsError::CannotPerformOnHead);
        }

        //Otherwise, we can starting iterating through the Linked List
        let mut counter = 0 as usize;
        let mut current_node = self;
        //In this case, we interate until we are one before the node to remove
        while counter < position_to_remove - 1 {
            if let Some(ref mut next_node) = current_node.next {
                counter += 1;
                current_node = &mut **next_node;
            } else {
                return Err(OperationsError::ListNotLongEnough);
            }
        }

        //When we break we know that the next node is the one we need to remove
        if current_node.next.is_some() {
            //If the next node is some, we take ownership of the node to remove
            //We can safely unwrap this value because we already check if it was some
            let mut node_to_remove = current_node.next.take().unwrap();
            if node_to_remove.next.is_some() {
                //If there is a node after the node we are removing, then we have to set the
                //next field in the current_node to the next node in the node_to_remove
                let new_next_node = node_to_remove.next.take();
                current_node.next = new_next_node;
            }
            //If there is no next node after the node to remove, then we are
            //finished, as the node we are removing is the end of the list
            return Ok(node_to_remove.value);
        } else {
            //If the next node is None, there is no node to remove in the first place
            return Err(OperationsError::ListNotLongEnough);
        }
    }

    fn insert(&mut self, t: T, position_to_insert: usize) -> Result<(), OperationsError> {
        //Insert a New Node somewhere in the Linked List determined by the position_to_insert
        //param.
        let mut counter = 0 as usize;
        let mut current_node = self;

        //We iterate over the LinkedList so long as the current position (the counter) is less
        //than the desired insertion position. If we reach a point in the loop where the next node
        // is None, then we return an Err, as we know that we cannot successfully insert at the requested position.
        while counter < position_to_insert {
            if let Some(ref mut next_node) = current_node.next {
                counter += 1;
                current_node = &mut **next_node;
            } else {
                return Err(OperationsError::ListNotLongEnough);
            }
        }
        //When the while loop breaks, we know that we are at the position we want to insert into.
        //There are two options at this point:
        // Option 1. If the current_node.next is None, then we immediately set the current_node.next to
        // a new ListNode which we build, box, and and put inside an option.
        if current_node.next.is_none() {
            current_node.next = Some(Box::new(ListNode::new(t)));
        } else {
            // Option 2. If the current_node.next is not None, then first we need to take the current_nodes
            //next node out of its Option. We are going to reassign such that the new node we are in the process of building's
            //next node is what was just taken from the current_node.

            let current_nodes_next = current_node.next.take();
            let mut node_to_insert = ListNode::new(t);
            node_to_insert.next = current_nodes_next;

            //Finally, we set the current_nodes next to be the new ListNode we just built.
            current_node.next = Some(Box::new(node_to_insert));
        }

        Ok(())
    }

    fn iter(&self) -> ListNodeIteratorRef<T> {
        //Iterate over the Linked List, putting values into a VecDeque as a reference
        let mut outbound_vec = VecDeque::new();
        let mut current_node = self;
        while let Some(ref next_node) = current_node.next {
            outbound_vec.push_back(&current_node.value);
            current_node = &*next_node;
        }
        //When the loop breaks, there is one last node to push
        outbound_vec.push_back(&current_node.value);

        ListNodeIteratorRef {
            items: outbound_vec,
        }
    }
}

impl<T> ListNode<T> {
    fn new(t: T) -> Self {
        //Create a New ListNode
        ListNode {
            value: t,
            next: None,
        }
    }

    fn pop_front(self) -> Result<ListNode<T>, OperationsError> {
        //Easy O(1) removal of the first element in the list.
        //To do this, we need to consume the Head and return a new head.
        match self.next {
            None => return Err(OperationsError::ListNotLongEnough),
            Some(next_list_node) => return Ok(*next_list_node),
        }
    }

    fn prepend(self, t: T) -> ListNode<T> {
        //Consumes self, as we need to Box the old Node, creates a new head of the linked list
        //returns the new head, as the old one has been consumed. Note that this is a O(1) operation.
        let mut new_head = ListNode::new(t);
        new_head.next = Some(Box::new(self));
        new_head
    }

    fn append(&mut self, t: T) {
        //Takes a mut self, because we will need to potentially mutate the first struct passed in.
        //This is an O(N) operation as the entire list must be traversed to reach the end.
        let mut current_node = self;
        loop {
            match current_node.next {
                Some(ref mut box_list_node) => current_node = &mut **box_list_node,
                None => {
                    current_node.next = Some(Box::new(ListNode::new(t)));
                    return;
                }
            }
        }
    }

    fn len(&self) -> usize {
        //Returns the length of the Linked List
        return self.iter().count();
    }

    fn has_value(&self, t: T) -> bool
    where
        T: std::cmp::PartialEq,
    {
        //Check whether a value is present in the Linked List
        for value in self.iter() {
            if value == &t {
                return true;
            }
        }
        return false;
    }
}

pub struct ListNodeIterator<T> {
    //struct for the IntoIter trait.
    //Consumes the T when the struct is built.
    items: VecDeque<T>,
}

impl<T> Iterator for ListNodeIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.items.pop_front()
    }
}

struct ListNodeMutableIterator<'a, T> {
    //Stores a mutable reference to each T.
    items: VecDeque<&'a mut T>,
}

impl<'a, T> Iterator for ListNodeMutableIterator<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.items.pop_front()
    }
}

struct ListNodeIteratorRef<'a, T> {
    items: VecDeque<&'a T>,
}

impl<'a, T> Iterator for ListNodeIteratorRef<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.items.pop_front()
    }
}

impl<T> PartialEq for ListNode<T>
where
    T: std::cmp::PartialEq,
{
    fn eq(&self, other: &ListNode<T>) -> bool {
        //To determine if equal, just collect the two lists and compare them
        // as Vecs. Simple and effective.
        let v1: Vec<&T> = self.iter().collect();
        let v2: Vec<&T> = other.iter().collect();
        return v1 == v2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepend() {
        let original_head = ListNode::new(55);
        let new_head = original_head.prepend(54);
        let final_head = new_head.prepend(53);
        let mut final_vec = Vec::new();
        for value in final_head {
            final_vec.push(value);
        }
        assert_eq!(final_vec, vec![53, 54, 55]);
    }

    #[test]
    fn test_append() {
        let mut append_head = ListNode::new(1);
        append_head.append(2);
        append_head.append(3);
        append_head.append(4);
        append_head.append(5);

        let mut final_vec = Vec::new();
        for value in append_head {
            final_vec.push(value);
        }
        assert_eq!(final_vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_iter_mut() {
        let mut append_head = ListNode::new(1);
        append_head.append(2);
        append_head.append(3);
        append_head.append(4);
        append_head.append(5);

        for value in append_head.iter_mut() {
            *value *= 2;
        }

        let mut final_vec = Vec::new();
        for value in append_head {
            final_vec.push(value);
        }
        assert_eq!(final_vec, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_pop_front() {
        //This should fail and return an Err
        let list_node = ListNode::new(1);
        assert!(list_node.pop_front().is_err());
        //This should succeed
        let mut new_list_node = ListNode::new(1);
        new_list_node.append(2);
        assert_eq!(new_list_node.pop_front().unwrap().value, 2);
    }

    #[test]
    fn test_iter() {
        let mut append_head = ListNode::new(1);
        append_head.append(2);
        append_head.append(3);
        append_head.append(4);
        append_head.append(5);

        let final_vec: Vec<&i32> = append_head.iter().collect();

        assert_eq!(final_vec, vec![&1, &2, &3, &4, &5]);
    }

    #[test]
    fn test_check_has_value() {
        let mut append_head = ListNode::new(1);
        append_head.append(2);
        append_head.append(3);

        assert!(append_head.has_value(1));
        assert!(append_head.has_value(2));
        assert!(!append_head.has_value(4));
    }

    #[test]
    fn test_len() {
        let mut append_head = ListNode::new(1);
        append_head.append(2);
        append_head.append(3);
        append_head.append(4);
        append_head.append(5);

        assert_eq!(append_head.len(), 5);
    }

    #[test]
    fn test_from_iterator() {
        let vec_to_test = vec![1, 2, 3, 4, 5];
        let list_head = ListNode::from_iter(vec_to_test);
        let final_vec: Vec<&i32> = list_head.iter().collect();
        assert_eq!(final_vec, vec![&1, &2, &3, &4, &5]);
    }

    #[test]
    fn test_partial_eq() {
        let linked_list1 = ListNode::from_iter(vec![1, 2, 3, 4, 5]);
        let linked_list2 = ListNode::from_iter(vec![2, 3, 4, 5, 6]);
        let linked_list3 = ListNode::from_iter(vec![1, 2, 3, 4, 5]);
        assert!(linked_list1 == linked_list3);
        assert!(linked_list1 != linked_list2);
    }

    #[test]
    fn test_insert() {
        //First, try to insert past the end of the list. This should return an err.
        let mut linked_list1 = ListNode::from_iter(vec![1, 2, 3, 4, 5]);
        let response1 = linked_list1.insert(55, 5);
        assert!(response1.is_err());

        //Now try to insert at the 3rd position, this should work fine.
        let mut linked_list2 = ListNode::from_iter(vec![1, 2, 3, 4, 5]);
        let response2 = linked_list2.insert(55, 3);
        assert!(response2.is_ok());
        //Also check that the contents of the insertion are ok
        let captured_vec: Vec<&i32> = linked_list2.iter().collect();
        assert_eq!(captured_vec, vec![&1, &2, &3, &4, &55, &5]);
    }

    #[test]
    fn test_insert2() {
        //A second set of tests on insert for good measure.
        // Try to insert at position 2
        let mut linked_list1 = ListNode::from_iter(vec![1, 2, 3, 4, 5]);
        let _res1 = linked_list1.insert(55, 1);
        let captured_vec: Vec<&i32> = linked_list1.iter().collect();
        assert_eq!(captured_vec, vec![&1, &2, &55, &3, &4, &5]);

        //Now try to insert at the 3rd position, this should work fine.
        let mut linked_list2 = ListNode::from_iter(vec![1, 2, 3, 4, 5]);
        let _res2 = linked_list2.insert(55, 4);
        //Also check that the contents of the insertion are ok
        let captured_vec: Vec<&i32> = linked_list2.iter().collect();
        assert_eq!(captured_vec, vec![&1, &2, &3, &4, &5, &55]);
    }

    #[test]
    fn test_remove_node() {
        //Try to remove the third item
        let mut linked_list1 = ListNode::from_iter(vec![1, 2, 3, 4, 5]);
        let res1 = linked_list1.remove(2);
        let captured_vec: Vec<&i32> = linked_list1.iter().collect();
        assert_eq!(captured_vec, vec![&1, &2, &4, &5]);
        assert_eq!(res1.unwrap(), 3);

        //Now try to remove the new 3rd item (which is the end of the list.)
        let res2 = linked_list1.remove(3);
        let captured_vec: Vec<&i32> = linked_list1.iter().collect();
        assert_eq!(captured_vec, vec![&1, &2, &4]);
        assert_eq!(res2.unwrap(), 5);

        //Finally, try to delete past the end of the list
        //The list should have stayed the same, and the remove operation should have returned
        // an error
        let res3 = linked_list1.remove(3);
        let captured_vec: Vec<&i32> = linked_list1.iter().collect();
        assert_eq!(captured_vec, vec![&1, &2, &4]);
        assert!(res3.is_err());
    }
}
