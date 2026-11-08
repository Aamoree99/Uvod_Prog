/*
Cvičení 05 — Úloha 5.3
Název: Sestupné třídění
Body: 3+1 body; 3+1 body; 2+1 body; 3+2 body (v pořadí podúloh)

Zadání:
Napište třídící funkce pro sestupné třídění. Body navíc lze získat za použití pomocných funkcí
z příkladu 5.2 pro porovnávání a přiřazování hodnot prvků. Pomocné funkce nepoužívejte pro práci
s řídící proměnnou cyklu. Výjimkou je Count sort, u kterého pomocné funkce se používají pro řídící
proměnnou cyklu v situacích, ve kterých je řídící proměnná cyklu brána jako prvek pole.

a) Selection sort [3+1 body]

b) Insertion sort [3+1  body]

c) Bubble sort [2+1  body]

d) Count sort [3+2 body]


Ukázka:

int[] pole = { 5, 7, 1, 5, 6, 8, 9, 0, 1, 2};

SelectionSort(pole); // 9, 8, 7, 6, 5, 5, 2, 1, 1, 0

Console.WriteLine("Prirazeni: {0}, Porovnani: {1}", prirazeni, porovnani); // 30,  55

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
template<class T> Array<T> cloneArray(const Array<T>& source) {
    Array<T> result = makeArray<T>(source.length);
    for (int i = 0; i < source.length; i++) result.data[i] = source.data[i];
    return result;
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
        if (a.data[i] < 0 || a.data[i] > 1000) throw std::invalid_argument("Counting sort value outside range");
        if (greater(a.data[i], maximum)) maximum = assign(a.data[i]);
    }
    Array<int> counts = makeArray<int>(1000 + 1);
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

int main() {
    Array<int> original = arrayOf<int>({5, 7, 1, 5, 6, 8, 9, 0, 1, 2});
    Array<std::string> names = arrayOf<std::string>({"Selection", "Insertion", "Bubble", "Counting"});
    for (int k = 0; k < 4; k++) {
        Array<int> a = cloneArray(original);
        assignments = comparisons = 0;
        sort(a, k);
        std::cout << (names.data[k]) << '\n';
        printArray(a);
        std::cout << ("Assignments: ") << (assignments) << (", Comparisons: ") << (comparisons) << '\n';
    }
}
