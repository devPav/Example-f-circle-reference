use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}
impl List {
    fn trail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a start rc = {}", Rc::strong_count(&a));
    println!("a next element = {:?}", a.trail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc after b was created = {}", Rc::strong_count(&a));
    println!("b start rc = {:?}", Rc::strong_count(&b));
    println!("b next element = {:?}", b.trail());

    if let Some(link) = a.trail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b число rc после изменения a = {}", Rc::strong_count(&b));
    println!("a число rc после изменения a = {}", Rc::strong_count(&a));

    // Раскомментируйте следующую строку кода, и вы увидите, что у нас цикл;
    // он переполнит стек.
    // println!("a следующий элемент = {:?}", a.trail());
}
