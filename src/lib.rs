use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct MyStruct {
    link: RefCell<Option<Rc<MyStruct>>>,
}
impl MyStruct {
    fn new() -> MyStruct {
        MyStruct { link: RefCell::new(None) }
    }
}
pub fn get_circle_references() {
    let first = Rc::new(MyStruct::new());
    let second = Rc::new(MyStruct{
            link: RefCell::new(Some(Rc::clone(&first))),
        });

    *first.link.borrow_mut() = Some(Rc::clone(&second)); 

    // println!("{:?}", second);
}
