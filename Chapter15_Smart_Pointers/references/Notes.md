## Chapter 15 – Smart Pointers: Treating Smart Pointers Like Regular References

### Overview
Implementing the Deref trait allows you to customize the behavior of the dereference operator *.

By implementing Deref in such a way that a smart pointer can be treated like a regular reference, you can write code that operates on references and use that code with smart pointers too.

#### Following The Reference to the Value
A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a value stored somewhere else.

```rs
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```
In the above example x is 5 and y is a reference to x. We can assert x is 5 but to make an assertion about the value in y we must dereference it.

#### Using Box<T> Like a Reference
We can rewrite the previous example to use Box<T> instead of a reference:

```rs
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

The main difference here is that here we set y to be an instance of a box pointing to a copied value of x rather than a reference pointing to the value of x.

In the last assertion, we can use the dereference operator to follow the box’s pointer in the same way that we did when y was a reference. 

#### Defining Our Own Smart Pointer
Let’s build a wrapper type similar to the Box<T> type provided by the standard library to experience how smart pointer types behave differently from references by default. Then, we’ll look at how to add the ability to use the dereference operator.

Well begin by copying how Box<T> works under the hood and our previous test.
```rs
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

Unfortunately we Error: Our MyBox<T> type can’t be dereferenced because we haven’t implemented that ability on our type. To enable dereferencing with the * operator, we implement the Deref trait.

#### Implementing the Deref Trait