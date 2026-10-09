/*
Cvičení 05 — Úloha 5.7
Název: Vylepšený Count Sort
Body: 3 body

Zadání:
Rozšiřte funkci z příkladu  5.3d pro sestupné setřídění pole pomocí třídícího algoritmu Count Sort.
Hodnoty prvků pole jsou z omezeného intervalu, který může obsahovat i záporná čísla [3 body]


Ukázka:

int[] p1 = { -5, 7, 15, 22, 4, 0, 4, 9, -11 };

CountSort(p1); // 22, 15, 9, 7, 4, 4, 0, -5, -11

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

// Глобальные счётчики из задания 5.2.
static ASSIGNMENTS: AtomicUsize = AtomicUsize::new(0);
static COMPARISONS: AtomicUsize = AtomicUsize::new(0);

// Присваиваем число и увеличиваем счётчик.
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

// Улучшенная сортировка подсчётом по убыванию.
//
// Теперь поддерживаются отрицательные числа.
//
// min_value используется как смещение:
// индекс числа = число - min_value.
//
// Массив counts хранит количество повторений
// каждого числа в диапазоне от min_value до max_value.
fn counting_sort(array: &mut [i32]) {
    // Пустой массив не нужно сортировать.
    if array.len() == 0 {
        return;
    }

    // Начальные минимум и максимум.
    let mut min_value = array[0];
    let mut max_value = array[0];

    let mut i = 1;

    // Находим минимальное и максимальное число.
    while i < array.len() {
        if is_less(array[i], min_value) {
            assign(&mut min_value, array[i]);
        }

        if is_greater(array[i], max_value) {
            assign(&mut max_value, array[i]);
        }

        i += 1;
    }

    // Вычисляем, сколько разных целых значений
    // может находиться между минимумом и максимумом.
    //
    // i64 нужен, чтобы избежать переполнения i32
    // при вычитании отрицательных чисел.
    let range = (max_value as i64 - min_value as i64 + 1) as usize;

    // Создаём массив частот нужной длины.
    //
    // Box<[i32]> — обычный массив фиксированной длины,
    // размещённый в памяти через Box.
    //
    // Это не Vec и не коллекция List.
    let mut counts = vec![0; range].into_boxed_slice();

    // Подсчитываем частоту каждого числа.
    i = 0;

    while i < array.len() {
        // Смещаем число так, чтобы индекс был >= 0.
        //
        // Например:
        // min_value = -11
        // array[i] = -5
        // index = -5 - (-11) = 6
        let index = (array[i] as i64 - min_value as i64) as usize;

        let new_count = counts[index] + 1;

        // Считаем присваивание элементу массива частот.
        assign(&mut counts[index], new_count);

        i += 1;
    }

    // Записываем значения обратно по убыванию.
    let mut position = 0;
    let mut index = range;

    // Идём по массиву частот с конца к началу.
    while index > 0 {
        index -= 1;

        // Восстанавливаем исходное значение числа.
        let number = (index as i64 + min_value as i64) as i32;

        // Записываем число столько раз,
        // сколько оно встретилось в массиве.
        while is_greater(counts[index], 0) {
            assign(&mut array[position], number);

            let new_count = counts[index] - 1;
            assign(&mut counts[index], new_count);

            position += 1;
        }
    }
}

// Проверяем, отсортирован ли массив по убыванию.
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

fn main() {
    // Пример из задания.
    let mut array = [-5, 7, 15, 22, 4, 0, 4, 9, -11];

    println!("IMPROVED COUNTING SORT");
    println!("========================================");

    println!("Original array: {:?}", array);

    // Сортируем массив по убыванию.
    counting_sort(&mut array);

    println!("Sorted array:   {:?}", array);

    // Проверяем правильность сортировки.
    println!("Sorted correctly: {}", is_descending(&array));

    // Выводим количество операций.
    println!();
    println!("OPERATION COUNTERS");
    println!("Assignments: {}", ASSIGNMENTS.load(Ordering::Relaxed));
    println!("Comparisons: {}", COMPARISONS.load(Ordering::Relaxed));
}
