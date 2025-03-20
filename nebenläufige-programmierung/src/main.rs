mod datastructures;
mod list;
mod utils;
use list::LinkedListDS;
use crate::datastructures::Datastructure;
use std::collections::LinkedList;


fn main() {
    


    let mut list = LinkedListDS::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.insert(4);
    list.push_front(5);


    println!("\nOriginale LinkedList: {}", list.to_string());
    println!("\nIst die Liste leer? {}", list.is_empty());
    println!("\nGröße der Liste: {}", list.size());
    let mut mapped_list = list.map(|x| x * 2, LinkedListDS::new());
    println!("\nNeue Liste (mapped): {}", mapped_list.to_string());
    let mut filtered_list = list.filter(|x| x % 2 != 0, LinkedListDS::new());
    println!("\nListe von der ungeraden Zahlen: {}", filtered_list.to_string());

    println!("For-each Ausgabe:");
    list.for_each(|x| println!("{}", x));

    let sum = list.reduce(|acc, x| acc + x, 0);
    println!("Summe mit reduce: {}", sum);

    let sum_right = list.reduce_right(|acc, x|, acc + x);
    println!("Summe mit reduce_right: {}", sum_right);

   
    let parallel_mapped = list.parallel_map(|x| x*x , LinkedListDS::new());
    println!("\nQuadrierte LinkedList (parallel_map): {}", parallel_mapped.to_string());


    let parallel_sum = list.parallel_reduce(|acc, x| acc + x , 0);
    println!("\nSumme aller Elemente (parallel_reduce): {}", parallel_sum);

    
   
}
    


