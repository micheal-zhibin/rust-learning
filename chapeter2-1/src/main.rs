// 命名规范
// 对于 type-level 的构造倾向于用驼峰命名法，只需对首字母进行大写：Uuid 而不是 UUID，Usize 而不是 USize
// 对于 value-level 的构造使用蛇形命名法，缩略词全小写：is_xid_start，且出最后一部分，其他部分不能由单字母组成：btree_map 而不是 b_tree_map

// struct Struct1 {
//     e: i32
// }

fn main() {
    // // error
    // // let x = 5;
    // // correct
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // // correct
    // let _x = 5;
    // // error
    // // let y = 10;
    // let _y = 10;

    // let (a, mut b): (bool, bool) = (true, false);
    // // a = true 不可变, b = false 可变
    // println!("a = {:?}, b = {:?}", a, b);
    // b = true;
    // assert_eq!(a, b);

    // let (a, b, c, d, e);
    // (a, b) = (1, 2);
    // //  _ 代表匹配一个值，但是我们不关心具体是什么，因此没有是一个变量名而是使用了 _
    // [c, .., d, _] = [1, 2, 3, 4, 5];
    // Struct1 { e, .. } = Struct1{ e: 5 };
    // assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e])

    // // correct
    // const MAX_POINTS: u32 = 100_000;
    // // error
    // const mut MAX_POINTS: u32 = 100_000;

    // let x = 5;
    // // 在 main 函数的作用于内对之前的 x 进行遮蔽
    // let x = x + 1;
    // {
    //     // 在当前花括号作用域内，对之前的 x 进行遮蔽
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {}", x);
    // }
    // println!("The value of x is: {}", x);

    // // 字符串类型
    // let spaces = "     ";
    // // usize 述职类型
    // let spaces = spaces.len();
    // // error, expected &str,found usize
    // // spaces = spaces.len();
    // println!("The spaces is: {}", spaces);

    // exercise1
    // let x: i32; // 未初始化，但被使用
    // x = 32;
    // let _y: i32; // 未初始化，也未被使用
    // println!("x is equal to {}", x);
    
    // exercise2
    // let mut x =  1;
    // x += 2; 
    // println!("x = {}", x); 

    // exercise3
    // let x: i32 = 10;
    // let y: i32 = 10;
    // {
    //     let y: i32 = 5;
    //     println!("x 的值是 {}, y 的值是 {}", x, y);
    // }
    // println!("x 的值是 {}, y 的值是 {}", x, y);

    // exercise4
    // fn define_x() -> String {
    //     let x = "hello".to_string();
    //     x
    // }
    // let x = define_x();
    // println!("{}, world", x); 

    // exercise5
    // let x: i32 = 5;
    // {
    //     let x = 12;
    //     assert_eq!(x, 12);
    // }
    // assert_eq!(x, 5);
    // let x =  42;
    // println!("{}", x); // 输出 "42".

    // exercise6
    // let mut x: i32 = 1;
    // x = 7;
    // // 遮蔽且再次绑定
    // // let x = x; 
    // x += 3;
    // let y = 4;
    // // 遮蔽
    // let y = "I can also be bound to text!";

    // exercise7
    // let _x = 1;
    // let x = 1;
    // println!("{}", x);

    // exercise8
    // let (x, y) = (1, 2);
    // let mut x = x;
    // x += 2;
    // assert_eq!(x, 3);
    // assert_eq!(y, 2);

    // exercise9
    // let (x, y);
    // (x,..) = (3, 4);
    // [.., y] = [1, 2];
    // // 填空，让代码工作
    // assert_eq!([x,y], [3, 2]);
}
