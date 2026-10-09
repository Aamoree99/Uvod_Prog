/*
Cvičení 04 — Úloha 4.5
Název: Maximum z pole
Body: 1 bod; 1 bod; 2 body (v pořadí podúloh)

Zadání:
Napište funkci, která v zadaném poli nezáporných čísel najde a pomocí parametrů funkce vrátí:

a) hodnotu největšího prvku [1 bod]

b) hodnotu druhého největšího prvku [1 bod]

c) třetího největšího prvku [2 body]


Ukázka:

int[] pole = { 2, 5, 7, 1, 4, 6, 9, 8, 2 };

MaxPole(pole, ref max, ref max2, ref max3); // 9, 8, 7

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

// Функция ищет:
// largest  - самое большое число
// second_largest - второе по величине
// third_largest - третье по величине
//
// По условию массив содержит только неотрицательные числа.
fn array_maxima(
    array: &[i32],
    largest: &mut i32,
    second_largest: &mut i32,
    third_largest: &mut i32,
) {
    // Так как числа неотрицательные,
    // можно начать все максимумы с 0.
    *largest = 0;
    *second_largest = 0;
    *third_largest = 0;

    let mut i = 0;

    while i < array.len() {
        let number = array[i];

        // Если нашли новое самое большое число:
        if number > *largest {
            // Старое второе становится третьим.
            *third_largest = *second_largest;

            // Старое первое становится вторым.
            *second_largest = *largest;

            // Новое число становится первым.
            *largest = number;
        }
        // Если число не больше первого,
        // но больше второго:
        else if number > *second_largest {
            // Старое второе становится третьим.
            *third_largest = *second_largest;

            // Новое число становится вторым.
            *second_largest = number;
        }
        // Если число не больше первых двух,
        // но больше третьего:
        else if number > *third_largest {
            *third_largest = number;
        }

        i += 1;
    }
}

// Запускаем пример из упражнения и выводим результаты.
fn main() {
    let array = [2, 5, 7, 1, 4, 6, 9, 8, 2];

    let mut largest = 0;
    let mut second_largest = 0;
    let mut third_largest = 0;

    array_maxima(
        &array,
        &mut largest,
        &mut second_largest,
        &mut third_largest,
    );

    println!("Array: {:?}", array);
    println!("Largest:       {}", largest);
    println!("Second largest: {}", second_largest);
    println!("Third largest: {}", third_largest);
}
