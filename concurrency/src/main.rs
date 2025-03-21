mod list;
use list::List;

fn main() {
    let list = List::new(vec![1, 2, 3, 4, 5]);
    //let empty_list: List<i32> = List::new(vec![]);

    let squared_list = list.parallel_map(|x| x * x);
    println!("Spuared List: {:?}", squared_list);

    let sum = list.parallel_reduce(|a, b| a + b);
    match sum {
        Some(result) => println!("Sum: {}", result),
        None => println!("Sum konnte nicht berechnet werden, da die Liste leer ist."),
    }

    
}
