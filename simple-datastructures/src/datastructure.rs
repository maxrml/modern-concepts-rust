pub trait Datastructure<T>{
    // isEmpty(): true wenn leer ist sonst false
    fn is_empty(&self) -> bool ;

    // isFull(): false wenn leer ist sonst true
    fn is_full(&mut self) -> bool {
        !self.is_empty()
    }

    // Gibt die Anzahl der Elemente in der Liste zurück
    fn size(&self) -> i32;

    // Gibt true zurück, wenn zwei Datenstrukturen des gleichen Typs 
    // die gleichen Werte (in gleicher Reihenfolge) enthalten; sonst false.
    fn equals(&self, other: &Self) -> bool {
        // konvertieren beide zu Strings und vergleichen diese
        self.to_string() == other.to_string()
    }

    //eine Repräsentation als String haben (toString), die den Inhalt der Datenstruktur darstellt
    //und die die programmatische Rekonstruktion der Datenstruktur ermöglicht bzw. erleichtert.
    fn to_string(&self) -> String;  
}