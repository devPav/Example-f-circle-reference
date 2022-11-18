use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct MyStruct {
    link: Option<RefCell<Rc<MyStruct>>>,
}
impl MyStruct {
    fn new() -> MyStruct {
        let mock_struct = MyStruct {link: None};
        MyStruct { link: Some(RefCell::new(Rc::new(mock_struct))) }
    }
}
pub fn get_circle_references() {
    let first = Rc::new(MyStruct::new());
    let second = Rc::new(MyStruct{
            link: Some(RefCell::new(Rc::clone(&first))),
        });
if let Some(value) = &first.link {
    *value.borrow_mut() = Rc::clone(&second);
}
    println!("rc of first is {}", Rc::strong_count(&first));
    println!("rc of second is {}", Rc::strong_count(&second));
    //println!("{:?}", second);
}
