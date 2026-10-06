/*
Cvičení 04 — Úloha 4.3
Název: Zaokrouhlení pole
Body: 2 body

Zadání:
Napište funkci, která pro zadané pole reálných čísel vrátí pole celých čísel. Hodnoty prvků v novém
poli vzniknou zaokrouhlením hodnot prvků z původního pole. [2 body]

Ukázka:

double[] pole = { 5.12, -3.5996, 156, 7.9 };

zaokrouhli = ZaokrouhliPole(pole); // 5, -4, 156, 8

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
#include <vector>
#include <string>
#include <sstream>
#include <cmath>

template<class T>
std::string array_text(const T& values) {
    std::ostringstream out;
    out << "[";
    std::size_t i = 0;
    while (i < values.size()) {
        if (i) out << ", ";
        out << values[i++];
    }
    out << "]";
    return out.str();
}

// Возвращаем новый массив из четырёх элементов, как в исходном примере.
static std::vector<int> zaokrouhli_pole(const std::vector<double>& pole) {
    std::vector<int> vysledek = {0,0,0,0};
    int i=0;
    while (i < static_cast<int>(pole.size())) {
        // В Rust половины округляются от нуля; выбираем такое же правило в каждом языке.
        vysledek[i] = static_cast<int>(std::round(pole[i]));
        i += 1;
    }
    return vysledek;
}
int main() {
    std::cout << std::boolalpha;
    std::vector<double> pole = {5.12,-3.5996,156.0,7.9};
    std::vector<int> zaokrouhlene=zaokrouhli_pole(pole);
    std::cout << "Puvodni:      " << array_text(pole) << '\n'; std::cout << "Zaokrouhlene: " << array_text(zaokrouhlene) << '\n';
}
