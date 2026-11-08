/*
Cvičení 07 — Úloha 7.3
Název: Zlomky
Body: 1 bod; 1 bod; 2 body; 2 body; 1 bod; 1 bod (v pořadí podúloh)

Zadání:
Napište funkce pracující se zlomky, které mají čitatele i jmenovatele reprezentované dlouhým celým
číslem. Využijte funkce z příkladu 7.1. Výsledky funkcí d až f budou v základním tvaru.

a) Funkce VytvorZlomek načte zlomek z řetězce. [1 bod]

b) Funkce VypisZlomek vypíše zlomek na obrazovku. [1 bod]

c) Funkce ZakladniTvar převede zlomek na základní tvar. [2 body]

d) Funkce SoucetZlomku sečte dva zlomky. [2 body]

e) Funkce SoucinZlomku vynásobí dva zlomky [1 bod]

f) Funkce PodilZlomku vydělí zlomek zlomkem. [1 bod]


Ukázka:

Zlomek z1 = VytvorZlomek("-56890/1223366");

Zlomek z2 = VytvorZlomek("2565689841/2365");

VypisZlomek(z1); VypisZlomek(z2);

ZakladniTvar(ref z1); // -28445/611683

ZakladniTvar(ref z2); // 233244531/215

VypisZlomek(z1); VypisZlomek(z2);

Zlomek z3 = SoucetZlomku(z1, z2); // 142671708339998/131511845

Zlomek z4 = SoucinZlomku(z1, z2); // -1326928136859/26302369

Zlomek z5 = PodilZlomku(z2, z1); // -142671714455673/6115675

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

struct Fraction {
    Array<int> numerator, denominator;
    bool negative = false;
};

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
static Fraction fraction(const Array<int>& n, const Array<int>& d, bool negative);
static Fraction createFraction(std::string text);
static std::string fractionText(Fraction f);
static Fraction reduce(Fraction f);
static Fraction addFractions(Fraction a, Fraction b);
static Fraction multiplyFractions(Fraction a, Fraction b);
static Fraction divideFractions(Fraction a, Fraction b);

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

// Только запись данных; все действия со дробями — отдельные процедуры.
static Fraction fraction(const Array<int>& n, const Array<int>& d, bool negative) {
    check(!isZero(d));
    Fraction result = Fraction{};
    result.numerator = n;
    result.denominator = d;
    result.negative = negative && !isZero(n);
    return result;
}
static Fraction createFraction(std::string text) {
    check(static_cast<int>(text.size()) > 0);
    int slash = 0;
    while (slash < static_cast<int>(text.size()) && text[slash] != '/') slash++;
    int start = text[0] == '-' || text[0] == '+' ? 1 : 0;
    check(slash > start && slash + 1 < static_cast<int>(text.size()));
    return fraction(createNumber(text.substr(start, (slash) - (start))), createNumber(text.substr(slash + 1)), text[0] == '-');
}
static std::string fractionText(Fraction f) {
    return (f.negative ? "-" : "") + numberText(f.numerator) + "/" + numberText(f.denominator);
}
static Fraction reduce(Fraction f) {
    Array<int> a = f.numerator, b = f.denominator;
    while (!isZero(b)) {
        Array<int> rest = divide(a, b, 10).data[1];
        a = b;
        b = rest;
    }
    return fraction(divide(f.numerator, a, 10).data[0], divide(f.denominator, a, 10).data[0], f.negative);
}
static Fraction addFractions(Fraction a, Fraction b) {
    Array<int> x = multiply(a.numerator, b.denominator, 10), y = multiply(b.numerator, a.denominator, 10);
    Array<int> d = multiply(a.denominator, b.denominator, 10);
    if (a.negative == b.negative) return reduce(fraction(add(x, y, 10), d, a.negative));
    return reduce(fraction(subtract(x, y, 10), d, compare(x, y) >= 0 ? a.negative : b.negative));
}
static Fraction multiplyFractions(Fraction a, Fraction b) {
    return reduce(fraction(multiply(a.numerator, b.numerator, 10), multiply(a.denominator, b.denominator, 10), a.negative != b.negative));
}
static Fraction divideFractions(Fraction a, Fraction b) {
    return reduce(fraction(multiply(a.numerator, b.denominator, 10), multiply(a.denominator, b.numerator, 10), a.negative != b.negative));
}
int main() {
    Fraction a = createFraction("-56890/1223366"), b = createFraction("2565689841/2365");
    std::cout << ("Z1: ") << (fractionText(a)) << '\n';
    std::cout << ("Z2: ") << (fractionText(b)) << '\n';
    a = reduce(a);
    b = reduce(b);
    std::cout << ("Reduced Z1: ") << (fractionText(a)) << '\n';
    std::cout << ("Reduced Z2: ") << (fractionText(b)) << '\n';
    std::cout << ("Sum: ") << (fractionText(addFractions(a, b))) << '\n';
    std::cout << ("Product: ") << (fractionText(multiplyFractions(a, b))) << '\n';
    std::cout << ("Z2 / Z1: ") << (fractionText(divideFractions(b, a))) << '\n';
}
