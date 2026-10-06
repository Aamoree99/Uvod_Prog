/*
Cvičení 01 — Úloha 1.7
Název: Lichá vs sudá čísla
Body: 4 body

Zadání:
Ve funkci Main deklarujte tři celočíselné proměnné (pojmenujte je libovolně) a tyto proměnné
inicializujte hodnotami. Napište program, který určí, zda součet lichých hodnot z těchto tří čísel
je vyšší nebo nižší než součet sudých hodnot z těchto tří čísel, případně zda si jsou součty rovny.
Cíl příkladu a jeho výsledek vypište vhodným způsobem na obrazovku. [4 body]

Ukázka:

Hodnota první proměnné je 9, hodnota druhé proměnné je 2 hodnota třetí proměnné je 6.

Zadaná čísla 9, 2 a 6 mají součet lichých hodnot 9 vyšší než součet sudých hodnot 8.

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

#include <iostream>

int main() {
    std::cout << std::boolalpha;
    int a = 9;
    int b = 2;
    int c = 6;

    // Суммы начинаются с 0: если подходящих чисел нет, их сумма остаётся нулевой.
    int odd = 0;
    int even = 0;

    // Остаток от деления на 2 отличает нечётные числа от чётных.
    // Проверка != 0 работает и для отрицательных нечётных чисел. Так обрабатываем a, b и c.
    if (a % 2 != 0) {
        odd += a;
    } else {
        even += a;
    }

    if (b % 2 != 0) {
        odd += b;
    } else {
        even += b;
    }

    if (c % 2 != 0) {
        odd += c;
    } else {
        even += c;
    }

    // После распределения чисел сравниваем суммы; учитываем больше, меньше и равенство.
    if (odd > even) {
        std::cout << "Zadaná čísla " << a << ", " << b << " a " << c << " mají součet lichých hodnot " << odd << " vyšší než součet sudých hodnot " << even << "." << '\n';
    } else if (odd < even) {
        std::cout << "Zadaná čísla " << a << ", " << b << " a " << c << " mají součet lichých hodnot " << odd << " nižší než součet sudých hodnot " << even << "." << '\n';
    } else {
        std::cout << "Zadaná čísla " << a << ", " << b << " a " << c << " mají součet lichých hodnot " << odd << " stejný jako součet sudých hodnot " << even << "." << '\n';
    }
    return 0;
}
