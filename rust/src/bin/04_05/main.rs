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
// max  - самое большое число
// max2 - второе по величине
// max3 - третье по величине
//
// По условию массив содержит только неотрицательные числа.
fn max_pole(pole: &[i32], max: &mut i32, max2: &mut i32, max3: &mut i32) {
    // Так как числа неотрицательные,
    // можно начать все максимумы с 0.
    *max = 0;
    *max2 = 0;
    *max3 = 0;

    let mut i = 0;

    while i < pole.len() {
        let cislo = pole[i];

        // Если нашли новое самое большое число:
        if cislo > *max {
            // Старое второе становится третьим.
            *max3 = *max2;

            // Старое первое становится вторым.
            *max2 = *max;

            // Новое число становится первым.
            *max = cislo;
        }
        // Если число не больше первого,
        // но больше второго:
        else if cislo > *max2 {
            // Старое второе становится третьим.
            *max3 = *max2;

            // Новое число становится вторым.
            *max2 = cislo;
        }
        // Если число не больше первых двух,
        // но больше третьего:
        else if cislo > *max3 {
            *max3 = cislo;
        }

        i += 1;
    }
}

fn main() {
    let pole = [2, 5, 7, 1, 4, 6, 9, 8, 2];

    let mut max = 0;
    let mut max2 = 0;
    let mut max3 = 0;

    max_pole(&pole, &mut max, &mut max2, &mut max3);

    println!("Pole: {:?}", pole);
    println!("Nejvetsi:       {}", max);
    println!("Druhe nejvetsi: {}", max2);
    println!("Treti nejvetsi: {}", max3);
}
