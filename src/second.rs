// second.rs
// implements a better stack

pub struct List {
    head: Link,
}

// This is what we were using last time. Now it can be seen that this can simply be replaced an Option<Box<Node>>, which would represent that a link can either be None, or it can be a Some(Box<Node>).
// enum Link {
//     Empty,
//     More(Box<Node>),
// }

// type indicates a type alias. It is used to give a new name to an existing type. Here, we are giving a new name to the type Option<Box<Node>>. This is done because we will be using this type a lot, and it is easier to type Link than Option<Box<Node>>.
type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        // instead of head being Link::Empty, we init it with Optional None
        List {head: None}
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take()  // this represents the same concept as mem::replace(&mut self.head, None)
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // match self.head.take() {
        //     None => None,
        //     Some(node) => {
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        // }
        // The above can be rewritten via a map which does the same thing as 
        // match option { None => None, Some(x) => Some(y) }
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem               // no need to wrap it in Some since map does return an option, either a None, or a Some of the specified return value
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod test
{
    use super::List;

    #[test]
    // #test specifies that this is a test
    fn basics() {
        // create a mutable list using the static constructor defined in impl
        let mut list = List::new();

        // assert_eq! macro compares the two things you give it, and panics the program if they don't match.
        assert_eq!(list.pop(), None);

        // let's populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // testing normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}