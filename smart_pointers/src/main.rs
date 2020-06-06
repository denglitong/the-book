mod boxes;
mod rc;
mod ref_cycle;
mod refcell;

// Rust 的智能指针，或者更广泛的说，类型系统，是经过设计以适应具体不同场景的
// 在选择数据类型/类型系统时，应先考察业务场景的特征，来选择对应的适合的类型
// 这样才能充分利用 Rust 的类型系统的设计来最大化的减少bug 并同时保证性能和高抽象
// 它要求我们必先做足调查和设计才能开始实践系统，是真正的把检查前移的设计

// Box<T> 装箱类型，堆分配，占用字节大小固定，可用于实现自包含类型
// Rc<T> 多 owner 类型，只读，单线程  vs Weak<T>
// RefCell<T> 提供了内部可变性类型，往往在只读结构体引用里面对其成员取得可变性，成员需声明为 RefCell<T> 类型
//      使用 RefCell<T> 时记住，它是为了让结构体的不可变引用可以修改内部字段用的
// 组合使用：
// Rc<RefCell<T>>
// RefCell<Rc<T>>

// Rust 中的智能指针是用来提供不同的保证并进行了权衡的（其他语言里面的智能指针也是这样比如 C++ 里面的move weak pointer）

fn main() {
    // boxes::main();
    // rc::main();
    // refcell::main();
    ref_cycle::main();
}
