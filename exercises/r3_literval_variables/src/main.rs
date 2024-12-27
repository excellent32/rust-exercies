
fn print_result(num: i32){
    println!("The result is {num}")
}

macro_rules! math{
    ($first:literal plus $second:literal) => {
        $first * $second
    };
    (square $first:literal) => {
        $first * $first
    }
}

fn main() {
    print_result(math!(3 plus 5));

    print_result( math!(square 5));
}
