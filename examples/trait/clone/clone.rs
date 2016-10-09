// A unit struct without members
#[derive(Debug, Clone, Copy)]
struct Nil;

// A tuple struct with members that implements the `Clone` trait
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    // Instantiate `Nil`
    let nil = Nil;
    // Copy `Nil`, there are no members to move
    let copied_nil = nil;

    // Both `Nil`s can be used independently
    println!("nil: {:?}", nil);
    println!("copied_nil: {:?}", copied_nil);

    // Instantiate `Pair`
    let pair = Pair(Box::new(1), Box::new(2));
    println!("pair: {:?}", pair);

    // Bind `pair` to `moved_pair` (move semantics)
    let moved_pair = pair;
    println!("moved_pair: {:?}", moved_pair);

    // Error! `use of a moved value: pair`
    //println!("pair: {:?}", pair);
    // TODO ^ Try uncommenting this line

    // Clone `moved_pair`, and bind the clone to `cloned_pair`
    let cloned_pair = moved_pair.clone();

    // Drop `moved_pair` and its members
    drop(moved_pair);

    // Error! `use of a moved value: moved_pair`
    //println!("moved_pair: {:?}", moved_pair);
    // TODO ^ Try uncommenting this line

    // The result from .clone() can still be used!
    println!("clone: {:?}", cloned_pair);
}
