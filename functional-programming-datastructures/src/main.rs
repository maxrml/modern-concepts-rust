mod datastructures;
use crate::datastructures::Datastructure;
mod queue;
use std::collections::VecDeque;
use queue::Queue;
mod list;
use list::LinkedListDS;
use std::collections::LinkedList;
mod stack;
use std::vec::Vec;
use stack::Stack;


fn main() {
    
    // Stack erstellen und Werte hinzufügen
    let mut stack = Stack::new();
    stack.push(10);
    stack.push(20);
    stack.push(30);

    // Stack als String ausgeben
    println!("Stack: {}", stack.to_string());

    // Teste is_empty() und size()
    println!("Ist der Stack leer? {}", stack.is_empty());
    println!("Größe des Stacks: {}", stack.size());

    // Peek (oberstes Element)
    if let Some(top) = stack.peek() {
        println!("Oberstes Element: {}", top);
    }

    // Pop (oberstes Element entfernen)
    if let Some(popped) = stack.pop() {
        println!("Entferntes Element: {}", popped);
    }

    println!("Stack nach pop: {}", stack.to_string());

    // map: Verdoppele alle Werte
    let doubled_stack = stack.map(|x| x * 2, Stack::new());
    println!("Verdoppelter Stack: {}", doubled_stack.to_string());

    // filter: Nur Werte > 15 behalten
    let filtered_stack = stack.filter(|x| *x > 15, Stack::new());
    println!("Gefilterter Stack (>15): {}", filtered_stack.to_string());

    // for_each: Alle Werte ausgeben
    println!("Alle Elemente ausgeben:");
    stack.for_each(|x| println!("{}", x));

    // reduce: Summe aller Werte berechnen
    let sum = stack.reduce(|acc, x| acc + x, 0);
    println!("Summe aller Elemente: {}", sum);

    // reduce_right: Subtraktion von rechts nach links
    let diff = stack.reduce_right(|acc, x| acc - x, 100);
    println!("Ergebnis von reduce_right: {}", diff);

}