#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>
}

// iterator implementations

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

// itermut

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

//into iter

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}


impl<T> List<T> {
	pub fn new() -> Self {
	    List { head: None }
	}

	pub fn push(&mut self, elem: T) {
	    let new_node = Box::new(Node {
	    	elem: elem, 
	    	next: self.head.take(),
	    });

	    self.head = Some(new_node);
	}

	pub fn pop(&mut self) -> Option<T> {
		self.head.take().map(|boxed_node| {
            let node = *boxed_node;
            self.head = node.next;
            node.elem
        })
	}

    pub fn peek(&self) -> Option<&T>{
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }


}


impl<T> Drop for List<T> {
    fn drop(&mut self) {
    	// cur_link is the runner of the iteration, currently we replaced self.head with empty
        let mut cur_link = self.head.take();

        // iterate until it doesnt matter, AKA empty
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1);
        assert_eq!(list.peek(), Some(&1));

        list.push(2);
        assert_eq!(list.peek_mut(), Some(&mut 2));

        list.push(3);
        assert_eq!(list.peek(), Some(&3));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        for i in 0..3 {
            list.push(i);
        }

        let mut iter = list.into_iter();
        for i in 2..-1 {
            assert_eq!(iter.next(), Some(i));
        }
    }


    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(0);
        list.push(1);
        list.push(2);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&0));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(0);
        list.push(1);
        list.push(2);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 0));
    }
}
