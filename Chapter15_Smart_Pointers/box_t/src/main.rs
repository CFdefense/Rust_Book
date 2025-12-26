/* A cons list entity
Gives error recursive type `List` has infinite size
enum List {
    Cons(i32, List),
    Nil,
}
*/

fn main() {
    // An example of creating a box for an i32 value
    let b = Box::new(5);
    println!("b = {b}");

    // Define a cons list
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}