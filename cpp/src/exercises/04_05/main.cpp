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
// Возвращаем текстовое представление массива с разделителями.
std::string array_text(const T& elements) {
    std::ostringstream out;
    out << "[";
    std::size_t i = 0;
    while (i < elements.size()) {
        if (i) out << ", ";
        out << elements[i++];
    }
    out << "]";
    return out.str();
}

// По условию числа неотрицательные, поэтому начальные максимумы равны нулю.
// Повторы учитываются; для короткого массива незаполненные места остаются нулевыми.
static void array_maxima(const std::vector<int>& array, int& largest, int& second_largest, int& third_largest) {
    largest=0; second_largest=0; third_largest=0;
    int i=0;
    while (i < static_cast<int>(array.size())) {
        int number=array[i];
        // Сдвигаем значения справа налево, чтобы не потерять прежние максимумы.
        if (number > largest) { third_largest=second_largest; second_largest=largest; largest=number; }
        else if (number > second_largest) { third_largest=second_largest; second_largest=number; }
        else if (number > third_largest) { third_largest=number; }
        i += 1;
    }
}
// Запускаем пример из упражнения и выводим результаты.
int main() {
    std::vector<int> array={2,5,7,1,4,6,9,8,2};
    int largest=0,second_largest=0,third_largest=0;
    array_maxima(array,largest,second_largest,third_largest);
    std::cout << "Array: " << array_text(array) << '\n';
    std::cout << "Largest:       " << largest << '\n';
    std::cout << "Second largest: " << second_largest << '\n';
    std::cout << "Third largest: " << third_largest << '\n';
}
