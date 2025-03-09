

mod list;
use list::LinkedList;


fn main() {
    let mut list = LinkedList::new();

    // Elemente hinzufügen

    list.push_back(3);
    list.push_back(4);

    println!("Liste: {}", list.to_string());
    

   
}
