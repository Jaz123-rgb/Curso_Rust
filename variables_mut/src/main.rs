fn main() {
    let mut x =5;
 
    println!("the value of x is:{x}");
        x = 123;
    println!("the new value of x si {x} ");

    let a = 10;
    let a = a + 2;
    {
        let a = a * 2;
        println!("the value of x in the inner scope is: {a}");
    }
    println!("the value of x is: {x}")
}
