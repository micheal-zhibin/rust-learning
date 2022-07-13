// use std::mem::size_of_val;

fn main() {
    // 所有 unicode 值都可以作为 Ruse 字符
    // let c = 'z';
    // let z = 'ℤ';
    // let g = '国';
    // let heart_eyed_cat = '😻';

    // let x = '中';
    // println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));

    // Rust 的字符只能用 '' 来表示， "" 是留给字符串的

    // let t = true;
    // let f: bool = false; // 使用类型标注,显式指定f的类型
    // if f {
    //     println!("这是段毫无意义的代码");
    // }

    // exercise1
    //  修改2处 `assert_eq!` 让代码工作
    // let c1 = 'a';
    // assert_eq!(size_of_val(&c1),4); 
    // let c2 = '中';
    // assert_eq!(size_of_val(&c2),4); 
    // println!("Success!")

    // exercise2
    //  修改一行让代码正常打印
    // fn print_char(c : char) {
    //     println!("{}", c);
    // }
    // let c1 = '中';
    // print_char(c1);

    // exercise3
    // 使成功打印
    // let _f: bool = false;
    // let t = false;
    // if !t {
    //     println!("Success!")
    // }

    // exercise4
    // let f = true;
    // let t = true && false || true;
    // assert_eq!(t, f);
    // println!("Success!")

    // exercise5
    // fn implicitly_ret_unit() {
    //     println!("I will return a ()")
    // }
    // // 不要使用下面的函数，它只用于演示！
    // fn explicitly_ret_unit() -> () {
    //     println!("I will return a ()")
    // }
    // // 让代码工作，但不要修改 `implicitly_ret_unit` !
    // let v0: () = ();
    // let v = (2， 3);
    // assert_eq!(v0, implicitly_ret_unit());
    // println!("Success!")

    // exercise6
    // 让代码工作：修改 `assert!` 中的 `4` 
    // let unit: () = ();
    // unit type does't occupy any memeory space
    // assert!(size_of_val(&unit) == 0);
    // println!("Success!")
}
