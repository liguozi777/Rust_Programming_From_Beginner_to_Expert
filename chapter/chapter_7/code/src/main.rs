use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;
// 智能指针
struct Custom {
    data: String,
}
impl Drop for Custom {
    fn drop(&mut self) {
        println!("Dropping Custom with data `{}`!", self.data);
    }
}
fn main() {
    // 7-1
    let x: Box<i32> = Box::new(5);
    let y: Box<i32> = x;
    // println!("x:{}", x);
    println!("y:{}", y);
    // 7-2
    let x: Box<i32> = Box::new(5);
    let y: i32 = *x;
    println!("x:{}", x);
    println!("y:{}", y);
    // 7-3
    let x: i32 = 5;
    let y: &i32 = &x;

    assert_eq!(5, *y);
    println!("pointer:{:p}", y);
    // 7-4
    let x: i32 = 5;
    let y: Box<i32> = Box::new(x);
    assert_eq!(5, *y);
    println!("pointer:{:p}", y);
    // 7-5
    let str1 = Custom {
        data: String::from("hello world!"),
    };
    let str2 = Custom {
        data: String::from("hello rust!"),
    };
    println!("Custom created.");
    println!("str1: {}", str1.data);
    println!("str2: {}", str2.data);
    // 7-6
    let x = Rc::new(5);
    println!(
        "{:p},count after constructing x:{}",
        x,
        Rc::strong_count(&x)
    );
    let y = x.clone();
    println!(
        "{:p},count after constructing y:{}",
        y,
        Rc::strong_count(&x)
    );
    {
        let z = Rc::clone(&x);
        println!(
            "{:p},count after constructing z:{}",
            z,
            Rc::strong_count(&x)
        );
    }
    println!("count after destroying z:{}", Rc::strong_count(&x));
    // 7-7
    let v: RefCell<Vec<i32>> = RefCell::new(vec![1, 2, 3, 4]);
    println!("{:?}", v.borrow());
    v.borrow_mut().push(5);
    println!("{:?}", v.borrow());
    // 7-8
    let v: RefCell<Vec<i32>> = RefCell::new(vec![1, 2, 3, 4]);
    let v_borrow_1: Ref<Vec<i32>> = v.borrow();
    println!("{:?}", v_borrow_1);
    let v_borrow_2: Ref<Vec<i32>> = v.borrow();
    println!("{:?}", v_borrow_2);
    // 7-9
    // let v: RefCell<Vec<i32>> = RefCell::new(vec![1, 2, 3, 4]);
    // let mut v_borrow_mut_1: RefMut<Vec<i32>> = v.borrow_mut();
    // v_borrow_mut_1.push(5);
    // println!("{:?}", v_borrow_mut_1);
    // let mut v_borrow_mut_2: RefMut<Vec<i32>> = v.borrow_mut();
    // v_borrow_mut_2.push(6);
    // println!("{:?}", v_borrow_mut_2);

    // 7-10
    let v: RefCell<Vec<i32>> = RefCell::new(vec![1, 2, 3, 4]);
    let v_borrow: Ref<Vec<i32>> = v.borrow();
    println!("{:?}", v_borrow);

    let mut v_borrow_mut: RefMut<Vec<i32>> = v.borrow_mut();
    v_borrow_mut.push(5);
    println!("{:?}", v_borrow_mut);
}
