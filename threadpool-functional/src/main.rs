use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;
use std::time::Instant;

/// Parallele Map-Funktion
/// Wendet `func` auf jedes Element von `data` an und gibt die Ergebnisse als `Vec<U>` zurück
pub fn parallel_map<T, U, F>(data: &[T], func: F) -> Vec<U>
where
    T: Send + Sync + Clone + 'static, // T muss zwischen Threads sicher kopierbar sein
    U: Send + 'static + Clone + std::fmt::Debug, // U muss zwischen Threads transportierbar sein
    F: Fn(T) -> U + Send + Sync + 'static + Clone, // Funktion, die parallel angewendet wird
{
    // Erstellen eines Thread-Pools mit 4 Threads
    let pool = ThreadPool::new(4);
    // Erstellen eines Arc<Mutex<Vec<Option<U>>>>, um die Ergebnisse zu speichern
    let results = Arc::new(Mutex::new(vec![None; data.len()]));

    // Iterieren über die Daten und für jedes Element einen Task erstellen
    for (i, item) in data.iter().cloned().enumerate() {
        let results = Arc::clone(&results); // Arc für Thread-Sicherheit klonen
        let func = func.clone(); // Funktion klonen, da sie in den Thread bewegt wird
        pool.execute(move || {
            let mut results = results.lock().unwrap(); // Mutex sperren
            results[i] = Some(func(item)); // Berechnung durchführen und Ergebnis speichern
        });
    }

    // Warten auf alle Threads
    pool.join();

    // Ergebnisse aus dem Arc<Mutex<>> extrahieren und in einen Vektor umwandeln
    Arc::try_unwrap(results)
        .unwrap()
        .into_inner()
        .unwrap()
        .into_iter()
        .map(|x| x.unwrap()) // Option<U> in U umwandeln
        .collect()
}

/// Parallele Reduce-Funktion
/// Reduziert `data` mit `func` parallel und gibt das Endergebnis zurück
pub fn parallel_reduce<T, F>(data: &[T], func: F, init: T) -> T
where
    T: Send + Sync + Clone + 'static + std::fmt::Debug, // T muss zwischen Threads sicher kopierbar sein
    F: Fn(T, T) -> T + Send + Sync + 'static + Copy, // Reduktionsfunktion
{
    // Erstellen eines Thread-Pools mit 4 Threads
    let pool = ThreadPool::new(4);
    // Erstellen eines Arc<Mutex<T>>, um das Ergebnis zu speichern
    let result = Arc::new(Mutex::new(init));

    // Iterieren über die Daten in Chunks und für jeden Chunk einen Task erstellen
    for chunk in data.chunks(data.len() / 4) {
        let result = Arc::clone(&result); // Arc für Thread-Sicherheit klonen
        let chunk = chunk.to_vec(); // Chunk in einen Vektor umwandeln
        pool.execute(move || {
            let mut result = result.lock().unwrap(); // Mutex sperren
            for item in chunk {
                *result = func(result.clone(), item); // Berechnung durchführen und Ergebnis speichern
            }
        });
    }

    // Warten auf alle Threads
    pool.join();

    // Ergebnis aus dem Arc<Mutex<>> extrahieren und zurückgeben
    Arc::try_unwrap(result).unwrap().into_inner().unwrap()
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];

    let mapped_data = parallel_map(&data, |x| x * 2);
    println!("Mapped Data: {:?}", mapped_data);

    let sum = parallel_reduce(&data, |a, b| a + b, 0);
    println!("Sum: {}", sum);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_map() {
        let list = vec![1, 2, 3, 4, 5];

        let mapped_data = parallel_map(&list, |x| x * 2);
        assert_eq!(mapped_data, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_parallel_reduce() {
        let list = vec![1, 2, 3, 4, 5];

        let data = parallel_reduce(&list, |a, b| a + b, 0);
        assert_eq!(data, 15);
    }

    #[test]
    fn test_parallel_map_with_timing() {
        let list: Vec<u32> = (1..=1_000_000).collect(); // Deutlich größerer Vektor
        
        let start = Instant::now();
        let mapped_data = parallel_map(&list, |x| x * 2);
        let duration = start.elapsed();
        
        assert_eq!(mapped_data.len(), list.len());
        assert_eq!(mapped_data[0], 2);
        assert_eq!(mapped_data[list.len() - 1], list[list.len() - 1] * 2);

        println!("Parallel map execution time: {:?}", duration);
    }




} 
