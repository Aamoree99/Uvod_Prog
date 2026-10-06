/*
Cvičení 04 — Úloha 4.1
Název: Sestupné setřídění
Body: 1 bod

Zadání:
Napište funkci, která své dva parametry setřídí sestupně. [1 bod]


Ukázka:

int a = 5, b = 7;

SetridSestupne(ref a, ref b); // a = 7; b = 5;

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

// Сортируем два числа по убыванию.
//
// После выполнения:
// a >= b
fn setrid_sestupne(a: &mut i32, b: &mut i32) {
    // Если первое число меньше второго,
    // значит они стоят в неправильном порядке.
    if *a < *b {
        // Сохраняем первое значение,
        // чтобы не потерять его.
        let pomocna = *a;

        // Во первое записываем второе.
        *a = *b;

        // Во второе записываем старое первое.
        *b = pomocna;
    }
}

fn main() {
    // mut нужен, потому что функция будет менять эти переменные.
    let mut a = 5;
    let mut b = 7;

    println!("Pred: a = {}, b = {}", a, b);

    // &mut означает:
    // передаём функции возможность изменить a и b.
    setrid_sestupne(&mut a, &mut b);

    println!("Po:   a = {}, b = {}", a, b);
}
