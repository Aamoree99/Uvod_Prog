/*
Cvičení 07 — Úloha 7.5
Název: Dlouhá celá čísla v obecné číselné soustavě
Body: 1/2 bodu; 1/2 bodu; 1/2 bodu; 1/2 bodu; 3 body (v pořadí podúloh)

Zadání:
Funkce z příkladu 7.1 upravte tak, aby pracovaly s čísly v zadané číselné soustavě:

a) Funkce Soucet sečte dvě čísla. [1/2 bodu]

b) Funkce Rozdil odečte menší číslo od většího. [1/2 bodu]

c) Funkce Soucin vynásobí dvě čísla. [1/2 bodu]

d) Funkce Podil vydělí číslo číslem. [1/2 bodu]

e) Napište funkci Prevod, která převede číslo používající způsob uložení cifer big endian z jedné
číselné soustavy do druhé číselné soustavy. Při praktických pokusech používejte číselné soustavy o
maximálním základu 10, snáze se v nich zobrazují výsledky. [3 body]


Ukázka:

int[] c1 = Vytvor("560111202232646");

int[] c2 = Prevod(c1, 10, 2);

// 1111111010110101100001101001111100110100101000110

int[] c3 = Prevod(c2, 2, 5); // 1041403330220232421041

int[] c4 = Soucet(c3, c3, 5); // 2133312210441020342132

int[] c5 = Rozdil(c4, c3, 5); // 1041403330220232421041

int[] c6 = Prevod(c5, 5, 7); // 225656461511451251

int[] c7 = Soucin(c6, c6, 7);

// 55402656051563425266136012062265231

int[] c8 = Podil(c7, c6, 7); // 225656461511451251

int[] c9 = Prevod(c8, 7, 10); // 560111202232646

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

constexpr int MAX_DIGITS = 128;

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
static Array<int> convert(const Array<int>& a, int fromBase, int toBase);

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

// Схема Горнера сразу над цифрами целевой системы.
static Array<int> convert(const Array<int>& a, int fromBase, int toBase) {
    validate(a, fromBase);
    check(toBase >= 2 && toBase <= 10);
    Array<int> result = makeArray<int>(MAX_DIGITS);
    for (int i = 0; i < MAX_DIGITS; i++) {
        int carry = a.data[i];
        for (int j = MAX_DIGITS - 1; j >= 0; j--) {
            int value = result.data[j] * fromBase + carry;
            result.data[j] = value % toBase;
            carry = value / toBase;
        }
        check(carry == 0);
    }
    return result;
}
int main() {
    Array<int> c1 = createNumber("560111202232646");
    Array<int> c2 = convert(c1, 10, 2);
    Array<int> c3 = convert(c2, 2, 5);
    Array<int> c4 = add(c3, c3, 5);
    Array<int> c5 = subtract(c4, c3, 5);
    Array<int> c6 = convert(c5, 5, 7);
    Array<int> c7 = multiply(c6, c6, 7);
    Array<Array<int>> division = divide(c7, c6, 7);
    Array<int> c8 = division.data[0], c9 = convert(c8, 7, 10);
    Array<Array<int>> numbers = arrayOf<Array<int>>({c1, c2, c3, c4, c5, c6, c7, c8, c9});
    Array<int> bases = arrayOf<int>({10, 2, 5, 5, 5, 7, 7, 7, 10});
    for (int i = 0; i < numbers.length; i++)
        std::cout << ("C") << ((i + 1)) << (" (base ") << (bases.data[i]) << ("): ") << (numberText(numbers.data[i])) << '\n';
    check(compare(c1, c9) == 0 && isZero(division.data[1]));
    std::cout << ("ALL CHECKS PASSED") << '\n';
}
