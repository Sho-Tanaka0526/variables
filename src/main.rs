fn main() {
    //下記の例ではx=6の部分でxの値が変更できない旨のコンパイルエラーが発生する
    /*
    let x = 5;
    println!("The value of x is: {}", x);   // xの値は{}です
    x = 6;
    println!("The value of x is: {}", x);
    */

    //mutキーワードを追加してxの値を可変(mutable)にする(エラーは発生しない)
    /*
    let mut x = 5;
    println!("The value of x is: {}", x);   //xの値は{}です
    x = 6;
    println!("The value of x is: {}", x);   //xの値は{}です
    */

    //シャドーイングしてみる
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
        //->12が出力される
    }
    println!("The value of x is: {}", x);
    //->6が出力される

    // //spacetest
    // let mut spaces = "    ";
    // spaces = spaces.len();
}
