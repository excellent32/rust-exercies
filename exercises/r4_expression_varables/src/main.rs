

// fn print_result(num: i32) {
//     println!("The result is {num}");
// }

// macro_rules! math {
//     ($a:expr, plus, $b:expr) => {
//         $a + $b
//     };
//     (square $a:expr) => {
//         $a * $a
//     };
// }


// fn main() {
//     let var = 5;
//     print_result(math!((2 * 3), plus, var));
//     print_result(math!(square var));
// }


fn print_success(){
    println!("Yay, the if statement worked.");
}

macro_rules! if_any{
    ($($e:expr), +; $block:block)=>{
        eprintln!("{:#?}", $($e))
        if $($e) || + {
            $block
        }
    }
}

fn main(){
    if_any!(true, 0 == 1, false; {
        print_success();
    })
}


// fn print_vec<V: std::fmt::Debug>(vec: &Vec<V>) {
//     println!("{vec:#?}");
// }

// macro_rules! graph{
//     ($($from:literal -> ( $( $to:literal),*);)*) => {
//         {
//             let mut vec = Vec::new();
//             $( $(vec.push(($from, $to));)* ) *

//             vec
//         }
//     }
// }

// #[allow(clippy::vec_init_then_push)]
// fn main(){
//     let my_graph = graph!(
//         1 -> (2, 3, 4, 5);
//         2 -> (1, 3);
//         3 -> (2);
//         4 -> ();
//         5 -> (1, 2, 3);
//     );

//     print_vec(&my_graph);
// }

// #[derive(Debug)]
// enum NumberType {
//     PositiveNumber(u32),
//     NegativeNumber(i32),
//     UnknownBecauseBlock(i32),
//     UnknownBecauseExpr(i32),
// }

// impl NumberType {
//     fn show(&self) {
//         println!("{self:#?}")
//     }
// }

// macro_rules! sum{
//     ($first:expr, $($expr:expr),+) => {
//         $first $(+ $expr)+
//     }
// }

// macro_rules! get_number_type {
//     (-$negative:literal) => {
//         NumberType::NegativeNumber(-$negative)
//     };
//     ( $positive:literal) => {
//         NumberType::PositiveNumber($positive)
//     };
//     ( $block:block) => {
//         NumberType::UnknownBecauseBlock($block)
//     };
//     ( $expr:expr) => {
//         NumberType::UnknownBecauseExpr($expr)
//     }
// }

// fn main(){
//     // get_number_type!(4).show();

//     // get_number_type!(-5).show();

//     // #[allow(clippy::let_and_return)]
//     // get_number_type!({
//     //     let x = 6;
//     //     x
//     // }).show();

//     get_number_type!(sum!(1, 2, 3, 4)).show();
// }


// use std::collections::HashMap;
// use std::fmt::Debug;

// fn print_pair<K: Debug, V: Debug>(pair: (K,V)) {
//     println!("{pair:#?}");
// }

// fn print_hashmap<K: Debug, V:Debug>(hashmap: &HashMap<K,V>){
//     println!("{hashmap:#?}");
// }

// macro_rules!pair {
//     ($i:expr => $e: expr) => {
//         ($i,$e)
//     }
// }

// macro_rules! hashmap{
//     ($($k:expr => $v:expr,)*) => {
//     {
//         HashMap::from([$(pair!($k => $v)),*])
//     }
//     }
// }

// fn main(){
//     let pair: (char, u8) = pair!('a' => 1);

//     print_pair(pair);

//     let value = "value";

//     let my_hashmap = hashmap!{
//         String::from("Hash") => "map",
//         String::from("Key") => value,
//     };

//     print_hashmap(&my_hashmap);
// }


// fn print_numbers(nums: &Vec<i32>) {
//     println!("Resulting Numbers: {nums:#?}");
// }

// fn get_example_vec() -> Vec<i32> {
//     vec![1,3,5,6,7,8]
// }

// fn print_curried_argument(val: impl std::fmt::Debug) {
//     println!("Currying value {:?}.", val);
// }

// macro_rules! curry{
//     (_, $block:block) => {
//         $block
//     };
//     (($argident:ident : $argtype:ty) => $(($argidents:ident : $argtypes:ty) => )* _, $block:block) => {
//         move |$argident: $argtype| {
//             print_curried_argument($argident);
//             curry!($(($argidents: $argtypes) => )* _, $block)
//         }
//     }
// }

// fn main(){
//     println!("===================");
//     let is_between = curry!((min: i32) => (max: i32) => (item: &i32) => _, {
//         min < *item && *item < max
//     });

//     let curry_filter_between = curry!((min: i32) => (max: i32) => (vec: &Vec<i32>) => _, {
//         let filter_between = is_between(min)(max);
//         vec.iter().filter_map(|i| if filter_between(i) {
//             Some(*i)
//         }else {
//             None
//         }).collect()
//     });

//     println!("==========create between 3_7====");
//     let between_3_7 = curry_filter_between(3)(7);
//     let my_vec = get_example_vec();
//     let some_numbers: Vec<i32> = between_3_7(&my_vec);
//     print_numbers(&some_numbers);
// }


