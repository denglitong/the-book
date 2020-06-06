// Interior mutability is a design patter that
// allows you to mutate data even when there are immutable references.
// to mutate data, the pattern uses `unsafe` code inside a data structure
// to bend Rust's usual rules that govern mutation and borrowing.

// we can use types that use the interior mutability pattern when
// we can ensure that the borrowing rules will be followed at runtime,
// even thought the compiler can't guarantee that.

// the `unsafe` code involved is then wrapped in a safe API, and the outer type is still immutable.

// RefCell<T> follows the interior mutability pattern.
// RefCell<T> takes ownership of data it holds

// RefCell<T> vs Box<T>
// Box<T> the borrowing rule's invariants are enforced at compile time, if check fail you'll get a compiler error
// RefCell<T> the invariants are enforced at runtime, if you break rules your program will panic and exit

// Rust 默认的编译期检查有些分析是做不了的，如果不给于编译器通过则 Rust 的检查规则将变得固执己见并给开发者带来了不便；
// 而如何给予编译通过，则又不能保证安全性。这个时候就出现了 RefCell<T>类型，它用于那些编译器分析不了的场景，
// 但是开发者可以确保它是遵循 ownership 的约束的。
// 和 Rc<T>一样，RefCell<T>只能在单线程中安全使用，如果在多线程上下文中使用会得到一个编译期错误

// Rc<T> vs Box<T>, RefCell<T>
// Rc<T>使得你可以有多个 owner，而使用 Box<T>, RefCell<T> 你只能有一个 owner；
// Box<T> 在编译期可以是 immutable 或者是 mutable，但是 Rc<T> 只能是 immutable（编译期检查），
// RefCell<T>运行是 immutable 和 mutable 的，但是是在运行时；
// RefCell<T> 允许在运行时做 mutable borrow，所以你可以修改 T 即时它是 immutable 的，
// 这也正是 RefCell<T>的 interior mutability（内部可变性）

// & -> RefCell.borrow() -> Ref: Deref
// &mut -> RefCell.borrow_mut() -> RefMut: Deref
// like borrow check at compiler time of Box<T>/Rc<T>, RefCell<T> keeps track of how many Ref<T>
// and RefMut<T> are active at runtime time, you can have onley one RefMut<T> and many Ref<T>,
// but can not have them the same time, otherwise you'll get runtime panic!
// 得到运行时 panic! 意味着你应该在开发环境中暴露这个问题，而不是等到生产环境

// RefCell<T>可以用在那些只有 immutable reference 但依然想改变内部数据的地方
// 另外 RefCell<T> 和 Rc<T> 结合可以实现对一个数据有多个 owner 并且都能拥有内部可变性！

// std 标准库还提供了另一个智能指针类型：Cell<T>，它和 RefCell<T> 不同的是里面装箱的是值而不是引用
// 在 Cell<T>.clone()时会将值拷贝一份，这使得它在跨线程时是安全的

use crate::refcell::List::Cons;
use std::cell::RefCell;
use std::rc::Rc;

pub fn main() {
    // let x = 5;
    // let y = &mut x;

    let x = 5;
    let y = RefCell::new(x);
    *(y.borrow_mut()) = 6;
    println!("y: {:?}", y);

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(List::Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    println!("value before = {:?}", value);
    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    *value.borrow_mut() += 10;

    println!("value after = {:?}", value);
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod test {
    use crate::refcell::{LimitTracker, Messenger};
    use std::borrow::BorrowMut;
    use std::cell::RefCell;

    struct MockMessenger {
        // sent_message: Vec<String>,
        sent_message: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_message: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // 在不改变接口约定的情况下，使用 RefCell<T> 能使用内部可变性来实现 struct 的功能
        fn send(&self, msg: &str) {
            // self is immutable reference, cannot be borrowed as mutable
            // self.sent_message.push(String::from(msg));

            // &mut -> RefCell.borrow_mut() -> RefMut: Deref
            self.sent_message.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // & -> RefCell.borrow() -> Ref: Deref
        assert_eq!(1, mock_messenger.sent_message.borrow().len());
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
