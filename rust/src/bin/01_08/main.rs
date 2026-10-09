/*
Cvičení 01 — Úloha 1.8
Název: Dělitel čísla
Body: 5 bodů

Zadání:
Ve funkci Main deklarujte tři celočíselné proměnné (pojmenujte je libovolně) a tyto proměnné
inicializujte hodnotami. Napište program, který určí, zda některé z čísel je dělitelem jiného
čísla, nebo zda žádné číslo není dělitelem jiného čísla. Je-li více správných dvojic číslo a jeho
dělitel, uveďte libovolnou z těchto dvojic. Cíl příkladu a jeho výsledek vypište vhodným způsobem
na obrazovku. [5 bodů]


Ukázka:

Hodnota první proměnné je 6, hodnota druhé proměnné je 3 hodnota třetí proměnné je 8.

Ze zadaných čísel 6, 3 a 8 je číslo 3 dělitelem čísla 6.

Hodnota první proměnné je 7, hodnota druhé proměnné je 2 hodnota třetí proměnné je 5.

Ze zadaných čísel 7, 2 a 5 není žádné číslo dělitelem jiného čísla.

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

// Запускаем пример из упражнения и выводим результаты.
fn main() {
    let a: i32 = 2;
    let b: i32 = 6;
    let c: i32 = 4;

    // Флаг запоминает, нашли ли хотя бы одну подходящую пару.
    let mut found: bool = false;

    // Проверяем все шесть направлений делимости между тремя числами.
    // Условие слева от && проверяется первым: при нулевом делителе остаток не вычисляется.
    if a != 0 && b % a == 0 {
        println!("The number {a} is a divisor of {b}.");
        found = true;
    }

    if b != 0 && a % b == 0 {
        println!("The number {b} is a divisor of {a}.");
        found = true;
    }

    if a != 0 && c % a == 0 {
        println!("The number {a} is a divisor of {c}.");
        found = true;
    }

    if c != 0 && a % c == 0 {
        println!("The number {c} is a divisor of {a}.");
        found = true;
    }

    if b != 0 && c % b == 0 {
        println!("The number {b} is a divisor of {c}.");
        found = true;
    }

    if c != 0 && b % c == 0 {
        println!("The number {c} is a divisor of {b}.");
        found = true;
    }

    // Сообщение об отсутствии делителей выводим только после проверки всех пар.
    // Этот вариант выводит все найденные пары, хотя по условию достаточно одной.
    if found == false {
        println!("Among the numbers {a}, {b} and {c} no number divides another.");
    }
}
