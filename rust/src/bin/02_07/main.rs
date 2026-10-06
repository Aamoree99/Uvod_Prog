/*
Cvičení 02 — Úloha 2.7
Název: Dokonalá čísla
Body: 3 body

Zadání:
Vypište všechna dokonalá čísla menší než 10 000. Číslo je dokonalé, pokud je rovno součtu svých
dělitelů, kromě sebe sama. Např. 6 = 1+2+3; 28 = 1+2+4+7+14 [3 body]


Ukázka:

Vsechna dokonala cisla mensi nez 10000: 6, 28, . . . 8128

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

fn main() {
    println!("Vsechna dokonala cisla mensi nez 10000:");

    // Совершенное число равно сумме своих положительных делителей, кроме самого себя.
    let mut number: i32 = 2;

    while number < 10000 {
        // Для каждого нового числа обнуляем сумму и начинаем проверку делителей с 1.
        let mut divisor: i32 = 1;
        let mut sum: i32 = 0;

        // Не включаем само number, иначе сумма делителей всегда была бы слишком большой.
        while divisor < number {
            if number % divisor == 0 {
                sum += divisor;
            }

            divisor += 1;
        }

        // После проверки всех делителей сравниваем их сумму с исходным числом.
        if sum == number {
            print!("{number} ");
        }

        number += 1;
    }

    println!();
}
