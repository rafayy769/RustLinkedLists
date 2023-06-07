// first.rs
// The first linked list from the tutorial series rust-unofficial.github.io/too-many-lists

// represents a linked list.
// pub represents that this can be used outside  of this module
// enum represents a sum type. From rust docs : "enums give you a way of saying a value is one of a possible set of values". More like a union in C.
pub enum List {
    Empty,
    Elem(i32, List),
}