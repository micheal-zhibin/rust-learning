fn main() {
    // 三种流派：
    // 垃圾回收机制(GC)，在程序运行时不断寻找不再使用的内存，典型代表：Java、Go
    // 手动管理内存的分配和释放, 在程序中，通过函数调用的方式来申请和释放内存，典型代表：C++
    // 通过所有权来管理内存，编译器在编译时会根据一系列规则进行检查，代表：Rust（检查只发生在编译期）

    // 所有权原则
    // Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
    // 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
    // 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)

    // {                      // s 在这里无效，它尚未声明
    //     let s = "hello";   // 从此处起，s 是有效的

    //     // 使用 s
    // }                      // 此作用域已结束，s不再有效

    // 字符串字面值是很方便的，但是它并不适用于所有场景。原因有二：
    // 1. 字符串字面值是不可变的，因为被硬编码到程序代码中
    // 2. 并非所有字符串的值都能在编写代码时得知

    // let mut s = String::from("hello");
    // s.push_str(", world!"); // push_str() 在字符串后追加字面值
    // println!("{}", s); // 将打印 `hello, world!`

    // let x = 5;
    // let y = x;
    // println!("{}", x);
    // println!("{}", y);

    // let s1 = String::from("hello");
    // let s2 = s1;
    // borrow of moved value: `s1`
    // println!("{}", s1);
    // println!("{}", s2);

    // x 只是引用了存储在二进制中的字符串 "hello, world"，并没有持有所有权
    // let x: &str = "hello, world";
    // let y = x;
    // println!("{},{}",x,y);

    // 克隆（深拷贝）
    // Rust 永远也不会自动创建数据的 “深拷贝”。因此，任何自动的复制都不是深拷贝，可以被认为对运行时性能影响较小。
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1 = {}, s2 = {}", s1, s2);

    // 拷贝（浅拷贝）
    // 浅拷贝只发生在栈上，因此性能很高
    // 如果一个类型拥有 Copy 特征，一个旧的变量在被赋值给其他变量后仍然可用。
    // 任何基本类型的组合可以 Copy ，不需要分配内存或某种形式资源的类型是可以 Copy 的
    // let x = 5;
    // let y = x;
    // println!("x = {}, y = {}", x, y);

    // fn main() {
    //     let s = String::from("hello");  // s 进入作用域
    
    //     takes_ownership(s);             // s 的值移动到函数里 ...
    //                                     // ... 所以到这里不再有效
    
    //     let x = 5;                      // x 进入作用域
    
    //     makes_copy(x);                  // x 应该移动函数里，
    //                                     // 但 i32 是 Copy 的，所以在后面可继续使用 x
    
    // } // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
    //   // 所以不会有特殊操作
    
    // fn takes_ownership(some_string: String) { // some_string 进入作用域
    //     println!("{}", some_string);
    // } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放
    
    // fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    //     println!("{}", some_integer);
    // } // 这里，some_integer 移出作用域。不会有特殊操作


    // fn main() {
    //     let s1 = gives_ownership();         // gives_ownership 将返回值
    //                                         // 移给 s1
    
    //     let s2 = String::from("hello");     // s2 进入作用域
    
    //     let s3 = takes_and_gives_back(s2);  // s2 被移动到
    //                                         // takes_and_gives_back 中,
    //                                         // 它也将返回值移给 s3
    // } // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
    //   // 所以什么也不会发生。s1 移出作用域并被丢弃
    
    // fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
    //                                              // 调用它的函数
    
    //     let some_string = String::from("hello"); // some_string 进入作用域.
    
    //     some_string                              // 返回 some_string 并移出给调用的函数
    // }
    
    // // takes_and_gives_back 将传入字符串并返回该值
    // fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    
    //     a_string  // 返回 a_string 并移出给调用的函数
    // }


    // exercise1
    // 使用尽可能多的方法来通过编译
    // let x: &str = "hello, world";
    // let y = x;
    // let x = String::from("hello, world");
    // let y = x.clone();
    // println!("{},{}",x,y);


    // exercise2
    // 只能修改下面的代码!
    // fn take_ownership(s: String) -> String {
    //     println!("{}", s);
    //     s
    // }
    // let s1 = String::from("hello, world");
    // let s2 = take_ownership(s1);
    // println!("{}", s2);

    // exercise3
    // 只能修改下面的代码!
    // fn give_ownership() -> String {
    //     let s = String::from("hello, world");
    //     // convert String to Vec
    //     // 将 String 转换成 Vec 类型
    //     let _s = s.as_bytes();
    //     s
    // }
    // let s = give_ownership();
    // println!("{}", s);

    // exercise4
    // fn print_str(s: String)  {
    //     println!("{}",s)
    // }
    // let s = String::from("hello, world");
    // print_str(s.clone());
    // println!("{}", s);

    // exercise5
    // let x = (1, 2, (), "hello".to_string());
    // let y = x.clone();
    // let x = (1, 2, (), "hello");
    // let y = x;
    // println!("{:?}, {:?}", x, y);

    // exercise6
    // let s = String::from("hello, ");
    // // 只修改下面这行代码 !
    // let mut s1 = s;
    // s1.push_str("world")

    // exercise7
    // let x = Box::new(5);
    // let mut y = x.clone();      // 完成该行代码，不要修改其它行！
    // let mut y = Box::new(3);    // 完成该行代码，不要修改其它行！
    // *y = 4;
    // assert_eq!(*x, 5);

    // 部分 move
    // struct Person {
    //     name: String,
    //     age: Box<u8>,
    // }
    // let person = Person {
    //     name: String::from("Alice"),
    //     age: Box::new(20),
    // };
    // // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
    // // 但是，这里 `age` 变量确是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age 
    // let Person { name, ref age } = person;
    // println!("The person's age is {}", age);
    // println!("The person's name is {}", name);
    // // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
    // //println!("The person struct is {:?}", person);
    // // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
    // println!("The person's age from person struct is {}", person.age);


    // exercise8
    // let t = (String::from("hello"), String::from("world"));
    // let _s = t.0;
    // // 仅修改下面这行代码，且不要使用 `_s`
    // println!("{:?}", t.1);

    // exercise9
    // let t = (String::from("hello"), String::from("world"));
    // // 填空，不要修改其它代码
    // let (s1, s2) = t.clone();
    // println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}
