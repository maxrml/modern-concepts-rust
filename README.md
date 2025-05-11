# Portfolio-Projekt: Moderne Programmierkonzepte

## Übersicht

Dieses Repository enthält mehrere Projekte, die im Rahmen der Vorlesung "Moderne Programmierkonzepte" erstellt wurden. Jedes Projekt demonstriert spezifische Konzepte und Techniken moderner Programmierung.

---

## Finale Abgabe

## Struktur des Repositories
Das Repository ist wie folgt strukturiert:


- **rpn-calculator/**: Aufgabe 1: Implementiert einen Rechner für die Reverse Polish Notation (RPN).
  - **src/**: Enthält die Hauptlogik in `main.rs`.
  - **Cargo.toml**: Projektkonfiguration für Rust.

- **simple-datastructures/**: Aufgabe 2: Implementiert grundlegende Datenstrukturen wie Listen, Stacks und Queues.
  - **src/**: Quellcode für die Datenstrukturen.
  - **Cargo.toml**: Projektkonfiguration für Rust.

- **functional-programming-datastructures/**: Aufgabe 3: Implementiert funktionale Methoden auf Datenstrukturen: Listen, Stacks und Queues.
  - **src/**: Quellcode für die Datenstrukturen.
  - **Cargo.toml**: Projektkonfiguration für Rust.
  
- **threadpool-functional/**: Aufgabe 4: Nebenläufige Programmierung mit Thread Pools.
  - **src/**: Enthält die Hauptlogik in `main.rs`.
  - **Cargo.toml**: Projektkonfiguration für Rust.

- **eDSL/**: Aufgabe 5: Enthält eine Implementierung eines eingebetteten DSLs (Domain-Specific Language) für mathematische und SVG-Operationen.
  - **src/**: Quellcode, einschließlich `math_edsl.rs`, `svg_edsl.rs` und zugehöriger Makros.
  - **output.svg**: Beispielausgabe der SVG-DSL.
  - **Cargo.toml**: Projektkonfiguration für Rust.

- **password-cracker/**: Aufgabe 6: Hashvergleiche in der rockyou.txt.
  - **src/**: Enthält die Hauptlogik in `main.rs`.
  - **Cargo.toml**: Projektkonfiguration für Rust.

- **README.md**: Diese Datei, die die Struktur des Repositories und die Build-Anweisungen erklärt.

---

## Build- und Test-Anweisungen

### Allgemeine Anforderungen

- Rust-Toolchain (mindestens Version 1.70) 
- Cargo als Build-System
-> https://www.rust-lang.org/learn/get-started

### Schritte

1. **Abhängigkeiten installieren**  
   Navigieren Sie in das jeweilige Projektverzeichnis und führen Sie aus: ```cargo build --release```

2. **Tests starten und Coverage einsehen**
Grundlegend getestet wird mit ```cargo test```. Die Test Coverage erhält man mit ```Befehl für Coverage```, anschließend wird ein /coverage Ordner erstellt, in diesem liegt dann eine HTML.

3. **Projekte starten**

- Aufgabe 1-5 werden gestartet mit ```cargo run``` jeweils immer im Projektordner. Für maximale Performance wird in Aufgabe 6 'cargo nightly verwendet' und empfohlen separat zu builden und die Datei auszuführen. Zusätzlich gibt es hier die Möglichkeit Compiler Flags zu setzen für eine aggresivere Kompilierung, angepasst an die Hardware.
- Build Flag: ```cargo +nightly build --release```
- Run: ```./target/release/untitled```