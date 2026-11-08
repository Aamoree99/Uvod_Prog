/*
Cvičení 05 — Úloha 5.6
Název: Vylepšený Bubble Sort
Body: 2 body; 1 bod (v pořadí podúloh)

Zadání:
Algoritmus Bubble Sort třídící sestupně zadané pole lze implementovat ve dvou verzích. V základní
verzi algoritmu se ve vnořeném cyklu prochází pole až do posledního prvku.  Ve vylepšené verzi
algoritmu se ve vnořeném cyklu prochází pole jen do místa poslední výměny, která proběhla
v předchozí iteraci vnějšího cyklu.

a) Napište vylepšenou verzi Bubble sortu. Pokud jste vylepšenou verzi napsali již v příklad 5.3c,
napište nyní verzi základní. [2 body]

b) Změřte (dle příkladu 5.4) počty operací a dobu běhu vylepšené a základní verze. [1 bod]

Ukázka:

Prvku 1000

Bubble sort – porovnani: 944055, prirazeni: 763212

Bubble sort2 – porovnani: 496415, prirazeni: 763212

Bubble sort – doba behu 46 ms

Bubble sort2 – doba behu 30 ms

Prvku 10000

...

Prvku 30000

...

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

#include <chrono>
#include <cstdint>
#include <initializer_list>
#include <iostream>
#include <memory>
#include <random>
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
template<class T> Array<T> cloneArray(const Array<T>& source) {
    Array<T> result = makeArray<T>(source.length);
    for (int i = 0; i < source.length; i++) result.data[i] = source.data[i];
    return result;
}
std::mt19937 randomEngine(std::random_device{}());
int randomInt(int bound) {
    return std::uniform_int_distribution<int>(0, bound - 1)(randomEngine);
}
std::int64_t nanoTime() {
    return std::chrono::duration_cast<std::chrono::nanoseconds>(
        std::chrono::steady_clock::now().time_since_epoch()).count();
}

static int assign(int value);
static bool less(int a, int b);
static bool greater(int a, int b);
static void swap(const Array<int>& a, int j);
static void basicBubbleSort(const Array<int>& a);
static void improvedBubbleSort(const Array<int>& a);
static void sort(const Array<int>& a, int algorithm);
static bool isDescending(const Array<int>& a);
static void benchmark(const Array<int>& original, int algorithm, std::string name);

// Считаем только операции над значениями, а не управление циклами.
static std::int64_t assignments = 0, comparisons = 0;
static int assign(int value) { assignments++; return value; }
static bool less(int a, int b) { comparisons++; return a < b; }
static bool greater(int a, int b) { comparisons++; return a > b; }
static void swap(const Array<int>& a, int j) {
    int temporary = assign(a.data[j]);
    a.data[j] = assign(a.data[j + 1]);
    a.data[j + 1] = assign(temporary);
}

static void basicBubbleSort(const Array<int>& a) {
    for (int i = 0; i + 1 < a.length; i++)
        for (int j = 0; j + 1 < a.length; j++)
            if (less(a.data[j], a.data[j + 1])) swap(a, j);
}
static void improvedBubbleSort(const Array<int>& a) {
    int limit = a.length;
    while (limit > 1) {
        int lastSwap = 0;
        for (int j = 0; j + 1 < limit; j++)
            if (less(a.data[j], a.data[j + 1])) {
                swap(a, j);
                lastSwap = j + 1;
            }
        limit = lastSwap;
    }
}
static void sort(const Array<int>& a, int algorithm) {
    if (algorithm == 0) basicBubbleSort(a);
    else improvedBubbleSort(a);
}

static bool isDescending(const Array<int>& a) {
    for (int i = 1; i < a.length; i++)
        if (a.data[i] > a.data[i - 1]) return false;
    return true;
}
static void benchmark(const Array<int>& original, int algorithm, std::string name) {
    Array<int> a = cloneArray(original);
    assignments = comparisons = 0;
    std::int64_t start = nanoTime();
    sort(a, algorithm);
    double milliseconds = (nanoTime() - start) / 1000000.0;
    if (!isDescending(a)) throw std::runtime_error("Sorting failed");
    std::cout << (name) << (" | Comparisons: ") << (comparisons) << (" | Assignments: ") << (assignments) << (" | ms: ") << (milliseconds) << (" | OK") << '\n';
}
int main() {
    Array<int> sizes = arrayOf<int>({1000, 10000, 30000});
    Array<std::string> names = arrayOf<std::string>({"Basic Bubble", "Improved Bubble"});
        for (int i = 0; i < sizes.length; i++) {
        Array<int> original = makeArray<int>(sizes.data[i]);
        for (int j = 0; j < original.length; j++) original.data[j] = randomInt(original.length + 1);
        std::cout << ("ARRAY SIZE: ") << (sizes.data[i]) << '\n';
        for (int k = 0; k < names.length; k++) benchmark(original, k, names.data[k]);
    }
}
