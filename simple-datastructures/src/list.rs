
#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

#[derive(Debug)]
pub enum LinkedListError {
    IndexOutOfBounds,
    ElementNotFound,
    EmptyList,
}

impl<T: PartialEq> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    //get(pos): Gibt das Element an der gegebenen Position zurück.
    pub fn get(&self, pos: usize) -> Result<&T, LinkedListError> {
        if pos >= self.len {
            return Err(LinkedListError::IndexOutOfBounds);
        }
        let mut current = &self.head;
        for _ in 0..pos {
            current = &current.as_ref().unwrap().next;
        }
        Ok(&current.as_ref().unwrap().value)
    }

    // add(elems): Fügt alle Elemente der Liste hinzu.
    pub fn add(&mut self, elems: Vec<T>) -> Result<(), LinkedListError> {
        for elem in elems {
            self.insert(elem, self.len)?;
        }
        Ok(())
    }

    // insert(elem, pos): Einfügen eines Elements an der gegeben Position.
    pub fn insert(&mut self, elem: T, pos: usize) -> Result<(), LinkedListError> {
        if pos > self.len {
            return Err(LinkedListError::IndexOutOfBounds);
        }

        let new_node = Box::new(Node { value: elem, next: None });
        if pos == 0 {
            let mut new_node = new_node;
            new_node.next = self.head.take();
            self.head = Some(new_node);
        } else {
            let mut current = &mut self.head;
            for _ in 0..pos - 1 {
                current = &mut current.as_mut().unwrap().next;
            }
            let mut new_node = new_node;
            new_node.next = current.as_mut().unwrap().next.take();
            current.as_mut().unwrap().next = Some(new_node);
        }
        self.len += 1;
        Ok(())
    }

    // remove(elem): Entfernt das erste Vorkommen des Elements aus einer nicht leeren Liste.
    pub fn remove(&mut self, elem: T) -> Result<(), LinkedListError> {
        if self.is_empty() {
            return Err(LinkedListError::EmptyList);
        }

        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            if node.value == elem {
                current = node.next.take();
                self.len -= 1;
                return Ok(());
            }
            current = &mut node.next;
        }
        Err(LinkedListError::ElementNotFound)
    }

    // removeAt(pos): Entfernt das Element an der gegebenen Position.
    pub fn remove_at(&mut self, pos: usize) -> Result<(), LinkedListError> {
        
        if pos >= self.len {
            return Err(LinkedListError::IndexOutOfBounds);
        }
        if pos == 0 {
            self.head = self.head.take().and_then(|node| node.next);
        } else {
            let mut current = &mut self.head;
            for _ in 0..pos - 1 {
                current = &mut current.as_mut().unwrap().next;
            }
            let next = current.as_mut().unwrap().next.take().and_then(|node| node.next);
            current.as_mut().unwrap().next = next;
        }
        self.len -= 1;
        Ok(())
    }
    

    // replace(elem, pos): Ersetzt ein Element an der gegeben Position durch das neue Element.
    pub fn replace(&mut self, elem: T, pos: usize) -> Result<(), LinkedListError> {
        if pos >= self.len {
            return Err(LinkedListError::IndexOutOfBounds);
        }

        let mut current = &mut self.head;
        for _ in 0..pos {
            current = &mut current.as_mut().unwrap().next;
        }
        if let Some(ref mut node) = current {
            node.value = elem;
        }
        Ok(())
    }

    // size(): Gibt die Anzahl der Elemente in der Liste zurück.
    pub fn size(&self) -> usize {
        self.len
    }

    // isEmpty(): true wenn die Liste leer ist sonst false
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // isFull(): false wenn die Liste leer ist sonst true
    pub fn is_full(&self) -> bool {
        !self.is_empty()
    }
}
/*
get(pos): Gibt das Element an der gegebenen Position zurück.
add(elems): Fügt alle Elemente der Liste hinzu.
insert(elem, pos): Einfügen eines Elements an der gegeben Position.
remove(elem): Entfernt das erste Vorkommen des Elements aus einer nicht leeren Liste.
removeAt(pos): Entfernt das Element an der gegebenen Position.
replace(elem, pos): Ersetzt ein Element an der gegeben Position durch das neue Element.
size(): Gibt die Anzahl der Elemente in der Liste zurück.
isEmpty(): true wenn die Liste leer ist sonst false
isFull(): false wenn die Liste leer ist sonst true
*/




