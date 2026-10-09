/*
Cvičení 04 — Úloha 4.4
Název: Obrácení pole
Body: 3 body

Zadání:
Napište funkci, která v zadaném poli obrátí pořadí prvků. První prvek bude posledním, druhý prvek
předposledním, atd. Je zakázáno použít pomocné pole. [3 body]


Ukázka:

int[] pole = { 5, 7, 2, 1, 0, 6 };

ObratPole(pole); // 6, 0, 1, 2, 7, 5

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

// Функция получает изменяемый массив
// и разворачивает его на месте.
fn reverse_array(array: &mut [i32]) {
    // Индекс слева.
    let mut left = 0;

    // Индекс справа.
    let mut right = array.len() - 1;

    // Пока левый индекс меньше правого,
    // меняем элементы местами.
    while left < right {
        // Временно сохраняем левый элемент.
        let saved_number = array[left];

        // На место левого ставим правый.
        array[left] = array[right];

        // На место правого ставим старый левый.
        array[right] = saved_number;

        // Сдвигаемся к центру.
        left += 1;
        right -= 1;
    }
}

// Запускаем пример из упражнения и выводим результаты.
fn main() {
    let mut array = [5, 7, 2, 1, 0, 6];

    println!("Before: {:?}", array);

    reverse_array(&mut array);

    println!("After:   {:?}", array);
}
