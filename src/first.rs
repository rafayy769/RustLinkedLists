// first.rs
// The first linked list from the tutorial series rust-unofficial.github.io/too-many-lists

// struct represents a node in the linked list. structs in rust are similar to structs in C/C++.
struct Node {
    elem: i32,  // i32 is a 32 bit integer
    next: List, // List is defined below.
}


/// This represents a linked list.
/// `pub` represents that this can be used outside  of this module
/// `enum` represents a sum type. From rust docs : "enums give you a way of saying a value is one of a possible set of values". More like a union in C, or an enum in C/C++.
/// `Box<T>`, casually referred to as a 'box', provides the simplest form of heap allocation in Rust. Boxes provide ownership for this allocation, and drop their contents when they go out of scope.
pub enum List {
    Empty,            // Empty is a variant of List, i.e. a list can have a value of Empty
    More(Box<Node>),  // or it can have a value of More, which is a box containing a Node
}