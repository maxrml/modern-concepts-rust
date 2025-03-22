mod list;
//use std::slice::SplitInclusiveMut;

use list::List;

fn main() {
    let list = List::new(vec![1, 2, 3, 4, 5]);
    

    let squared_list = list.parallel_map(|x| x * x);
    println!("Spuared List: {:?}", squared_list);

    let sum = list.parallel_reduce(|a, b| a + b);
    println!("Sum: {:?}", sum);

    

   


    
}
