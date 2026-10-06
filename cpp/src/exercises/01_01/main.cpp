/*
Cvičení 01 — Úloha 1.1
Název: Specifický aritmetický výraz
Body: 1 bod

Zadání:
Ve funkci Main deklarujte tři celočíselné proměnné (pojmenujte je libovolně) a tyto proměnné
inicializujte hodnotami (ideálně různými). Napište program, který spočte následující aritmetický
výraz: Hodnoty posledních dvou proměnných vynásobte a od získaného výsledku odečtěte hodnotu první
proměnné. Aritmetický výraz a jeho výsledek vypište vhodným způsobem na obrazovku. [1 bod].

Ukázka:

Hodnota první proměnné je 12, hodnota druhé proměnné je 5, hodnota třetí proměnné je 9.

Výsledek výrazu 5*9-12 je 33

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

// Пример исходных данных из условия. int — целое число со знаком.
int main() {
    std::cout << std::boolalpha;
    int a = 12;
    int b = 5;
    int c = 9;

    // Сначала умножаем второе число на третье, затем вычитаем первое: b * c - a.
    auto result = b * c - a;

    std::cout << "Výsledek výrazu " << b << "*" << c << "-" << a << " je " << result << '\n';
    return 0;
}
