// first.rs
// The first linked list from the tutorial series rust-unofficial.github.io/too-many-lists

use std::mem;

pub struct List {
    head: Link,
}

/// This represents a linked list link.
/// `pub` represents that this can be used outside  of this module
/// `enum` represents a sum type. From rust docs : "enums give you a way of saying a value is one of a possible set of values". More like a union in C, or an enum in C/C++.
/// `Box<T>`, casually referred to as a 'box', provides the simplest form of heap allocation in Rust. Boxes provide ownership for this allocation, and drop their contents when they go out of scope.
enum Link {
    Empty,            // Empty is a variant of List, i.e. a list can have a value of Empty
    More(Box<Node>),  // or it can have a value of More, which is a box containing a Node
}

// struct represents a node in the linked list. structs in rust are similar to structs in C/C++.
struct Node {
    elem: i32,  // i32 is a 32 bit integer
    next: Link, // List is defined below.
}


// So our linked list is a struct with a single field, head, which is a Link. What is a link? A link is an enum with two variants: Empty and More. Empty is a variant that represents the end of the list. More is a variant that represents a node in the list. A node is a struct with two fields: elem, which is the value of the node, and next, which is the next node in the list.

// impl is a keyword that allows us to implement methods on a struct. We can implement methods on a struct that we define, or on a struct that someone else defined. In this case, we are implementing methods on the List struct that we defined above.
impl List {
    // pub is a keyword that allows us to make this method public. If we didn't have pub, this method would be private to this module.
    // this method returns Self, which is a type alias for List. Self is a special type in rust that represents the type that we are implementing methods on. In this case, we are implementing methods on List, so Self is an alias for List.
    pub fn new() -> Self {

        // List { head: Link::Empty } is a struct literal. It creates a new List with a head of Link::Empty. Link::Empty is a variant of the Link enum that we defined above. {} is used to initialize a struct much like initalizer lists in C/C++.
        List { head: Link::Empty }

        // note we didn't return anything. In rust, the last expression in a function is the return value. So we don't need to explicitly return anything.
    }

    /// ### QUICK PRIMER ON RUST OWNERSHIP (I can be very wrong but this is what I have gathered so far)
    /// non-static methods in rust look like follows:
    /// ```rust
    /// fn foo(self, arg1: Type1, arg2: Type2) -> ReturnType {
    ///    // body
    /// }
    /// ```
    /// `self` here can take three types:
    /// 1. `self` - this is a value of the type that we are implementing methods on. This is the most common type of `self` that you will see. This is similar to `this` in C++, and represents the concept of pass by value.
    /// 2. `&self` - this is a reference to the type that we are implementing methods on. This is similar to `this` in C++, and represents the concept of pass by const reference.
    /// 3. `&mut self` - this is a mutable reference to the type that we are implementing methods on. This is similar to `this` in C++, and represents the concept of pass by reference.
    /// 
    /// push is a method that takes a mutable reference to self, and an i32, and returns nothing. (Why mutable ref to self? Because we are modifying the list, so we need a mutable reference to it (and not a copy or a shared reference))
    pub fn push(&mut self, elem: i32) {

        // create a new node. the value is the provided elem, while the next is the current head.
        let new_node = Box::new(Node {
            elem: elem,
            // next: self.head, // this is wrong, because we are trying to move self.head into next, but self.head is not owned by us, we just 'borrowed' it, and the new_node will remain partially initialized once the borrowed contents are returned back. What we can do is replace the contents with something else.
            next: mem::replace(&mut self.head, Link::Empty), // yup we just pulled a devious lick. replaced the borrowed self.head with empty, and then moved the borrowed head to the next of the next node.
        });
        
        // set the head of the linked list to the new node.
        self.head = Link::More(new_node)
    }

    // the pop method, that can either return a value from the list, or can return an empty value.
    pub fn pop(&mut self) -> Option<i32> {
        // pattern matching for self.head. self.head is an enum Link that can have any of the following values, which we can handle individually.

        let result; // what we will return.

        // by default match moves its content into the new branch, but we can't do this since we have a mutable reference here. we can get a shared reference tho, but then we won't be able to modify the head. hence, we should replace and return the self.head since we will be replacing the self.head.
        match mem::replace(&mut self.head, Link::Empty)
        {
            Link::Empty => {
                result = None;
            }
            Link::More(node) => {
                result = Some(node.elem);
                self.head = node.next;
            }
        };

        // things ending with ! indicate macros. this macro indicates that this function hasnt been implemented yet fully, and crashes the program in a controlled manner when we reach here.
        // unimplemented!()

        result

        // A more concise way to write the above function would be:
        // match mem::replace(&mut self.head, Link::Empty) {
        //     Link::Empty => None,
        //     Link::More(node) => {
        //         result = Some(node.elem);
        //         self.head = node.next
        //     }
        // }
        // but let's stick with verbosity for now
    }
}