fn main(){
    let ary = [1, 2, 3];
    
    // 普通のfor文はなく、 while 文
    let mut i = 0;
    while i < (&ary).len() {
        println!("{}", ary[i]);
        i += 1;
    }

    println!();

    // 範囲for文
    for e in &ary {
        println!("{}", e);
    }

}