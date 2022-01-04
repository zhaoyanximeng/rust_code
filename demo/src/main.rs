// const MAX_POINTS: u32 = 100_000;

fn main() {
    // // 变量声明
    // println!("Hello, world!");
    //
    // let x = 5;
    // let x = 4;
    // let x = x * MAX_POINTS;
    // println!("The value of x is {}", x);
    //
    // let spaces = "    ";
    // let spaces = spaces.len();
    //
    // println!("{}", spaces);

    // // 类型转换
    // let guess: u32 = "42".parse().expect("Not a number");
    // println!("{}", guess);

    // // 默认类型
    // let x = 5; //i32
    // let y = 4.1; //f64

    // // 字符声明
    // let x = 'z';
    // let y: char = '$';
    // let z = '😭';

    // // 复合类型Tuple
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // println!("{}, {}, {}", tup.0, tup.1, tup.2);

    // // 数组(数据存放在stack栈上而不是heap堆上)
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let b = [3; 5];
    // println!("{}, {}", a[0], b[2])

    // // 函数
    // another_function(3, 4)

    // // 语句和表达式
    // let x = five(1);
    // println!("The value of x is {}", x)

    // // if
    // let number = 3;
    // if number == 1 {
    //     println!("The value of number is 1")
    // } else if number == 2 {
    //     println!("The value of number is 2")
    // } else {
    //     println!("The value of number is not 1 and 2")
    // }
    // // if多分支用match重构
    // match number {
    //     1 => println!("The value of number is 1"),
    //     2 => println!("The value of number is 2"),
    //     _ => println!("The value of number is not 1 and 2")
    // }
    // // if表达式放在等号右边
    // let value = true;
    // let number = if value { 1 } else { 2 };
    // println!("The value of number is {}", number);

    // loop循环
    let mut count = 0;
    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };
    println!("The result is {}", result);

    // while循环
    while count != 0 {
        println!("{}!", count);
        count = count - 5;
    }
    println!("The end of while");

    // for循环
    let a = [1, 2, 3, 4, 5];
    for ele in a.iter() {
        println!("The value is {}", ele)
    }
    // for range
    for number in (1 .. 4).rev() {
        println!("{}", number);
    }
    println!("END!!!");
}

// fn five(x: i32) -> i32 {
//     x + 5
// }

// fn another_function(x: i32, y: i32) {
//     println!("The value of x is: {}, {}", x, y);
// }