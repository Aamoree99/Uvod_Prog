/*
Cvičení 02 — Úloha 2.3
Název: Dvojciferná čísla
Body: 1 bod

Zadání:
Vypište všechna dvojciferná čísla. [1 bod]


Ukázka:

Vsechna dvojciferna cisla jsou: 10, 11, 12, . . . . 98, 99

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
    println!("All two-digit numbers:");

    // Начинаем с 10 — первого положительного двузначного числа.
    let mut number: i32 = 10;

    // Включаем 99. После каждого вывода увеличиваем число на 1, иначе цикл не завершится.
    // print оставляет вывод на той же строке, println после цикла добавляет перевод строки.
    while number <= 99 {
        print!("{number} ");
        number += 1;
    }
    println!();
}
