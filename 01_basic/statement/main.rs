fn get_add(x: u64, y: u64) -> u64
{
    x + y
}

fn f1()
{
    println!("first function call");
    let x: u64 = 12;
    let y: u64 = 13;
    let z: u64 = get_add(x, y);

    println!("result of addtion : {}", z);    
    println!("");
}

fn f2()
{
    println!("second function call");
    for i in 0..10
    {
        if i % 2 == 0
        {
            println!("{} is even", i);
        }
        else
        {
            println!("{} is odd", i);
        }
    }
    println!("");
}

fn f3()
{
    println!("third function call");
    let numbers: [u64; 5] = [ 1, 5, 3, 2, 7 ];
    for x in numbers.iter().enumerate()
    {
        let (i, number): (usize, &u64) = x;
        print!("{},",number);
    }
    println!("");
}

fn main() 
{
    f1();
    f2();
    f3();
}