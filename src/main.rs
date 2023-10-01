pub(crate) use std::io;

fn countdown(_n:i32)->i32{
    // Counts down from user input given 'n'
    let mut _total:i32=_n;
    if _total==0 {
        return _total;
    }else{
        _total= _total-1;
        let mut _size= _total.to_string().chars().count();
        println!("t-{_total:>space$}",_total=_total,space=_size);
        return countdown(_total);
    }
}
fn main() {
    println!("Input an Integer");

    let mut input: String= String::new();
    io::stdin().read_line(&mut input).expect("Failed to Read Line");
    let value: i32 = input.trim().parse::<i32>().expect("Input a Integer");

    let _start: &str ="Count Down Initialized!";
    let _end: &str ="Count Down Completed!";

    println!("{_start}");
    countdown(value);
    println!("{_end}");
}
