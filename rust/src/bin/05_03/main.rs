/*
Cvičení 05 — Úloha 5.3
Název: Sestupné třídění
Body: 3+1 body; 3+1 body; 2+1 body; 3+2 body (v pořadí podúloh)

Zadání:
Napište třídící funkce pro sestupné třídění. Body navíc lze získat za použití pomocných funkcí
z příkladu 5.2 pro porovnávání a přiřazování hodnot prvků. Pomocné funkce nepoužívejte pro práci
s řídící proměnnou cyklu. Výjimkou je Count sort, u kterého pomocné funkce se používají pro řídící
proměnnou cyklu v situacích, ve kterých je řídící proměnná cyklu brána jako prvek pole.

a) Selection sort [3+1 body]

b) Insertion sort [3+1  body]

c) Bubble sort [2+1  body]

d) Count sort [3+2 body]


Ukázka:

int[] pole = { 5, 7, 1, 5, 6, 8, 9, 0, 1, 2};

SelectionSort(pole); // 9, 8, 7, 6, 5, 5, 2, 1, 1, 0

Console.WriteLine("Prirazeni: {0}, Porovnani: {1}", prirazeni, porovnani); // 30,  55

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

// Глобальные счётчики операций.
// Используем атомарные переменные, чтобы обойтись без unsafe.
static ASSIGNMENTS: AtomicUsize = AtomicUsize::new(0);
static COMPARISONS: AtomicUsize = AtomicUsize::new(0);

// Сбрасываем оба счётчика перед новой сортировкой.
fn reset_counters() {
    ASSIGNMENTS.store(0, Ordering::Relaxed);
    COMPARISONS.store(0, Ordering::Relaxed);
}

// Присваиваем значение через изменяемую ссылку.
// Каждое использование функции считается одним присваиванием.
fn assign(target: &mut i32, value: i32) {
    ASSIGNMENTS.fetch_add(1, Ordering::Relaxed);
    *target = value;
}

// Проверяем, меньше ли первое число второго.
// Одновременно увеличиваем счётчик сравнений.
fn is_less(a: i32, b: i32) -> bool {
    COMPARISONS.fetch_add(1, Ordering::Relaxed);
    a < b
}

// Проверяем, больше ли первое число второго.
// Используем тот же счётчик сравнений.
fn is_greater(a: i32, b: i32) -> bool {
    COMPARISONS.fetch_add(1, Ordering::Relaxed);
    a > b
}

// --------------------------------------------------
// A) SELECTION SORT
// --------------------------------------------------

// Сортировка выбором.
//
// На каждом шаге ищем максимальный элемент
// в неотсортированной части массива.
//
// Затем ставим его в начало этой части.
fn selection_sort(array: &mut [i32]) {
    let mut i = 0;

    while i < array.len() {
        // Индекс максимального элемента.
        let mut max_index = i;

        // Ищем максимальное число справа.
        let mut j = i + 1;

        while j < array.len() {
            if is_greater(array[j], array[max_index]) {
                max_index = j;
            }

            j += 1;
        }

        // Сохраняем значения перед обменом.
        // Так мы не нарушаем правила заимствования Rust.
        let mut temp = 0;
        assign(&mut temp, array[i]);

        // Сначала читаем значение максимального элемента.
        let max_value = array[max_index];

        // Затем записываем его на позицию i.
        assign(&mut array[i], max_value);

        // На прежнее место максимума ставим старый array[i].
        assign(&mut array[max_index], temp);

        i += 1;
    }
}
// --------------------------------------------------
// B) INSERTION SORT
// --------------------------------------------------

// Сортировка вставками.
//
// Представляем, что левая часть массива
// уже отсортирована по убыванию.
//
// Берём следующий элемент и вставляем его
// в правильную позицию внутри левой части.
fn insertion_sort(array: &mut [i32]) {
    let mut i = 1;

    while i < array.len() {
        // Запоминаем элемент, который будем вставлять.
        let mut key = 0;
        assign(&mut key, array[i]);

        let mut j = i;

        // Сдвигаем меньшие элементы вправо,
        // чтобы освободить место для key.
        while j > 0 {
            if is_less(array[j - 1], key) {
                let value = array[j - 1];
                assign(&mut array[j], value);

                j -= 1;
            } else {
                break;
            }
        }

        // Вставляем элемент на найденную позицию.
        assign(&mut array[j], key);

        i += 1;
    }
}

// --------------------------------------------------
// C) BUBBLE SORT
// --------------------------------------------------

// Пузырьковая сортировка.
//
// Сравниваем соседние элементы.
//
// Если левый меньше правого,
// меняем их местами.
//
// После каждого полного прохода
// минимальный элемент оказывается справа.
fn bubble_sort(array: &mut [i32]) {
    let mut i = 0;

    while i < array.len() {
        let mut j = 0;

        // Последние i элементов уже стоят на своих местах.
        while j + 1 < array.len() - i {
            if is_less(array[j], array[j + 1]) {
                // Обмениваем два соседних элемента.
                let mut temp = 0;

                assign(&mut temp, array[j]);

                // Сначала считываем следующее значение,
                // затем записываем его в текущую позицию.
                let next = array[j + 1];
                assign(&mut array[j], next);

                assign(&mut array[j + 1], temp);
            }

            j += 1;
        }

        i += 1;
    }
}

// --------------------------------------------------
// D) COUNTING SORT
// --------------------------------------------------

// Сортировка подсчётом.
//
// Работает с неотрицательными целыми числами.
//
// Сначала находим максимальное значение.
// Затем подсчитываем частоту каждого числа.
// После этого записываем числа обратно
// в исходный массив по убыванию.
//
// В отличие от предыдущих алгоритмов,
// здесь нужен вспомогательный массив частот.
fn counting_sort(array: &mut [i32]) {
    // Пустой массив уже отсортирован.
    if array.len() == 0 {
        return;
    }

    // Находим максимальное значение.
    let mut max_value = 0;

    let mut i = 0;

    while i < array.len() {
        if is_greater(array[i], max_value) {
            assign(&mut max_value, array[i]);
        }

        i += 1;
    }

    // Создаём массив счётчиков.
    //
    // Для учебного варианта ограничиваем
    // диапазон значений числами от 0 до 1000.
    //
    // counts[5] показывает количество пятёрок,
    // counts[8] — количество восьмёрок и т.д.
    const MAX_NUMBER: usize = 1000;

    assert!(
        max_value >= 0 && max_value as usize <= MAX_NUMBER,
        "Counting sort supports values from 0 to 1000"
    );

    let mut counts = [0; MAX_NUMBER + 1];

    // Подсчитываем, сколько раз встречается каждое число.
    i = 0;

    while i < array.len() {
        let index = array[i] as usize;

        // Используем вспомогательную функцию,
        // поскольку меняем элемент массива counts.
        let new_count = counts[index] + 1;
        assign(&mut counts[index], new_count);

        i += 1;
    }

    // Записываем отсортированные значения обратно.
    let mut position = 0;

    // Начинаем с самого большого числа.
    let mut number = max_value;

    loop {
        // Пока частота данного числа больше нуля,
        // записываем его в исходный массив.
        while is_greater(counts[number as usize], 0) {
            assign(&mut array[position], number);

            // Уменьшаем частоту использованного числа.
            let index = number as usize;
            let new_count = counts[index] - 1;
            assign(&mut counts[index], new_count);

            // position — управляющий индекс, а не элемент массива.
            position += 1;
        }

        // После нуля заканчиваем сортировку.
        if number == 0 {
            break;
        }

        number -= 1;
    }
}

// --------------------------------------------------
// RESULTS
// --------------------------------------------------

// Выводим отсортированный массив
// и количество выполненных операций.
fn print_results(name: &str, array: &[i32]) {
    let assignments = ASSIGNMENTS.load(Ordering::Relaxed);
    let comparisons = COMPARISONS.load(Ordering::Relaxed);

    println!("----------------------------------------");
    println!("{}", name);
    println!("Sorted array: {:?}", array);
    println!("Assignments: {}", assignments);
    println!("Comparisons: {}", comparisons);
    println!();
}

// --------------------------------------------------
// MAIN
// --------------------------------------------------

fn main() {
    // Исходный массив из задания.
    let original = [5, 7, 1, 5, 6, 8, 9, 0, 1, 2];

    println!("DESCENDING SORTING ALGORITHMS");
    println!("========================================");
    println!("Original array: {:?}", original);
    println!();

    // Для каждого алгоритма создаём отдельную копию,
    // чтобы все начинали с одинаковых данных.

    // A) Selection sort.
    let mut selection_array = original;

    reset_counters();
    selection_sort(&mut selection_array);
    print_results("A) Selection sort", &selection_array);

    // B) Insertion sort.
    let mut insertion_array = original;

    reset_counters();
    insertion_sort(&mut insertion_array);
    print_results("B) Insertion sort", &insertion_array);

    // C) Bubble sort.
    let mut bubble_array = original;

    reset_counters();
    bubble_sort(&mut bubble_array);
    print_results("C) Bubble sort", &bubble_array);

    // D) Counting sort.
    let mut counting_array = original;

    reset_counters();
    counting_sort(&mut counting_array);
    print_results("D) Counting sort", &counting_array);
}
