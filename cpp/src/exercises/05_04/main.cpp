/*
Cvičení 05 — Úloha 5.4
Název: Testování rychlostí algoritmů
Body: 2 body; 2 body (v pořadí podúloh)

Zadání:
Otestujte třídící algoritmy z příkladu 5.3 na polích různé velikosti (1000, 10 000 a 30 000 prvků),
naplněných náhodnými hodnotami z intervalu 0 až velikost pole. Všechny třídící algoritmy budou
pracovat nad kopií stejného pole. Výsledky přehledně prezentujte.

a) s použitím počtu operací [2 body]

b) s použitím měření času [2 body]


Ukázka – část (a):

Prvku 1000

Selection sort – porovnani: 500500, prirazeni: 3000

Insertion sort – porovnani: 259065, prirazeni: 260069

Bubble sort – porovnani: 974025, prirazeni: 774213

Count sort – porovnani: 2995, prirazeni: 3006

Prvku 10000

...

Prvku 30000

...


Ukázka – část (b):

Prvku 1000

...

Prvku 10000

...

Prvku 30000

Selection sort – doba behu 11194 ms

Insertion sort – doba behu 9300 ms

Bubble sort – doba behu 39415 ms

Count sort – doba behu 4 ms

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
static void selectionSort(const Array<int>& a);
static void insertionSort(const Array<int>& a);
static void bubbleSort(const Array<int>& a);
static void swap(const Array<int>& a, int j);
static void countingSort(const Array<int>& a);
static void printArray(const Array<int>& a);
static void sort(const Array<int>& a, int algorithm);
static bool isDescending(const Array<int>& a);
static void benchmark(const Array<int>& original, int algorithm, std::string name);

// Считаем только операции над значениями, а не управление циклами.
static std::int64_t assignments = 0, comparisons = 0;
static int assign(int value) { assignments++; return value; }
static bool less(int a, int b) { comparisons++; return a < b; }
static bool greater(int a, int b) { comparisons++; return a > b; }

static void selectionSort(const Array<int>& a) {
    for (int i = 0; i < a.length; i++) {
        int maximum = i;
        for (int j = i + 1; j < a.length; j++)
            if (greater(a.data[j], a.data[maximum])) maximum = j;
        int temporary = assign(a.data[i]);
        a.data[i] = assign(a.data[maximum]);
        a.data[maximum] = assign(temporary);
    }
}
static void insertionSort(const Array<int>& a) {
    for (int i = 1; i < a.length; i++) {
        int key = assign(a.data[i]), j = i;
        while (j > 0 && less(a.data[j - 1], key)) {
            a.data[j] = assign(a.data[j - 1]);
            j--;
        }
        a.data[j] = assign(key);
    }
}
static void bubbleSort(const Array<int>& a) {
    for (int i = 0; i < a.length; i++)
        for (int j = 0; j + 1 < a.length - i; j++)
            if (less(a.data[j], a.data[j + 1])) swap(a, j);
}
static void swap(const Array<int>& a, int j) {
    int temporary = assign(a.data[j]);
    a.data[j] = assign(a.data[j + 1]);
    a.data[j + 1] = assign(temporary);
}
// Счётчики и диапазон соответствуют исходной версии Rust.
static void countingSort(const Array<int>& a) {
    if (a.length == 0) return;
    int maximum = 0;
    for (int i = 0; i < a.length; i++) {
        if (a.data[i] < 0 || a.data[i] > 30000) throw std::invalid_argument("Counting sort value outside range");
        if (greater(a.data[i], maximum)) maximum = assign(a.data[i]);
    }
    Array<int> counts = makeArray<int>(30000 + 1);
    for (int i = 0; i < a.length; i++) counts.data[a.data[i]] = assign(counts.data[a.data[i]] + 1);
    int position = 0;
    for (int number = maximum; number >= 0; number--)
        while (greater(counts.data[number], 0)) {
            a.data[position++] = assign(number);
            counts.data[number] = assign(counts.data[number] - 1);
        }
}
static void printArray(const Array<int>& a) {
    for (int i = 0; i < a.length; i++) std::cout << (a.data[i]) << (" ");
    std::cout << '\n';
}
static void sort(const Array<int>& a, int algorithm) {
    if (algorithm == 0) selectionSort(a);
    else if (algorithm == 1) insertionSort(a);
    else if (algorithm == 2) bubbleSort(a);
    else countingSort(a);
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
    Array<std::string> names = arrayOf<std::string>({"Selection", "Insertion", "Bubble", "Counting"});
        for (int i = 0; i < sizes.length; i++) {
        Array<int> original = makeArray<int>(sizes.data[i]);
        for (int j = 0; j < original.length; j++) original.data[j] = randomInt(original.length + 1);
        std::cout << ("ARRAY SIZE: ") << (sizes.data[i]) << '\n';
        for (int k = 0; k < names.length; k++) benchmark(original, k, names.data[k]);
    }
}
