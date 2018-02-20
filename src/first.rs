use std::mem;

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
pub struct List {
    head: Link
}

impl List {
	pub fn new() -> Self {
	    List { head: Link::Empty }
	}

	pub fn push(&mut self, elem: i32) {
	    let new_node = Box::new(Node {
	    	elem: elem, 
	    	next: mem::replace(&mut self.head, Link::Empty)
	    });

	    self.head = Link::More(new_node);
	}

	pub fn pop(&mut self) -> Option<i32> {
		match mem::replace(&mut self.head, Link::Empty) {
		    Link::More(boxed_node) => {
		    	let node = *boxed_node;
		    	self.head = node.next;
		    	Some(node.elem)
		    },
		    Link::Empty => None,
		}
	}
}

impl Drop for List {
    fn drop(&mut self) {
    	// cur_link is the runner of the iteration, currently we replaced self.head with empty
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        // iterate until it doesnt matter, AKA empty
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // we then replace the boxed_node.next with empty and assign it as the current runner
            // when after this loop, boxed_node goes out of scope, thus
        }
    }
}

#[cfg(test)]
mod test {
	use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // check pop on empty list
        assert_eq!(list.pop(), None);

        // populate it
        list.push(1);
        list.push(2);
        list.push(3);

        // check for normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
		assert_eq!(list.pop(), None);


        // then populate again
        list.push(1);
        list.push(2);

        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
    }
}
