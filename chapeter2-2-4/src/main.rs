fn main() {
    // fn add(i: i32, j: i32) -> i32 {
    //     i + j
    // }

    // 函数要点
    // 函数名和变量名使用蛇形命名法(snake case)，例如 fn add_two() -> {}
    // 函数的位置可以随便放，Rust 不关心我们在哪里定义了函数，只要有定义即可
    // 每个函数参数都需要标注类型

    // Rust 是强类型语言，因此需要你为每一个函数参数都标识出它的具体类型
    // fn another_function(x: i32, y: f32) {
    //     println!("The value of x is: {}", x);
    //     println!("The value of y is: {}", y);
    // }
    // another_function(5, 6.1);

    // fn plus_five(x:i32) -> i32 {
    //     x + 5
    // }
    // let x = plus_five(5);
    // println!("The value of x is: {}", x);

    // fn plus_or_minus(x:i32) -> i32 {
    //     if x > 5 {
    //         return x - 5
    //     }
    
    //     x + 5
    // }
    // let x = plus_or_minus(5);
    // println!("The value of x is: {}", x);

    // Rust 中的特殊返回类型
    // 1. 无返回值():
    // 函数没有返回值，那么返回一个 ()
    // 通过 ; 结尾的表达式返回一个 ()
    // fn clear(text: &mut String) -> () {
    //     *text = String::from("");
    // }
    // 2. 永不返回的发散函数 !
    // 当用 ! 作函数返回类型的时候，表示该函数永不返回( diverge function )，特别的，这种语法往往用做会导致程序崩溃的函数
    // fn dead_end() -> ! {
    //     panic!("你已经到了穷途末路，崩溃吧！");
    // }
    // dead_end()

    // exercise1
    // fn sum(x: i32, y: i32) -> i32 {
    //     x + y
    // }
    // // 不要修改下面两行代码!
    // let (x, y) = (1, 2);
    // let s = sum(x, y);
    // assert_eq!(s, 3);

    // exercise2
    // 使用另一个类型来替代 i32
    // fn print() -> () {
    //     println!("hello,world");
    // }
    // print();

    // exercise3
    // 用两种方法求解
    // way1
    // fn never_return() -> ! {
    //     // 实现这个函数，不要修改函数签名!
    //     panic!("你已经到了穷途末路，崩溃吧！");
    // }
    // fn never_return() -> ! {
    //     loop {
    //     };
    // }
    // never_return();

    // exercise4
    // 发散函数( Diverging function )不会返回任何值，因此它们可以用于替代需要返回任何值的地方
    // fn get_option(tp: u8) -> Option<i32> {
    //     match tp {
    //         1 => {
    //             // TODO
    //         }
    //         _ => {
    //             // TODO
    //         }
    //     };
        
    //     // 这里与其返回一个 None，不如使用发散函数替代
    //     never_return_fn()
    // }
    
    // // 使用三种方法实现以下发散函数
    // fn never_return_fn() -> ! {
    //     panic!("你已经到了穷途末路，崩溃吧！");
    // }
    // println!("Success!");

    // exercise5
    // 填空
    // let b = false;

    // let v = match b {
    //     true => 1,
    //     // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
    //     false => {
    //         println!("Success!");
    //         panic!("we have no value for `false`, but we can panic")
    //     }
    // };

    // println!("Excercise Failed if printing out this line!");
}
