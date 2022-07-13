// use std::ops::{Range, RangeInclusive};

fn main() {
    // error
    // let guess = "42".parse().expect("Not a number!");
    // correct
    // let guess = "42".parse::<i32>().expect("Not a number!");

    // 整数类型定义形式：有无符号+类型大小（位数）
    // 溢出时，如是 debug会panic，如是release则不检查溢出，但会进行补码转换，如 u8 中 256->0, 257->1

    // 使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
    // 如果使用 checked_* 方法时发生溢出，则返回 None 值
    // 使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
    // 使用 saturating_* 方法使值达到最小值或最大值

    // 避免浮点数陷阱准则：
    // 1. 避免在浮点数上测试相等性
    // 2. 当结果在数学上可能存在未定义时，需要格外的小心
    // 断言0.1 + 0.2与0.3相等，会抛panic
    // error
    // assert!(0.1 + 0.2 == 0.3);
    // correct
    // assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.00001);

    // let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    // let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
    // println!("abc (f32)");
    // println!("  0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    // println!("        0.3: {:x}", (abc.2).to_bits());
    // println!("");
    // println!("xyz (f64)");
    // println!("  0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    // println!("        0.3: {:x}", (xyz.2).to_bits());
    // println!("");
    // assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);

    // NaN 不能用于比较，否则会panic
    // let x = (-42.0_f32).sqrt();
    // error
    // assert_eq!(x, x);
    // correct
    // if x.is_nan() {
    //     println!("x is NaN");
    // }

    // // 加法
    // let sum = 5 + 10;
    // // 减法
    // let difference = 95.5 - 4.3;
    // // 乘法
    // let product = 4 * 30;
    // // 除法
    // let quotient = 56.7 / 32.2;
    // // 求余
    // let remainder = 43 % 5;

    // // 编译器会进行自动推导，给予twenty i32的类型
    // let twenty = 20;
    // // 类型标注
    // let twenty_one: i32 = 21;
    // // 通过类型后缀的方式进行类型标注：22是i32类型
    // let twenty_two = 22i32;
    // // 只有同样类型，才能运算
    // let addition = twenty + twenty_one + twenty_two;
    // println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);
    // // 对于较长的数字，可以用_进行分割，提升可读性
    // let one_million: i64 = 1_000_000;
    // println!("{}", one_million.pow(2));
    // // 定义一个f32数组，其中42.0会自动被推导为f32类型
    // let forty_twos = [
    //     42.0,
    //     42f32,
    //     42.0_f32,
    // ];
    // // 打印数组中第一个值，并控制小数位为2位
    // println!("{:.2}", forty_twos[0]);

    // // 二进制为00000010
    // let a:i32 = 2;
    // // 二进制为00000011
    // let b:i32 = 3;
    // // 2
    // println!("(a & b) value is {}", a & b);
    // // 3
    // println!("(a | b) value is {}", a | b);
    // // 1
    // println!("(a ^ b) value is {}", a ^ b);
    // // -4
    // println!("(!b) value is {} ", !b);
    // // 16
    // println!("(a << b) value is {}", a << b);
    // // 0
    // println!("(a >> b) value is {}", a >> b);
    // let mut a = a;
    // // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    // a <<= b;
    // // 16
    // println!("(a << b) value is {}", a);

    // 序列只允许用于数字或字符类型，原因是：它们可以连续，同时编译器在编译期可以检查该序列是否为空，字符和数字值是 Rust 中仅有的可以用于判断是否为空的类型。
    // for i in 1..=5 {
    //     println!("{}",i);
    // }
    // for i in 'a'..='z' {
    //     println!("{}",i);
    // }

    // 有理数和复数可以使用 num 库

    // 总结：
    // 1. Rust 拥有相当多的数值类型. 因此你需要熟悉这些类型所占用的字节数，这样就知道该类型允许的大小范围以及你选择的类型是否能表达负数
    // 2. 类型转换必须是显式的. Rust 永远也不会偷偷把你的 16bit 整数转换成 32bit 整数
    // 3. Rust 的数值上可以使用方法. 例如你可以用以下方法来将 13.14 取整：13.14_f32.round()，在这里我们使用了类型后缀，因为编译器需要知道 13.14 的具体类型

    // exercise1
    // // 移除某个部分让代码工作
    // let x: i32 = 5;
    // let mut y: u32 = 5;
    // // y = x;
    // let z = 10; // 这里 z 的类型是? z是i32

    // exercise2
    // // 填空
    // let v: u16 = 38_u8 as u16;

    // exercise3
    // //  修改 `assert_eq!` 让代码工作
    // // 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
    // fn type_of<T>(_: &T) -> String {
    //     format!("{}", std::any::type_name::<T>())
    // }
    // let x = 5;
    // assert_eq!("i32".to_string(), type_of(&x));

    // exercise4
    // 填空，让代码工作
    // assert_eq!(i8::MAX, 127); 
    // assert_eq!(u8::MAX, 255); 

    // exercise5
    // 解决代码中的错误和 `panic`
    // let v1 = u8::wrapping_add(251_u8, 8);
    // let v2 = i8::checked_add(100, 8).unwrap();
    // println!("{},{}",v1,v2);

    // exercise6
    // 修改 `assert!` 让代码工作
    // let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    // assert!(v == 1597);

    // exercise7
    // 将 ? 替换成你的答案
    // let x = 1_000.000_1; // f64
    // let y: f32 = 0.12; // f32
    // let z = 0.01_f64; // f64
    // println!("{},{},{}",type_of(&x),type_of(&y),type_of(&z));

    // exercise8
    // 使用两种方法来让下面代码工作
    // assert!(0.1_f32+0.2_f32==0.3_f32);
    // assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.00001);

    // exercise9
    // 1. 修改 assert! 让它工作 2. 让 println! 输出: 97 - 122
    // let mut sum = 0;
    // for i in -3..2 {
    //     sum += i
    // }
    // assert!(sum == -5);
    // for c in 'a'..='z' {
    //     println!("{}",c as u8);
    // }

    // exercise10
    // assert_eq!((1..5), Range{ start: 1, end: 5 });
    // assert_eq!((1..=5), RangeInclusive::new(1, 5));

    // exercise11
    // 填空，并解决错误
    // 整数加法
    // assert!(1u32 + 2 == 3);
    // // 整数减法
    // assert!(1i32 - 2 == -1);
    // assert!(1i8 - 2 == -1);
    // assert!(3 * 50 == 150);
    // assert!(9 / 3 == 3); // error ! 修改它让代码工作
    // assert!(24 % 5 == 4);
    // // 逻辑与或非操作
    // assert!(true && false == false);
    // assert!(true || false == true);
    // assert!(!true == false);
    // // 位操作
    // println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    // println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    // println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    // println!("1 << 5 is {}", 1u32 << 5);
    // println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
