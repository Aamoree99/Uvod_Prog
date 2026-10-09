/*
Cvičení 02 — Úloha 2.6
Název: Prvočísla
Body: 3 body

Zadání:
Vypište všechna prvočísla menší než 1000 [3 body]


Ukázka:

Vsechna prvocisla mensi nez 1000: 2, 3, 5, . . . . 991, 997

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

// По условию достаточно перебрать числа до 1000 и проверить делители каждого.
// Здесь автор выбрал более эффективный алгоритм — решето Эратосфена —
// и увеличил предел до 100000. Это расширенный вариант для изучения.
// Он использует Vec (динамический массив) и отличается от ограничений исходного задания.
fn main() {
    println!("Note:");
    println!("This exercise can be solved by looping through the numbers");
    println!("and testing all possible divisors for each number.");
    println!("This implementation uses the sieve of Eratosthenes");
    println!("and increases the limit from 1000 to 100000.");
    println!();

    let limit: usize = 100000;

    // Индекс — само число; true означает, что число пока считается простым.
    // 0 и 1 исключаем отдельно: они не являются простыми.
    let mut is_prime = vec![true; limit];

    is_prime[0] = false;
    is_prime[1] = false;

    let mut number = 2;

    // Достаточно обработать делители до квадратного корня из предела: у каждого
    // составного числа есть делитель не больше его корня.
    while number * number < limit {
        if is_prime[number] {
            // Начинаем вычёркивание с number²: меньшие кратные уже вычеркнули предыдущие простые.
            // Шаг number перебирает все следующие кратные.
            let mut multiple = number * number;

            while multiple < limit {
                is_prime[multiple] = false;
                multiple += number;
            }
        }

        number += 1;
    }

    println!("Prime numbers less than {limit}:");
    println!();

    let mut number = 2;
    // При выводе считаем простые числа и переносим строку после каждых десяти.
    let mut count = 0;

    while number < limit {
        if is_prime[number] {
            print!("{number:>6}");
            count += 1;

            if count % 10 == 0 {
                println!();
            }
        }

        number += 1;
    }

    if count % 10 != 0 {
        println!();
    }

    println!();
    println!("Total prime count: {count}");
}
