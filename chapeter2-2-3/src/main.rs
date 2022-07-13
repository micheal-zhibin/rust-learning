// fn add_with_extra(x: i32, y: i32) -> i32 {
//     let x = x + 1; // 语句
//     let y = y + 5; // 语句
//     x + y // 表达式
// }


fn main() {
    // 语句会执行一些操作但是不会返回一个值，而表达式会在求值后返回一个值
    // let a = 8;
    // let b: Vec<f64> = Vec::new();
    // let (a, c) = ("hi", false);
    
    // error，由于 let 是语句，因此不能将 let 语句赋值给其它值
    // let b = (let a = 8);

    // let y = {
    //     let x = 3;
    // 表达式不能包含分号，一旦你在表达式后加上分号，它就会变成一条语句，再也不会返回一个值
    //     x + 1
    // };
    // println!("The value of y is: {}", y);

    // 表达式如果不返回任何值，会隐式地返回一个 ()
    // fn ret_unit_type() {
    //     let x = 1;
    //     // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    //     if x > 1 {
    //     }
    // }
    // assert_eq!(ret_unit_type(), ())

    // let x = 5u32;
    // let y = {
    //     let x_squared = x * x;
    //     let x_cube = x_squared * x;

    //     // 下面表达式的值将被赋给 `y`
    //     x_cube + x_squared + x
    // };
    // let z = {
    //     // 分号让表达式变成了语句，因此返回的不再是表达式 `2 * x` 的值，而是语句的值 `()`
    //     2 * x;
    // };
    // println!("x is {:?}", x);
    // println!("y is {:?}", y);
    // println!("z is {:?}", z);

    // exercise1
    // 使用两种方法让代码工作起来
    // way1
    // let v = {
    //     let mut x = 1;
    //     x + 2
    // };
    // assert_eq!(v, 3);
    // way2
    // let v = {
    //     let mut x = 1;
    //     x += 2;
    //     x
    // };
    // assert_eq!(v, 3);

    // exercise2
    // let v = {
    //     let x = 3;
    //     x
    // };
    // assert!(v == 3);

    // exercise3
    // fn sum(x: i32, y: i32) -> i32 {
    //     x + y
    // }
    // let s = sum(1 , 2);
    // assert_eq!(s, 3);
}
