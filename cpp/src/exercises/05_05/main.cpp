/*
Cvičení 05 — Úloha 5.5
Název: Binární vyhledávání
Body: 2 body; +1 bod (v pořadí podúloh)

Zadání:
Napište funkci hledání zadané hodnoty v sestupně setříděném poli za použití algoritmu binárního
vyhledávání:

a) Je vrácen index libovolného prvku s hledanou hodnotou. [2 body]

b) Je vrácen index prvního prvku v poli, který má hledanou hodnotu. [+1 bod]


Ukázka

int[] p1 = { 12, 9, 6, 6, 6, 5, 3, 1, 1 };

int index = BinarniVyhledavani(p1, 6);

// (a) index = 2, 3, nebo 4 (b) index = 2

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

#include <initializer_list>
#include <iostream>
#include <memory>
#include <stdexcept>
#include <string>

// Обычный динамический массив и его длина. У записи нет методов;
// shared_ptr управляет временем жизни выделенного массива при возврате из функций.
template<class T> struct Array {
    std::shared_ptr<T[]> data;
    int length = 0;
};
template<class T> Array<T> makeArray(int length) {
    if (length < 0) throw std::invalid_argument("Negative array length");
    return {std::shared_ptr<T[]>(new T[length]{}), length};
}
template<class T> Array<T> arrayOf(std::initializer_list<T> values) {
    Array<T> result = makeArray<T>(static_cast<int>(values.size()));
    const T* source = values.begin();
    for (int i = 0; i < result.length; i++) result.data[i] = source[i];
    return result;
}

static int binarySearch(const Array<int>& a, int target, bool first);

// Полуинтервал [left, right) работает и с пустым массивом.
static int binarySearch(const Array<int>& a, int target, bool first) {
    int left = 0, right = a.length, result = -1;
    while (left < right) {
        int middle = left + (right - left) / 2;
        if (a.data[middle] == target) {
            if (!first) return middle;
            result = middle;
            right = middle;
        } else if (a.data[middle] > target) left = middle + 1;
        else right = middle;
    }
    return result;
}
int main() {
    Array<int> a = arrayOf<int>({12, 9, 6, 6, 6, 5, 3, 1, 1});
    std::cout << ("Any index: ") << (binarySearch(a, 6, false)) << '\n';
    std::cout << ("First index: ") << (binarySearch(a, 6, true)) << '\n';
    std::cout << ("Missing 10: ") << (binarySearch(a, 10, false)) << '\n';
}
