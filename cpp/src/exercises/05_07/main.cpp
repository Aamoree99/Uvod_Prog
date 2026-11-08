/*
Cvičení 05 — Úloha 5.7
Název: Vylepšený Count Sort
Body: 3 body

Zadání:
Rozšiřte funkci z příkladu  5.3d pro sestupné setřídění pole pomocí třídícího algoritmu Count Sort.
Hodnoty prvků pole jsou z omezeného intervalu, který může obsahovat i záporná čísla [3 body]


Ukázka:

int[] p1 = { -5, 7, 15, 22, 4, 0, 4, 9, -11 };

CountSort(p1); // 22, 15, 9, 7, 4, 4, 0, -5, -11

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

#include <cstdint>
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

static int assign(int value);
static bool less(int a, int b);
static bool greater(int a, int b);
static void countingSort(const Array<int>& a);
static void printArray(const Array<int>& a);

// Считаем только операции над значениями, а не управление циклами.
static std::int64_t assignments = 0, comparisons = 0;
static int assign(int value) { assignments++; return value; }
static bool less(int a, int b) { comparisons++; return a < b; }
static bool greater(int a, int b) { comparisons++; return a > b; }
// Смещение позволяет учитывать отрицательные значения без изменения порядка.
static void countingSort(const Array<int>& a) {
    if (a.length == 0) return;
    int minimum = a.data[0], maximum = a.data[0];
    for (int i = 1; i < a.length; i++) {
        if (less(a.data[i], minimum)) minimum = assign(a.data[i]);
        if (greater(a.data[i], maximum)) maximum = assign(a.data[i]);
    }
    std::int64_t range = (std::int64_t)maximum - minimum + 1;
    if (range > 10000000) throw std::invalid_argument("Counting sort range is too large");
    Array<int> counts = makeArray<int>((int)range);
    for (int i = 0; i < a.length; i++) {
        int index = a.data[i] - minimum;
        counts.data[index] = assign(counts.data[index] + 1);
    }
    int position = 0;
    for (int index = counts.length - 1; index >= 0; index--)
        while (greater(counts.data[index], 0)) {
            a.data[position++] = assign(index + minimum);
            counts.data[index] = assign(counts.data[index] - 1);
        }
}
static void printArray(const Array<int>& a) {
    for (int i = 0; i < a.length; i++) std::cout << (a.data[i]) << (" ");
    std::cout << '\n';
}

int main() {
    Array<int> a = arrayOf<int>({-5, 7, 15, 22, 4, 0, 4, 9, -11});
    countingSort(a);
    printArray(a);
    std::cout << ("Assignments: ") << (assignments) << (", Comparisons: ") << (comparisons) << '\n';
}
