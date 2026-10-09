/*
Cvičení 05 — Úloha 5.2
Název: Příprava na třídění
Body: 1 bod; 1/2 bodu; 1/2 bodu (v pořadí podúloh)

Zadání:
Vytvořte dvě globální proměnné (prirazeni a porovnani), které budou sloužit pro měření počtu volání
funkcí vytvořených v tomto příkladu. Napište následující funkce, které budou nahrazovat operátory:

a) funkce přiřazení (globální proměnná prirazeni ) [1 bod]

b) funkce je menší (globální proměnná porovnani ) [1/2 bodu]

c) funkce je větší (globální proměnná porovnani ) [1/2 bodu]


Ukázka:

int x = 5, y = 4, z = 3;

bool je1 = JeMensi(y, z); // false

bool je2 = JeMensi(x, z); // false

bool je3 = JeVetsi(x, y); // true

Prirazeni(ref x, y); // x = 4

Prirazeni(ref y, z); // y = 3

Console.WriteLine("Prirazeni: {0}, Porovnani: {1}", prirazeni, porovnani); // 2 3

Požadavky / omezení:
a) K řešení příkladů je možné používat pouze syntaktických konstrukcí a funkcí probraných na
přednáškách. K řešení cvičení s nižším číslem nelze používat učivo hodin s vyšším číslem.

b) Řešení musí být procedurální, nesmí být objektové. Je zakázáno použití cyklu foreach. Je
zakázáno použití kolekcí (Dictionary, List, LinkedList, SortedList, atd.) a jejich obdoby v jiných
programovacích jazycích.

c) Příklady můžete řešit i v jiném jazyku než C#, pokud tento jazyk umožňuje procedurální
programování, např. C, C++, Java. Při této volbě zdůrazňuji platnost pravidla 2a, lze používat
pouze ekvivalenty syntaktických konstrukcí a funkcí probraných na přednášce.

d) Úloha musí být funkční. Otestujte ji minimálně na datech uvedených v zadání příkladu.

e) Zdrojové kódy musí být formátovány jednotným způsobem, zejména odsazování.

f) Proměnné a funkce musejí mít vhodné názvy

g) Zdrojové kódy musí být vhodně členěny do funkcí (platí pro 3. cvičení a následující)

h) Pro každé cvičení vytvořte vlastní Solution (Rešení). Jednotlivé příklady budou jako samostatné
Projekty.

i) Je zakázáno používat složené příkazy { }, pokud obsahují pouze jeden příkaz.
*/

use std::sync::atomic::{AtomicUsize, Ordering};

// Глобальный счётчик вызовов функции присваивания.
static ASSIGNMENTS: AtomicUsize = AtomicUsize::new(0);

// Глобальный счётчик вызовов функций сравнения.
static COMPARISONS: AtomicUsize = AtomicUsize::new(0);

// Функция присваивает переменной a значение b.
//
// a — изменяемая ссылка на исходную переменную.
// b — значение, которое нужно записать.
//
// Каждый вызов увеличивает счётчик ASSIGNMENTS на 1.
fn assign(a: &mut i32, b: i32) {
    // Увеличиваем глобальный счётчик присваиваний.
    ASSIGNMENTS.fetch_add(1, Ordering::Relaxed);

    // Изменяем исходную переменную через ссылку.
    *a = b;
}

// Проверяем, меньше ли первое число второго.
//
// Возвращает true, если a < b.
// Каждый вызов увеличивает COMPARISONS на 1.
fn is_less(a: i32, b: i32) -> bool {
    // Увеличиваем глобальный счётчик сравнений.
    COMPARISONS.fetch_add(1, Ordering::Relaxed);

    // Возвращаем результат сравнения.
    a < b
}

// Проверяем, больше ли первое число второго.
//
// Возвращает true, если a > b.
// Использует тот же счётчик COMPARISONS.
fn is_greater(a: i32, b: i32) -> bool {
    // Увеличиваем глобальный счётчик сравнений.
    COMPARISONS.fetch_add(1, Ordering::Relaxed);

    // Возвращаем результат сравнения.
    a > b
}

fn main() {
    // Исходные значения из задания.
    let mut x: i32 = 5;
    let mut y: i32 = 4;
    let z: i32 = 3;

    println!("INITIAL VALUES");
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!();

    // Выполняем три сравнения.
    // Результаты сохраняем в переменные типа bool.
    let result1 = is_less(y, z);
    let result2 = is_less(x, z);
    let result3 = is_greater(x, y);

    println!("COMPARISONS");
    println!("y < z: {}", result1);
    println!("x < z: {}", result2);
    println!("x > y: {}", result3);
    println!();

    // Присваиваем x значение y.
    // x изменится с 5 на 4.
    assign(&mut x, y);

    // Присваиваем y значение z.
    // y изменится с 4 на 3.
    assign(&mut y, z);

    println!("VALUES AFTER ASSIGNMENTS");
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!();

    // load() читает текущее значение атомарного счётчика.
    // AtomicUsize нельзя напрямую вывести через {}.
    let assignments = ASSIGNMENTS.load(Ordering::Relaxed);
    let comparisons = COMPARISONS.load(Ordering::Relaxed);

    println!("OPERATION COUNTERS");
    println!("Assignments: {}", assignments);
    println!("Comparisons: {}", comparisons);
}
