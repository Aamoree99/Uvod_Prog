/*
Cvičení 07 — Úloha 7.1
Název: Dlouhá celá čísla
Body: 1 bod; 1 bod; 1 bod; 1 bod; 1 bod; 2 body; 3 body (v pořadí podúloh)

Zadání:
Napište funkce pracující s dlouhými celými čísly, které používají způsob uložení cifer v poli big
endian (na přednášce byl použit způsob little endian).

a) Funkci Vytvor, která převede textový řetězec na číslo. [1 bod]

b) Funkci Vypis, která vypíše číslo na obrazovku bez zbytečných nul. [1 bod]

c) Funkce Compare porovná dvě čísla. Je-li první číslo menší než druhé číslo, je vrácena hodnota
-1. Je-li první číslo větší než druhé číslo, je vrácena hodnota 1. Jsou-li si obě čísla rovna, je
vrácena hodnota 0. [1 bod]

d) Funkce Soucet sečte dvě čísla. [1 bod]

e) Funkce Rozdil odečte menší číslo od většího. [1 bod]

f) Funkce Soucin vynásobí dvě čísla. [2 body]

g) Funkce Podil vydělí číslo číslem. [3 body]


Ukázka:

int[] c1 = Vytvor("560111202232646");

int[] c2 = Vytvor("560100600690986");

Vypis(c1); // 560111202232646

int i = Compare(c1, c2); // 1

int[] c3 = Soucet(c1, c2);  // 1120211802923632

int[] c4 = Rozdil(c1, c2);  // 10601541660

int[] c5 = Soucin(c1, c4); // 5938042244702081581032360

int[] c6 = Podil(c1, c4); // 52832

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
template<class T> Array<T> cloneArray(const Array<T>& source) {
    Array<T> result = makeArray<T>(source.length);
    for (int i = 0; i < source.length; i++) result.data[i] = source.data[i];
    return result;
}

constexpr int MAX_DIGITS = 100;

static void check(bool valid);
static Array<int> createNumber(std::string text);
static std::string numberText(const Array<int>& a);
static bool isZero(const Array<int>& a);
static void validate(const Array<int>& a, int base);
static int compare(const Array<int>& a, const Array<int>& b);
static Array<int> add(const Array<int>& a, const Array<int>& b, int base);
static void subtractInPlace(const Array<int>& a, const Array<int>& b, int base);
static Array<int> subtract(const Array<int>& a, const Array<int>& b, int base);
static Array<int> multiply(const Array<int>& a, const Array<int>& b, int base);
static Array<Array<int>> divide(const Array<int>& a, const Array<int>& b, int base);

static void check(bool valid) {
    if (!valid) throw std::invalid_argument("Invalid number, base or arithmetic overflow");
}
// Цифры расположены от старших к младшим. Встроенная длинная арифметика не используется.
static Array<int> createNumber(std::string text) {
    check(static_cast<int>(text.size()) > 0 && static_cast<int>(text.size()) <= MAX_DIGITS);
    Array<int> result = makeArray<int>(MAX_DIGITS);
    for (int i = 0; i < static_cast<int>(text.size()); i++) {
        char c = text[i];
        check(c >= '0' && c <= '9');
        result.data[MAX_DIGITS - static_cast<int>(text.size()) + i] = c - '0';
    }
    return result;
}
static std::string numberText(const Array<int>& a) {
    int i = 0;
    while (i + 1 < a.length && a.data[i] == 0) i++;
    std::string result = "";
    while (i < a.length) result += static_cast<char>('0' + a.data[i++]);
    return result;
}
static bool isZero(const Array<int>& a) {
    for (int i = 0; i < a.length; i++)
        if (a.data[i] != 0) return false;
    return true;
}
static void validate(const Array<int>& a, int base) {
    check(base >= 2 && base <= 10 && a.length == MAX_DIGITS);
    for (int i = 0; i < a.length; i++) check(a.data[i] >= 0 && a.data[i] < base);
}
static int compare(const Array<int>& a, const Array<int>& b) {
    check(a.length == b.length);
    for (int i = 0; i < a.length; i++)
        if (a.data[i] != b.data[i]) return a.data[i] < b.data[i] ? -1 : 1;
    return 0;
}
static Array<int> add(const Array<int>& a, const Array<int>& b, int base) {
    validate(a, base);
    validate(b, base);
    Array<int> result = makeArray<int>(MAX_DIGITS);
    int carry = 0;
    for (int i = MAX_DIGITS - 1; i >= 0; i--) {
        int sum = a.data[i] + b.data[i] + carry;
        result.data[i] = sum % base;
        carry = sum / base;
    }
    check(carry == 0);
    return result;
}
static void subtractInPlace(const Array<int>& a, const Array<int>& b, int base) {
    int borrow = 0;
    for (int i = a.length - 1; i >= 0; i--) {
        int difference = a.data[i] - b.data[i] - borrow;
        borrow = difference < 0 ? 1 : 0;
        if (difference < 0) difference += base;
        a.data[i] = difference;
    }
}
static Array<int> subtract(const Array<int>& a, const Array<int>& b, int base) {
    validate(a, base);
    validate(b, base);
    bool aLarger = compare(a, b) >= 0;
    Array<int> result = cloneArray(aLarger ? a : b);
    subtractInPlace(result, aLarger ? b : a, base);
    return result;
}
static Array<int> multiply(const Array<int>& a, const Array<int>& b, int base) {
    validate(a, base);
    validate(b, base);
    Array<int> work = makeArray<int>(MAX_DIGITS * 2);
    for (int i = MAX_DIGITS - 1; i >= 0; i--)
        for (int j = MAX_DIGITS - 1; j >= 0; j--) work.data[i + j + 1] += a.data[i] * b.data[j];
    for (int i = work.length - 1; i > 0; i--) {
        work.data[i - 1] += work.data[i] / base;
        work.data[i] %= base;
    }
    Array<int> result = makeArray<int>(MAX_DIGITS);
    for (int i = 0; i < MAX_DIGITS; i++) {
        check(work.data[i] == 0);
        result.data[i] = work.data[MAX_DIGITS + i];
    }
    return result;
}
static Array<Array<int>> divide(const Array<int>& a, const Array<int>& b, int base) {
    validate(a, base);
    validate(b, base);
    check(!isZero(b));
    Array<int> quotient = makeArray<int>(MAX_DIGITS), remainder = makeArray<int>(MAX_DIGITS + 1), divisor = makeArray<int>(MAX_DIGITS + 1);
    for (int i = 0; i < MAX_DIGITS; i++) divisor.data[i + 1] = b.data[i];
    for (int i = 0; i < MAX_DIGITS; i++) {
        for (int j = 0; j < MAX_DIGITS; j++) remainder.data[j] = remainder.data[j + 1];
        remainder.data[MAX_DIGITS] = a.data[i];
        while (compare(remainder, divisor) >= 0) {
            subtractInPlace(remainder, divisor, base);
            quotient.data[i]++;
        }
    }
    Array<int> rest = makeArray<int>(MAX_DIGITS);
    for (int i = 0; i < MAX_DIGITS; i++) rest.data[i] = remainder.data[i + 1];
    return arrayOf<Array<int>>({quotient, rest});
}

int main() {
    Array<int> a = createNumber("560111202232646"), b = createNumber("560100600690986");
    Array<int> difference = subtract(a, b, 10);
    std::cout << ("First: ") << (numberText(a)) << '\n';
    std::cout << ("Second: ") << (numberText(b)) << '\n';
    std::cout << ("Comparison: ") << (compare(a, b)) << '\n';
    std::cout << ("Sum: ") << (numberText(add(a, b, 10))) << '\n';
    std::cout << ("Difference: ") << (numberText(difference)) << '\n';
    std::cout << ("Product: ") << (numberText(multiply(a, difference, 10))) << '\n';
    std::cout << ("Quotient: ") << (numberText(divide(a, difference, 10).data[0])) << '\n';
}
