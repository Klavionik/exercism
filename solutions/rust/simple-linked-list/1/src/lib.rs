struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self { value, next: None }
    }

    fn with_node(self, node: Node<T>) -> Self {
        Self { value: self.value, next: Some(Box::new(node)) }
    }
}


pub struct SimpleLinkedList<T> {
    head: Option<Node<T>>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.head, None)
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        
        if let Some(node) = &self.head {
           let mut stack = vec![node];
            
            while let Some(next_node) = stack.pop() {
                count += 1;
                
                if let Some(node) = &next_node.next {
                    stack.push(node)
                }
            }
        }
        
        count
    }

    pub fn push(&mut self, element: T) {
        let mut new_head = Node::new(element);
        let curr_head = self.head.take();
        
        if let Some(node) = curr_head {
            new_head = new_head.with_node(node)
        }
        
        self.head = Some(new_head)
    }

    pub fn pop(&mut self) -> Option<T> {
        let curr_head = self.head.take();
        
        match curr_head {
            None => None,
            Some(node) => {
                if let Some(next_node) = node.next {
                    self.head = Some(*next_node);
                }
                
                Some(node.value)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(node) => {
                Some(&node.value)
            }
        }
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut reversed_list = Self::new();
        
        while let Some(value) = self.pop() {
            reversed_list.push(value)
        }
        
        reversed_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut linked_list = Self::new();
        
        for item in iter {
            linked_list.push(item)
        }
        
        linked_list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vector = vec![];
        
        while let Some(value) = linked_list.pop() {
            vector.push(value)
        }
        
        vector.reverse();
        vector
    }
}