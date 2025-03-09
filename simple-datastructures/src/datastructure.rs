

pub trait Datastructure{

    // isEmpty(): true wenn leer ist sonst false
    fn is_empty(&self) -> bool ;

    

    // isFull(): false wenn leer ist sonst true
    fn is_full(&mut self) -> bool {
        !self.is_empty()
    }

    
    fn equals(&self, other: &Self) -> bool; // Gibt true zurück, wenn zwei Datenstrukturen des gleichen Typs 
                                              // die gleichen Werte (in gleicher Reihenfolge) enthalten; sonst false.

    fn to_string(&self) -> String;  //eine Repräsentation als String haben (toString), die den Inhalt der Datenstruktur darstellt
                                         //und die die programmatische Rekonstruktion der Datenstruktur ermöglicht bzw. erleichtert.
}