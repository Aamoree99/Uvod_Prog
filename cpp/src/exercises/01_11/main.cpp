/*
Cvičení 01 — Úloha 1.11
Název: Rychlost na dálnici
Body: 3 body

Zadání:
Nejnižší povolená rychlost na dálnici je 60 km/h, za její nedodržení je udělena pokuta 1000 Kč.
Nejvyšší povolená rychlost na dálnici je 130 km/h, za její překročení je udělena pokuta 2000 Kč.
V případě výrazného překročení nejvyšší povolené se pokuta zvyšuje o 500 Kč za každých započatých 5
km/h nad 150 km/h.

Ve funkci Main deklarujte jednu celočíselnou proměnnou (pojmenujte je libovolně) a tuto proměnnou
inicializujte hodnotou, které by mohla reprezentovat rychlost automobilu.  Napište program, který
určí výši pokuty odpovídající dané rychlosti. Cíl příkladu a jeho výsledek vypište vhodným způsobem
na obrazovku. [3 body]

Ukázka:

Hodnota proměnné je 146.

Rychlost 146 km/h je pokutována částkou 2000 Kč.

Hodnota proměnné je 162.

Rychlost 162 km/h je pokutována částkou 3500 Kč.

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

// Запускаем пример из упражнения и выводим результаты.
int main() {
    std::cout << std::boolalpha;
    int speed = 146;

    // Границы диапазонов важны: 60 и 130 входят в диапазон без штрафа, 150 — в следующий.
    if (speed < 60) {
        std::cout << "A speed of " << speed << " km/h incurs a fine of 1000 CZK." << '\n';
    } else if (speed <= 130) {
        std::cout << "A speed of " << speed << " km/h incurs no fine." << '\n';
    } else if (speed <= 150) {
        std::cout << "A speed of " << speed << " km/h incurs a fine of 2000 CZK." << '\n';
    } else {
        // Считаем количество начатых пятёрок сверх 150. Добавление 4 перед целочисленным
        // делением округляет вверх: превышение на 1–5 даёт одну пятёрку, на 6–10 — две.
        auto extra = (speed - 150 + 4) / 5;
        auto fine = 2000 + extra * 500;

        std::cout << "A speed of " << speed << " km/h incurs a fine of " << fine << " CZK." << '\n';
    }
    return 0;
}
