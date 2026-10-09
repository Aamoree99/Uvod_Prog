/*
Cvičení 05 — Úloha 5.1
Název: Test setříděnosti pole.
Body: 1 bod

Zadání:
Napište funkci, která rozhodne, zda zadané pole je sestupně setříděné. Upozornění: ve studijních
materiálech jsme pracovali se vzestupně setříděnými poli. [1 bod]


Ukázka:

int[] pole = { 15, 12, 9, 9, 8, 5, 2, 2, 1 };

bool je = JeSestupne(pole); // true

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

// Функция проверяет, отсортирован ли массив по убыванию.
//
// array — массив целых чисел.
// Возвращает:
// true  — массив отсортирован по убыванию.
// false — порядок элементов нарушен.
fn is_descending(array: &[i32]) -> bool {
    // Начинаем со второго элемента,
    // потому что будем сравнивать его с предыдущим.
    let mut i: usize = 1;

    // Проходим по всем элементам массива.
    while i < array.len() {
        // Если текущий элемент больше предыдущего,
        // значит порядок убывания нарушен.
        if array[i] > array[i - 1] {
            return false;
        }

        // Переходим к следующему элементу.
        i += 1;
    }

    // Если дошли до конца без нарушений,
    // значит массив отсортирован правильно.
    true
}

fn main() {
    // Исходный массив из задания.
    let array: [i32; 9] = [15, 12, 9, 9, 8, 5, 2, 2, 1];

    // Вызываем функцию и сохраняем результат проверки.
    let is_sorted = is_descending(&array);

    // Выводим исходный массив.
    println!("Array: {:?}", array);

    // Выводим результат проверки.
    if is_sorted == true {
        println!("The array is sorted in descending order.");
    } else {
        println!("The array is not sorted in descending order.");
    }
}
