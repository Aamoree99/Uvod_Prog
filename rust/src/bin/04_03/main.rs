/*
Cvičení 04 — Úloha 4.3
Název: Zaokrouhlení pole
Body: 2 body

Zadání:
Napište funkci, která pro zadané pole reálných čísel vrátí pole celých čísel. Hodnoty prvků v novém
poli vzniknou zaokrouhlením hodnot prvků z původního pole. [2 body]


Ukázka:

double[] pole = { 5.12, -3.5996, 156, 7.9 };

zaokrouhli = ZaokrouhliPole(pole); // 5, -4, 156, 8

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

// Функция получает массив вещественных чисел
// и возвращает новый массив целых чисел.
fn round_array(array: [f64; 4]) -> [i32; 4] {
    // Создаём новый массив из четырёх нулей.
    // Сюда будем записывать округлённые значения.
    let mut result = [0; 4];

    let mut i = 0;

    // Проходим по всем элементам исходного массива.
    while i < array.len() {
        // round() округляет число до ближайшего целого.
        //
        // Например:
        // 5.12    -> 5.0
        // -3.5996 -> -4.0
        // 7.9     -> 8.0
        //
        // После round() тип всё ещё f64,
        // поэтому через "as i32" превращаем его в целое число.
        result[i] = array[i].round() as i32;

        i += 1;
    }

    // Возвращаем готовый массив.
    result
}

// Запускаем пример из упражнения и выводим результаты.
fn main() {
    // Исходный массив вещественных чисел.
    let array: [f64; 4] = [5.12, -3.5996, 156.0, 7.9];

    // Вызываем функцию и сохраняем новый массив.
    let rounded = round_array(array);

    // Показываем исходный массив.
    println!("Original:      {:?}", array);

    // Показываем округлённый массив.
    println!("Rounded: {:?}", rounded);
}
