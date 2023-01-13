# About

This is a library crate that implements a Linked List in Rust.
The core type is:

```
pub struct ListNode<T> {
    pub value: T,
    pub next: Option<Box<ListNode<T>>>,
}
```

It is generic over type T, with no trait bounds on the type itself.
To create a new `ListNode<T>`, use the `fn new(t: T) -> Self` associated function. Any time a method returns a `Result<T, E>`, the Error is of type:

```
enum OperationsError {
    ListNotLongEnough,
    CannotPerformOnHead,
}
```

## Basic API

All of the basic required methods for building and changing a Linked List are included in the API:

---

```
fn insert(&mut self, t: T, position_to_insert: usize) -> Result<(),OperationsError>
```

Tries to insert a new `ListNode<T>` at `position_to_insert` with value `t`.
Returns an Opertions Error if the list is not long enough.

---

```
fn remove(&mut self, position_to_remove: usize) -> Result<T, OperationsError>
fn pop_front(self) -> Result<ListNode<T>, OperationsError>
```

`remove` attempts to remove the specified node from the List. Note that this method
cannot remove the head of the list, as that would require owning self. When successful, the value `t` for the deleted node is returned.
`pop_front` acts like `remove`, but consumes self, and returns a new head for the list.

---

```
fn prepend(self, t: T) -> ListNode<T>
fn append(&mut self, t: T)
```

The method `prepend` is O(1), consumes self and returns a new head for the list.
`append` is O(N) but only needs a mutable reference to self.

---

```
fn len(&self) -> usize
fn has_value(&self, t: T) -> bool
```

Self-explanatory. `len` returns the length of the Linked List.
`has_value` checks whether a value t is in the list.

---

## Iterators API

The trait `IntoIterator` is implemented for `ListNode<T>`.
As such the list can be iterated in a for...in loop.
In addition, the methods `iter_mut` & `iter` are implemented, allowing
iteration over `&mut T` and `&T` respectively. Finally, the trait `FromIterator`
is implemented, allowing a new Linked List to be built from an iterator.

---

## Tests

Unit tests have been written for all methods listed above.
