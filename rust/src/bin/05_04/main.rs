/*
Cvičení 05 — Úloha 5.4
Název: Testování rychlostí algoritmů
Body: 2 body; 2 body (v pořadí podúloh)

Zadání:
Otestujte třídící algoritmy z příkladu 5.3 na polích různé velikosti (1000, 10 000 a 30 000 prvků),
naplněných náhodnými hodnotami z intervalu 0 až velikost pole. Všechny třídící algoritmy budou
pracovat nad kopií stejného pole. Výsledky přehledně prezentujte.

a) s použitím počtu operací [2 body]

b) s použitím měření času [2 body]


Ukázka – část (a):

Prvku 1000

Selection sort – porovnani: 500500, prirazeni: 3000

Insertion sort – porovnani: 259065, prirazeni: 260069

Bubble sort – porovnani: 974025, prirazeni: 774213

Count sort – porovnani: 2995, prirazeni: 3006

Prvku 10000

...

Prvku 30000

...


Ukázka – část (b):

Prvku 1000

...

Prvku 10000

...

Prvku 30000

Selection sort – doba behu 11194 ms

Insertion sort – doba behu 9300 ms

Bubble sort – doba behu 39415 ms

Count sort – doba behu 4 ms

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

// Максимальный размер тестового массива.
// По заданию проверяем 1000, 10000 и 30000 элементов.
const MAX_SIZE: usize = 30_000;

// Глобальные счётчики из задания 5.2.
static ASSIGNMENTS: AtomicU64 = AtomicU64::new(0);
static COMPARISONS: AtomicU64 = AtomicU64::new(0);

// Сбрасываем счётчики перед каждой сортировкой.
fn reset_counters() {
    ASSIGNMENTS.store(0, Ordering::Relaxed);
    COMPARISONS.store(0, Ordering::Relaxed);
}

// Присваиваем значение и увеличиваем счётчик.
fn assign(target: &mut i32, value: i32) {
    ASSIGNMENTS.fetch_add(1, Ordering::Relaxed);
    *target = value;
}

// Проверяем, меньше ли первое число второго.
fn is_less(a: i32, b: i32) -> bool {
    COMPARISONS.fetch_add(1, Ordering::Relaxed);
    a < b
}

// Проверяем, больше ли первое число второго.
fn is_greater(a: i32, b: i32) -> bool {
    COMPARISONS.fetch_add(1, Ordering::Relaxed);
    a > b
}

// A) SELECTION SORT
//
// На каждом шаге ищем максимальный элемент
// и переносим его в начало неотсортированной части.
fn selection_sort(array: &mut [i32]) {
    let mut i = 0;

    while i < array.len() {
        let mut max_index = i;
        let mut j = i + 1;

        // Ищем наибольшее число справа.
        while j < array.len() {
            if is_greater(array[j], array[max_index]) {
                max_index = j;
            }

            j += 1;
        }

        // Сохраняем оба значения до изменения массива,
        // чтобы не нарушать правила заимствования Rust.
        let mut temp = 0;
        assign(&mut temp, array[i]);

        let max_value = array[max_index];

        // Меняем элементы местами.
        assign(&mut array[i], max_value);
        assign(&mut array[max_index], temp);

        i += 1;
    }
}

// B) INSERTION SORT
//
// Берём очередной элемент и вставляем его
// в правильную позицию отсортированной части.
fn insertion_sort(array: &mut [i32]) {
    let mut i = 1;

    while i < array.len() {
        let mut key = 0;
        assign(&mut key, array[i]);

        let mut j = i;

        // Сдвигаем меньшие числа вправо.
        while j > 0 {
            if is_less(array[j - 1], key) {
                let value = array[j - 1];
                assign(&mut array[j], value);

                j -= 1;
            } else {
                break;
            }
        }

        // Вставляем число на найденное место.
        assign(&mut array[j], key);

        i += 1;
    }
}

// C) BUBBLE SORT
//
// Сравниваем соседние элементы.
// Если левый меньше правого — меняем их местами.
fn bubble_sort(array: &mut [i32]) {
    let mut i = 0;

    while i < array.len() {
        let mut j = 0;

        // После каждого прохода один из минимальных
        // элементов оказывается в конце массива.
        while j + 1 < array.len() - i {
            if is_less(array[j], array[j + 1]) {
                let mut temp = 0;
                assign(&mut temp, array[j]);

                let next = array[j + 1];

                assign(&mut array[j], next);
                assign(&mut array[j + 1], temp);
            }

            j += 1;
        }

        i += 1;
    }
}

// D) COUNTING SORT
//
// Считаем, сколько раз встречается каждое число,
// а потом записываем значения по убыванию.
//
// Для нашей задачи числа находятся в диапазоне
// от 0 до 30000, поэтому размер массива
// частот известен заранее.
fn counting_sort(array: &mut [i32]) {
    if array.len() == 0 {
        return;
    }

    // Массив частот.
    // Индекс — значение числа.
    // Элемент — сколько раз число встретилось.
    let mut counts = [0; MAX_SIZE + 1];

    let mut max_value = 0;
    let mut i = 0;

    // Подсчитываем частоты и находим максимум.
    while i < array.len() {
        let value = array[i];

        if is_greater(value, max_value) {
            assign(&mut max_value, value);
        }

        let index = value as usize;
        let new_count = counts[index] + 1;

        assign(&mut counts[index], new_count);

        i += 1;
    }

    // Начинаем записывать элементы
    // с самого большого значения.
    let mut position = 0;
    let mut number = max_value;

    loop {
        let index = number as usize;

        // Записываем число столько раз,
        // сколько оно встретилось в исходном массиве.
        while is_greater(counts[index], 0) {
            assign(&mut array[position], number);

            let new_count = counts[index] - 1;
            assign(&mut counts[index], new_count);

            position += 1;
        }

        // После нуля все значения обработаны.
        if number == 0 {
            break;
        }

        number -= 1;
    }
}

// Проверяем результат сортировки.
//
// Если текущий элемент больше предыдущего,
// значит порядок убывания нарушен.
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

// Заполняем массив случайными числами.
//
// Для массива размера 1000:
// числа от 0 до 1000 включительно.
//
// Для массива размера 30000:
// числа от 0 до 30000 включительно.
fn fill_random(array: &mut [i32]) {
    let mut rng = rand::thread_rng();

    let mut i = 0;

    while i < array.len() {
        array[i] = rng.gen_range(0..=array.len() as i32);
        i += 1;
    }
}

// Запускаем выбранный алгоритм.
//
// algorithm:
// 0 = Selection sort
// 1 = Insertion sort
// 2 = Bubble sort
// 3 = Counting sort
fn run_sort(algorithm: i32, array: &mut [i32]) {
    match algorithm {
        0 => selection_sort(array),
        1 => insertion_sort(array),
        2 => bubble_sort(array),
        3 => counting_sort(array),
        _ => println!("Unknown sorting algorithm."),
    }
}

// Возвращаем название алгоритма.
fn algorithm_name(algorithm: i32) -> &'static str {
    match algorithm {
        0 => "Selection sort",
        1 => "Insertion sort",
        2 => "Bubble sort",
        3 => "Counting sort",
        _ => "Unknown",
    }
}

// Тестируем один алгоритм на заданном массиве.
fn benchmark(algorithm: i32, original: &[i32]) {
    // Создаём отдельный массив для сортировки.
    // Используем обычный массив, а не Vec.
    let mut working = [0; MAX_SIZE];

    // Копируем исходные данные.
    // Копирование не входит в измеряемое время.
    working[..original.len()].copy_from_slice(original);

    // Берём только заполненную часть массива.
    let array = &mut working[..original.len()];

    // Сбрасываем счётчики операций.
    reset_counters();

    // Засекаем время непосредственно перед сортировкой.
    let start = Instant::now();

    // Запускаем выбранный алгоритм.
    run_sort(algorithm, array);

    // Получаем продолжительность выполнения.
    let duration = start.elapsed();

    // Читаем счётчики ПОСЛЕ сортировки.
    let assignments = ASSIGNMENTS.load(Ordering::Relaxed);
    let comparisons = COMPARISONS.load(Ordering::Relaxed);

    // Проверяем правильность результата.
    // Эта проверка не входит в измеряемое время
    // и не изменяет глобальные счётчики.
    let correct = is_descending(array);

    // Переводим время в миллисекунды.
    let milliseconds = duration.as_secs_f64() * 1000.0;

    // Показываем название, операции, время и результат.
    println!(
        "{:<16} | {:>12} | {:>12} | {:>12.3} | {}",
        algorithm_name(algorithm),
        comparisons,
        assignments,
        milliseconds,
        if correct { "OK" } else { "FAILED" }
    );
}

// Тестируем все четыре алгоритма
// на массивах заданного размера.
fn benchmark_size(size: usize) {
    // Создаём исходный массив.
    let mut original = [0; MAX_SIZE];

    // Заполняем случайными числами только нужную часть.
    fill_random(&mut original[..size]);

    println!();
    println!("ARRAY SIZE: {}", size);
    println!("{}", "-".repeat(81));

    println!(
        "{:<16} | {:>12} | {:>12} | {:>12} | {}",
        "Algorithm", "Comparisons", "Assignments", "Time (ms)", "Sorted"
    );

    println!("{}", "-".repeat(81));

    // Запускаем четыре сортировки.
    // Каждая получит одинаковые исходные данные.
    let mut algorithm = 0;

    while algorithm < 4 {
        benchmark(algorithm, &original[..size]);
        algorithm += 1;
    }
}

fn main() {
    println!("SORTING ALGORITHM BENCHMARK");
    println!("========================================");
    println!("Order: Descending");
    println!("Data: Random integers");
    println!("Algorithms: Selection, Insertion, Bubble, Counting");

    // Проверяем три размера из задания.
    benchmark_size(1_000);
    benchmark_size(10_000);
    benchmark_size(30_000);

    println!();
    println!("Benchmark completed.");
}
