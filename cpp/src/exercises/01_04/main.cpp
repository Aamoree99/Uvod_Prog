/*
Cvičení 01 — Úloha 1.4
Název: Největší ze tří čísel
Body: 1 bod

Zadání:
Ve funkci Main deklarujte tři celočíselné proměnné (pojmenujte je libovolně) a tyto proměnné
inicializujte hodnotami (ideálně různými). Napište program, který určí nejvyšší z hodnot výše
uvedených tří proměnných. Cíl příkladu a jeho výsledek vypište vhodným způsobem na obrazovku. [1
bod]

Ukázka:

Hodnota první proměnné je 14, hodnota druhé proměnné je 6, hodnota třetí proměnné je 23.

Nejvyšší z hodnot 14, 6 a 23 je 23.

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
    int a = 14;
    int b = 6;
    int c = 23;

    // За начальный максимум берём первое число, а не 0: так работают и отрицательные числа.
    // Переменная хранит текущий максимум и изменяется при сравнении.
    int max = a;

    // Сравниваем каждый следующий кандидат с текущим максимумом и сохраняем больший.
    if (b > max) {
        max = b;
    }

    // Здесь нужен отдельный if: после проверки b число c всё ещё может оказаться больше.
    if (c > max) {
        max = c;
    }

    std::cout << "Nejvyšší z hodnot " << a << ", " << b << " a " << c << " je " << max << "." << '\n';
    return 0;
}
