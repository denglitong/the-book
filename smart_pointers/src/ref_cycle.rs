use crate::ref_cycle::List::{Cons, Nil};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn next(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// prevent ref cycles: turning a Rc<T> to Weak<T>
// Rc::downgrade() -> Weak<T>, unlike of strong count, the weak count doesn't need bo to be 0
// to be cleaned up, Weak<T> don't express an ownership relationship, and thus when you do anything
// with Weak<T> it doesn't guarantee the ref is valid, so you must need to check before you can use it
// by using Weak<T>.upgrade() -> Option<Rc<T>>

#[derive(Debug)]
struct Node {
    value: i32,
    // 这里要加 RefCell<> 装箱的原因是，Node 在业务场景里需要有多个 owner，所以需要使用 Rc<Node>
    // 而 Rc<T> 是只读的，只读的 Node 无法对其成员 parent/children 做出修改，
    // 除非对成员使用 RefCell<> 类型取得内部可变性，所以这里 parent/children 需要使用 RefCell<> 类型
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.next());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.next());

    if let Some(link) = a.next() {
        // cycle reference occurs, memory leak
        // you must ensure that you don't create ref cycles, you can't rely on Rust to catch them
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing b = {}", Rc::strong_count(&a));

    // fatal runtime error: stack overflow
    // println!("a next item = {:?}", a.next());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let mut branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // `DerefMut` is required to modify through a dereference, but it is not implement for Rc<Mode>
        // 这就是我们为什么需要 RefCell<> 类型，它提供了内部可变性，不管是对于 Rc<T> 还是其他类型
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );

        let another_leaf = Rc::new(Node {
            value: 10,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        // Rc<Node> 是只读的，要想可修改只能借助 RefCell<T> 取得 mutability 使得 Vec<T> 是 mutable 的可以 push 进去
        // (*branch).children.push(Rc::clone(&another_leaf));
        branch.children.borrow_mut().push(Rc::clone(&another_leaf));
        println!("branch children = {:?}", branch);
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
