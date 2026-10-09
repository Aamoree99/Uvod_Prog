/*
Cvičení 05 — Úloha 5.6
Název: Vylepšený Bubble Sort
Body: 2 body; 1 bod (v pořadí podúloh)

Zadání:
Algoritmus Bubble Sort třídící sestupně zadané pole lze implementovat ve dvou verzích. V základní
verzi algoritmu se ve vnořeném cyklu prochází pole až do posledního prvku.  Ve vylepšené verzi
algoritmu se ve vnořeném cyklu prochází pole jen do místa poslední výměny, která proběhla
v předchozí iteraci vnějšího cyklu.

a) Napište vylepšenou verzi Bubble sortu. Pokud jste vylepšenou verzi napsali již v příklad 5.3c,
napište nyní verzi základní. [2 body]

b) Změřte (dle příkladu 5.4) počty operací a dobu běhu vylepšené a základní verze. [1 bod]

Ukázka:

Prvku 1000

Bubble sort – porovnani: 944055, prirazeni: 763212

Bubble sort2 – porovnani: 496415, prirazeni: 763212

Bubble sort – doba behu 46 ms

Bubble sort2 – doba behu 30 ms

Prvku 10000

...

Prvku 30000

...

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

use rand::Rng;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

// Глобальные счётчики из задания 5.2.
static ASSIGNMENTS: AtomicU64 = AtomicU64::new(0);
static COMPARISONS: AtomicU64 = AtomicU64::new(0);

// Максимальный размер тестового массива.
const MAX_SIZE: usize = 30_000;

// Сбрасываем счётчики перед каждым тестом.
fn reset_counters() {
    ASSIGNMENTS.store(0, Ordering::Relaxed);
    COMPARISONS.store(0, Ordering::Relaxed);
}

// Присваиваем значение и увеличиваем счётчик.
fn assign(target: &mut i32, value: i32) {
    ASSIGNMENTS.fetch_add(1, Ordering::Relaxed);
    *target = value;
}

// Сравниваем два числа и увеличиваем счётчик.
fn is_less(a: i32, b: i32) -> bool {
    COMPARISONS.fetch_add(1, Ordering::Relaxed);
    a < b
}

// Меняем два соседних элемента местами.
//
// Используем функцию assign(), чтобы
// каждый обмен учитывался в счётчике.
//
// Один обмен = три присваивания.
fn swap_elements(array: &mut [i32], index: usize) {
    let mut temp = 0;

    assign(&mut temp, array[index]);

    // Сначала читаем значение следующего элемента,
    // чтобы избежать ошибки заимствования Rust.
    let next = array[index + 1];

    assign(&mut array[index], next);
    assign(&mut array[index + 1], temp);
}

// --------------------------------------------------
// A) BASIC BUBBLE SORT
// --------------------------------------------------

// Обычная пузырьковая сортировка.
//
// На каждом проходе проверяем все соседние пары.
// Если левое число меньше правого,
// меняем их местами.
//
// Не сокращаем область проверки.
fn bubble_sort(array: &mut [i32]) {
    if array.len() < 2 {
        return;
    }

    let mut i = 0;

    while i < array.len() - 1 {
        let mut j = 0;

        // Каждый раз проходим до конца массива.
        while j + 1 < array.len() {
            if is_less(array[j], array[j + 1]) {
                swap_elements(array, j);
            }

            j += 1;
        }

        i += 1;
    }
}

// --------------------------------------------------
// B) IMPROVED BUBBLE SORT
// --------------------------------------------------

// Улучшенная пузырьковая сортировка.
//
// Запоминаем позицию последнего обмена.
//
// Все элементы правее этой позиции уже стоят
// в правильном порядке относительно друг друга.
//
// Поэтому на следующем проходе проверяем
// только часть массива до последнего обмена.
fn improved_bubble_sort(array: &mut [i32]) {
    // Правая граница проверяемой области.
    let mut limit = array.len();

    while limit > 1 {
        // Позиция последнего обмена.
        // Если обменов не было, останется 0.
        let mut last_swap = 0;

        let mut j = 0;

        // Проверяем соседние элементы
        // только до текущей границы.
        while j + 1 < limit {
            if is_less(array[j], array[j + 1]) {
                swap_elements(array, j);

                // Запоминаем новую границу.
                // Обмен был между j и j + 1.
                last_swap = j + 1;
            }

            j += 1;
        }

        // Всё правее last_swap уже отсортировано.
        // Если обменов не было, limit станет 0
        // и сортировка завершится.
        limit = last_swap;
    }
}

// --------------------------------------------------
// RANDOM DATA
// --------------------------------------------------

// Заполняем массив случайными числами
// от 0 до размера массива включительно.
fn fill_random(array: &mut [i32]) {
    let mut rng = rand::thread_rng();
    let mut i = 0;

    while i < array.len() {
        array[i] = rng.gen_range(0..=array.len() as i32);
        i += 1;
    }
}

// Проверяем, отсортирован ли массив по убыванию.
//
// Эта функция не использует счётчики операций,
// поскольку проверка не является частью сортировки.
fn is_descending(array: &[i32]) -> bool {
    let mut i = 1;

    while i < array.len() {
        if array[i] > array[i - 1] {
            return false;
        }

        i += 1;
    }

    true
}

// --------------------------------------------------
// BENCHMARK
// --------------------------------------------------

// Тестируем одну из двух сортировок.
//
// improved = false — обычная версия.
// improved = true  — улучшенная версия.
fn benchmark(original: &[i32], improved: bool) {
    // Создаём отдельный массив без Vec.
    // Каждый алгоритм получает одинаковые данные.
    let mut working = [0; MAX_SIZE];

    // Копирование выполняем до измерения времени.
    working[..original.len()].copy_from_slice(original);

    let array = &mut working[..original.len()];

    reset_counters();

    // Начинаем измерение времени.
    let start = Instant::now();

    if improved == true {
        improved_bubble_sort(array);
    } else {
        bubble_sort(array);
    }

    // Останавливаем измерение после сортировки.
    let duration = start.elapsed();

    // Читаем количество выполненных операций.
    let comparisons = COMPARISONS.load(Ordering::Relaxed);
    let assignments = ASSIGNMENTS.load(Ordering::Relaxed);

    // Проверяем правильность результата.
    let correct = is_descending(array);

    // Переводим время в миллисекунды.
    let milliseconds = duration.as_secs_f64() * 1000.0;

    let name = if improved == true {
        "Improved Bubble"
    } else {
        "Basic Bubble"
    };

    // Выводим строку таблицы.
    println!(
        "{:<17} | {:>12} | {:>12} | {:>12.3} | {}",
        name,
        comparisons,
        assignments,
        milliseconds,
        if correct { "OK" } else { "FAILED" }
    );
}

// Тестируем обе сортировки на массиве заданного размера.
fn benchmark_size(size: usize) {
    // Исходный массив для обоих алгоритмов.
    let mut original = [0; MAX_SIZE];

    fill_random(&mut original[..size]);

    println!();
    println!("ARRAY SIZE: {}", size);
    println!("{}", "-".repeat(82));

    println!(
        "{:<17} | {:>12} | {:>12} | {:>12} | {}",
        "Algorithm", "Comparisons", "Assignments", "Time (ms)", "Sorted"
    );

    println!("{}", "-".repeat(82));

    // Оба алгоритма получают копию original.
    benchmark(&original[..size], false);
    benchmark(&original[..size], true);
}

// --------------------------------------------------
// MAIN
// --------------------------------------------------

fn main() {
    println!("BUBBLE SORT PERFORMANCE COMPARISON");
    println!("========================================");
    println!("Order: Descending");
    println!("Data: Random integers");

    // Дополнительная проверка на небольшом массиве.
    let example = [5, 7, 1, 5, 6, 8, 9, 0, 1, 2];

    let mut basic_example = example;
    let mut improved_example = example;

    bubble_sort(&mut basic_example);
    improved_bubble_sort(&mut improved_example);

    println!();
    println!("EXAMPLE");
    println!("Original: {:?}", example);
    println!("Basic:    {:?}", basic_example);
    println!("Improved: {:?}", improved_example);

    // Тесты производительности из задания.
    benchmark_size(1_000);
    benchmark_size(10_000);
    benchmark_size(30_000);

    println!();
    println!("Benchmark completed.");
}
