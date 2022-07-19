fn main() {
    // let x = 5;
    // let y = &x;
    // assert_eq!(5, x);
    // // correct
    // assert_eq!(5, *y);
    // // error: can't compare `{integer}` with `&{integer}`
    // assert_eq!(5, y);

    // fn calculate_length(s: &String) -> usize {
    //     s.len()
    // }
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}.", s1, len);

    // fn change(some_string: &String) {
    //     // error: 正如变量默认不可变一样，引用指向的值默认也是不可变的
    //     some_string.push_str(", world");
    // }
    // let s = String::from("hello");
    // change(&s);
    // println!("The str is {}.", s);

    // fn change(some_string: &mut String) {
    //     some_string.push_str(", world");
    // }
    // let mut s = String::from("hello");
    // change(&mut s);
    // println!("The str is {}.", s);

    // 同一作用域，特定数据只能有一个可变引用
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // // cannot borrow `s` as mutable more than once at a time
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // let mut s = String::from("hello");
    // {
    //     let r1 = &mut s;
    // } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
    // let r2 = &mut s;

    // let mut s = String::from("hello");
    // let r1 = &s; // 没问题
    // let r2 = &s; // 没问题
    // // error: cannot borrow `s` as mutable because it is also borrowed as immutable
    // let r3 = &mut s; // 大问题
    // println!("{}, {}, and {}", r1, r2, r3);

    // fn main() {
    //     let mut s = String::from("hello");
     
    //      let r1 = &s;
    //      let r2 = &s;
    //      println!("{} and {}", r1, r2);
    //      // 新编译器中，r1,r2作用域在这里结束
     
    //      let r3 = &mut s;
    //      println!("{}", r3);
    // } // 老编译器中(<1.31)，r1、r2、r3作用域在这里结束
    // // 新编译器中(>=1.31)，r3作用域在这里结束

    // 悬垂引用也叫做悬垂指针，意思为指针指向某个值后，这个值被释放掉了，而指针仍然存在，其指向的内存可能不存在任何值或已被其它变量重新使用
    // fn dangle() -> &String { // dangle 返回一个字符串的引用
    //     let s = String::from("hello"); // s 是一个新字符串
    //     &s // 返回字符串 s 的引用
    // } // 这里 s 离开作用域并被丢弃。其内存被释放。
    //   // 危险！
    // // this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    // let reference_to_nothing = dangle();


    // exercise1
    // let x = 5;
    // // 填写空白处
    // let p = &x;
    // println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84

    // exercise2
    // let x = 5;
    // let y = &x;
    // // 只能修改以下行
    // assert_eq!(5, *y);

    // exercise3
    // fn borrow_object(s: &String) {}
    // let mut s = String::from("hello, ");
    // borrow_object(&s)

    // exercise4
    // fn push_str(s: &mut String) {
    //     s.push_str("world")
    // }
    // let mut s = String::from("hello, ");
    // push_str(&mut s)

    // exercise5
    // let mut s = String::from("hello, ");
    // // 填写空白处，让代码工作
    // let p = &mut s;
    // p.push_str("world");

    // ref 与 & 类似，可以用来获取一个值的引用，但是它们的用法有所不同。

    // exercise6
    // // 获取传入引用的内存地址的字符串形式
    // fn get_addr(r: &char) -> String {
    //     format!("{:p}", r)
    // }
    // let c = '中';
    // let r1 = &c;
    // // 填写空白处，但是不要修改其它行的代码
    // let ref r2 = c;
    // assert_eq!(*r1, *r2);
    // // 判断两个内存地址的字符串是否相等
    // assert_eq!(get_addr(r1),get_addr(r2));

    // exercise7
    // // 移除代码某个部分，让它工作
    // // 你不能移除整行的代码！
    // let mut s = String::from("hello");
    // let r1 = &s;
    // // let r2 = &mut s;
    // let r2 = &s;
    // println!("{}, {}", r1, r2);

    // exercise8
    // fn borrow_object(s: &mut String) {}
    // // 通过修改下面一行代码来修复错误
    // let mut s = String::from("hello, ");
    // borrow_object(&mut s)

    // exercise9
    // fn borrow_object(s: &String) {}
    // let mut s = String::from("hello, ");
    // borrow_object(&s);
    // s.push_str("world");

    // exercise10
    // // 注释掉一行代码让它工作
    // let mut s = String::from("hello, ");
    // let r1 = &mut s;
    // r1.push_str("world");
    // let r2 = &mut s;
    // r2.push_str("!");
    // // println!("{}",r1);


    // exercise11
    // let mut s = String::from("hello, ");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // // 在下面增加一行代码人为制造编译错误：cannot borrow `s` as mutable more than once at a time
    // // 你不能同时使用 r1 和 r2
    // r1.push_str("world");


}
