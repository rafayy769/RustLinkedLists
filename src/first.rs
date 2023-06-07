// first.rs
// The first linked list from the tutorial series rust-unofficial.github.io/too-many-lists

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
    next: List, // List is defined below.
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
}