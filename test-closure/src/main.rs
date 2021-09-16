


fn main() {
    // 普通の関数
    fn add(x:i32, y:i32) -> i32 { x + y }
    println!("{}", add(2, 3) );

    // ラムダ版
    let add_lambda = |x:i32, y:i32|{ x + y };
    println!("{}", add_lambda(1, 5));


    // クロージャ版。ラムダ版に比べて、引数が一つ減り、変数を一つキャプチャしている。
    let y:i32 = 1;
    let add_v2 = |x:i32| { x + y };
    println!("{}", add_v2(4));

    // 戻り値としてクロージャを返すクロージャ
    let add_v3 = |x:i32|{move |y:i32|{x + y}};
    println!("add_v3 {}", add_v3(2)(5));

    // 関数を引数にする
    fn add_1(x:i32)->i32 { x + 1} 
    fn add_arg(x:i32, f: fn(i32) -> i32 ) -> i32 { f(x) }
    println!("add_arg {}",  add_arg(3, add_1 ));
    println!("add_arg {}",  add_arg(3, |y:i32|{ y + 1 } ));

    // クロージャを引数にする
    fn add_v4(x:i32, |y:i32|{}->i32){x+y};
    println!("{}", add_v4(5, |5|{}()));
}