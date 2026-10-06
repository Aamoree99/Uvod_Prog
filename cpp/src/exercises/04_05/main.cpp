/*
Cvičení 04 — Úloha 4.5
Název: Maximum z pole
Body: 1 bod; 1 bod; 2 body (v pořadí podúloh)

Zadání:
Napište funkci, která v zadaném poli nezáporných čísel najde a pomocí parametrů funkce vrátí:

a) hodnotu největšího prvku [1 bod]

b) hodnotu druhého největšího prvku [1 bod]

c) třetího největšího prvku [2 body]

Ukázka:

int[] pole = { 2, 5, 7, 1, 4, 6, 9, 8, 2 };

MaxPole(pole, ref max, ref max2, ref max3); // 9, 8, 7

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

// По условию числа неотрицательные, поэтому начальные максимумы равны нулю.
// Повторы учитываются; для короткого массива незаполненные места остаются нулевыми.
static void max_pole(const std::vector<int>& pole, int& max, int& max2, int& max3) {
    max=0; max2=0; max3=0;
    int i=0;
    while (i < static_cast<int>(pole.size())) {
        int cislo=pole[i];
        // Сдвигаем значения справа налево, чтобы не потерять прежние максимумы.
        if (cislo > max) { max3=max2; max2=max; max=cislo; }
        else if (cislo > max2) { max3=max2; max2=cislo; }
        else if (cislo > max3) { max3=cislo; }
        i += 1;
    }
}
int main() {
    std::vector<int> pole={2,5,7,1,4,6,9,8,2};
    int max=0,max2=0,max3=0;
    max_pole(pole,max,max2,max3);
    std::cout << "Pole: " << array_text(pole) << '\n';
    std::cout << "Nejvetsi:       " << max << '\n';
    std::cout << "Druhe nejvetsi: " << max2 << '\n';
    std::cout << "Treti nejvetsi: " << max3 << '\n';
}
