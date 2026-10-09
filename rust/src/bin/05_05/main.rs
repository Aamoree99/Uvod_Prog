/*
Cvičení 05 — Úloha 5.5
Název: Binární vyhledávání
Body: 2 body; +1 bod (v pořadí podúloh)

Zadání:
Napište funkci hledání zadané hodnoty v sestupně setříděném poli za použití algoritmu binárního
vyhledávání:

a) Je vrácen index libovolného prvku s hledanou hodnotou. [2 body]

b) Je vrácen index prvního prvku v poli, který má hledanou hodnotu. [+1 bod]


Ukázka

int[] p1 = { 12, 9, 6, 6, 6, 5, 3, 1, 1 };

int index = BinarniVyhledavani(p1, 6);

// (a) index = 2, 3, nebo 4 (b) index = 2

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

// A) Бинарный поиск.
//
// Функция ищет число в массиве,
// отсортированном по убыванию.
//
// Возвращает индекс любого найденного элемента.
// Если число отсутствует, возвращает -1.
fn binary_search(array: &[i32], target: i32) -> isize {
    // Левая граница поиска.
    let mut left: usize = 0;

    // Правая граница поиска (не включительно).
    let mut right: usize = array.len();

    // Продолжаем, пока область поиска не стала пустой.
    while left < right {
        // Находим середину текущей области.
        let middle = left + (right - left) / 2;

        // Если нашли число, возвращаем его индекс.
        if array[middle] == target {
            return middle as isize;
        }

        // Массив отсортирован по убыванию!
        //
        // Если число в середине больше искомого,
        // нужное число может находиться справа.
        if array[middle] > target {
            left = middle + 1;
        } else {
            // Иначе ищем слева, включая middle.
            right = middle;
        }
    }

    // Число не найдено.
    -1
}

// B) Поиск первого совпадения.
//
// Если число встречается несколько раз,
// возвращаем индекс самого левого совпадения.
//
// Например:
// [12, 9, 6, 6, 6, 5, 3]
//
// Для числа 6 результат должен быть 2.
fn binary_search_first(array: &[i32], target: i32) -> isize {
    let mut left: usize = 0;
    let mut right: usize = array.len();

    // Здесь будем хранить индекс найденного числа.
    // -1 означает, что число пока не найдено.
    let mut result: isize = -1;

    while left < right {
        let middle = left + (right - left) / 2;

        if array[middle] == target {
            // Нашли совпадение.
            result = middle as isize;

            // Но это может быть не первое совпадение.
            // Продолжаем искать левее.
            right = middle;
        } else if array[middle] > target {
            // Среднее число больше искомого.
            // При сортировке по убыванию идём вправо.
            left = middle + 1;
        } else {
            // Среднее число меньше искомого.
            // Идём влево.
            right = middle;
        }
    }

    // Возвращаем индекс первого совпадения
    // или -1, если число не найдено.
    result
}

fn main() {
    // Исходный массив из задания.
    let array: [i32; 9] = [12, 9, 6, 6, 6, 5, 3, 1, 1];

    // Искомое число.
    let target: i32 = 6;

    // A) Ищем любое совпадение.
    let index = binary_search(&array, target);

    // B) Ищем первое совпадение.
    let first_index = binary_search_first(&array, target);

    println!("BINARY SEARCH");
    println!("========================================");
    println!("Array: {:?}", array);
    println!("Target: {}", target);
    println!();

    println!("A) Any matching index: {}", index);
    println!("B) First matching index: {}", first_index);

    // Дополнительная проверка числа,
    // которого нет в массиве.
    let missing_number = 10;

    println!();
    println!(
        "Search for missing number {}: {}",
        missing_number,
        binary_search(&array, missing_number)
    );
}
