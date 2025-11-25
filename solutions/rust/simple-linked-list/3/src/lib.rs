struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<Box<Node<T>>>) -> Self {
        Self { value, next }
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut curr_head = &self.head;

        while let Some(node) = curr_head {
            count += 1;
            curr_head = &node.next;
        }

        count
    }

    pub fn push(&mut self, element: T) {
        let new_head = Box::new(Node::new(element, self.head.take()));

        self.head = Some(new_head)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            if let Some(next_node) = node.next {
                self.head = Some(next_node)
            }

            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reversed_list = Self::new();
        let mut curr_head = self.head;

        while let Some(node) = curr_head {
            reversed_list.push(node.value);
            curr_head = node.next
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