mod datastructure;
use datastructure::Datastructure;
    
    // Definiert eine generische Stack-Struktur
    pub struct Stack<T> {
        elements: Vec<T>,
    }
    
    #[derive(Debug)]
    pub enum StackError {
        EmptyStack,            // Fehler, wenn versucht wird, ein Element von einem leeren Stack zu entfernen
        Overflow,              // Fehler, wenn der Stack seine maximale Kapazität erreicht
        InvalidPosition,       // Fehler, wenn eine ungültige Position angegeben wird
        ElementNotFound,       // Fehler, wenn ein zu entfernendes Element nicht gefunden wird
    }
        
    
    impl<T> Datastructure for Stack<T>{
        // Neue Instanz eines leeren Stacks erstellen
        fn new() -> Self {
            Stack {
                elements: Vec::new(),
            }
        }
    
        // push(elem): Fügt ein neues Element dem Stapel hinzu.
        fn push(&mut self, item: T) {
            self.elements.push(item);
        }
    
        // pushAll(elems): Fügt alle Elemente dem Stapel hinzu. Wenn möglich, dann nutzen Sie eine variadische Methode (*Varargs* in Java).
        fn push_all<I>(&mut self, items: I)
        where
            I: IntoIterator<Item = T>,
        {
            for item in items {
                self.push(item);
            }
        }
    
        // pop(): Entfernt das zuletzt hinzugefügte Element.
        fn pop(&mut self) -> Result<T, StackError> {
            self.elements.pop().ok_or(StackError::EmptyStack)
        }
    
        // Speek(): Gibt das zuletzt hinzugefügte Element zurück, ohne es zu entfernen.
        fn peek(&self) -> Option<&T> {
            self.elements.last()
        }
    
        
    }


    /*
    push(elem): Fügt ein neues Element dem Stapel hinzu.
    pushAll(elems): Fügt alle Elemente dem Stapel hinzu. Wenn möglich, dann nutzen Sie eine variadische Methode (*Varargs* in Java).
    */