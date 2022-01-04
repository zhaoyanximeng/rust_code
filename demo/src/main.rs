// const MAX_POINTS: u32 = 100_000;

fn main() {
    // 变量声明
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

    // 类型转换
    let guess: u32 = "42".parse().expect("Not a number");
    println!("{}", guess);
}
