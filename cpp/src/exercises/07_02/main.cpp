/*
Cvičení 07 — Úloha 7.2
Název: Dlouhá reálná čísla
Body: 4 body; 1 bod; 1 bod; 5 bodů; 4 body; 3 body (v pořadí podúloh)

Zadání:
Napište funkce pracující s dlouhými reálnými čísly v plovoucí řádové čárce, které používají způsob
uložení cifer v poli little endian (tento způsob byl použit na přednášce).

a) Funkci Vytvor, která převede textový řetězec na číslo. Textový řetězec může (ale nemusí)
obsahovat znak pro exponent. Př.: 7896e4 = 78960000; 5636.7e-2 =56.367; [4 body]

b) Funkci Vypis, která vypíše číslo na obrazovku bez zbytečných nul. [1 bod]

c) Funkce Compare porovná dvě čísla. Je-li první číslo menší než druhé číslo, je vrácena hodnota
-1. Je-li první číslo větší než druhé číslo, je vrácena hodnota 1. Jsou-li si obě čísla rovna, je
vrácena hodnota 0. [1 bod]

d) Funkce Soucet sečte dvě čísla. V závislosti na znaménku sčítaných čísel bude docházet ke sčítání
(př. 2 + 3) nebo odečítání (př. 2 + -3) absolutních hodnot čísel. Na rozdíl od implementace celých
čísel vyřešte i možná přetečení a podtečení výsledku. [5 bodů]

e) Funkce Soucin vynásobí dvě čísla. [4 body]

f) Funkce Podil vydělí číslo číslem. [3 body]


Ukázka:

Real r1 = Vytvor("-256.2e-1"); // - 2.562e1

Real r2 = Vytvor("-9534.07256e15"); // -9.53407256e18

Real r3 = Vytvor("9534.07256e15"); // 9.53407256e18

Vypis(r1); Vypis(r2); Vypis(r3);

int i = Compare(r1,r2); // -1

Real r4 = Soucet(r2, r2);  // -1.906814512e19

Real r5 = Soucet(r1, r3);  // 9.53407255999999997438e18

Real r6 = Soucet(r2, r3);  // 0

Real r7 = Soucin(r1, r2);  // 2.442629389872e20

Real r8 = Podil(r2, r1);  // 3.7213397970335675253...e17

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

constexpr int PRECISION = 40;

struct Real {
    Array<int> digits = makeArray<int>(PRECISION);
    bool negative = false;
    int exponent = 0;
};

static void check(bool valid);
static int exponent(std::int64_t value);
static int highest(Real a);
static Real normalize(Real a);
static Real createReal(std::string text);
static std::string realText(Real a);
static int digitAt(Real a, std::int64_t power);
static int compareAbsolute(Real a, Real b);
static int compare(Real a, Real b);
static Real fromWork(const Array<int>& work, std::int64_t base, bool negative);
static Real add(Real a, Real b);
static Real multiply(Real a, Real b);
static int compareDigits(const Array<int>& a, const Array<int>& b);
static Real divide(Real a, Real b);

// Мантисса little endian, знак и десятичный показатель — отдельные поля данных.
static void check(bool valid) {
    if (!valid) throw std::invalid_argument("Invalid real number or exponent overflow");
}
static int exponent(std::int64_t value) {
    check(value >= -1000000000 && value <= 1000000000);
    return (int)value;
}
static int highest(Real a) {
    for (int i = PRECISION - 1; i >= 0; i--)
        if (a.digits.data[i] != 0) return i;
    return -1;
}
static Real normalize(Real a) {
    if (highest(a) < 0) return Real{};
    int shift = 0;
    while (a.digits.data[shift] == 0) shift++;
    for (int i = 0; i < PRECISION; i++) a.digits.data[i] = i + shift < PRECISION ? a.digits.data[i + shift] : 0;
    a.exponent = exponent((std::int64_t)a.exponent + shift);
    return a;
}
static Real createReal(std::string text) {
    check(static_cast<int>(text.size()) > 0);
    Real a = Real{};
    a.negative = text[0] == '-';
    int i = text[0] == '-' || text[0] == '+' ? 1 : 0;
    int decimalPlaces = 0, significant = 0, stored = 0, total = 0;
    bool point = false, started = false;
    Array<int> digits = makeArray<int>(PRECISION);
    while (i < static_cast<int>(text.size()) && text[i] != 'e' && text[i] != 'E') {
        char c = text[i++];
        if (c == '.') {
            check(!point);
            point = true;
        } else {
            check(c >= '0' && c <= '9');
            total++;
            if (point) decimalPlaces++;
            if (c != '0') started = true;
            if (started) {
                significant++;
                if (stored < PRECISION) digits.data[stored++] = c - '0';
            }
        }
    }
    check(total > 0);
    std::int64_t scientific = 0;
    if (i < static_cast<int>(text.size())) {
        i++;
        bool negativeExponent = i < static_cast<int>(text.size()) && text[i] == '-';
        if (i < static_cast<int>(text.size()) && (text[i] == '+' || text[i] == '-')) i++;
        check(i < static_cast<int>(text.size()));
        while (i < static_cast<int>(text.size())) {
            char c = text[i++];
            check(c >= '0' && c <= '9');
            scientific = scientific * 10 + c - '0';
            check(scientific <= 1000000000);
        }
        if (negativeExponent) scientific = -scientific;
    }
    if (!started) return Real{};
    for (int j = 0; j < stored; j++) a.digits.data[j] = digits.data[stored - 1 - j];
    a.exponent = exponent(scientific - decimalPlaces + significant - stored);
    return normalize(a);
}
static std::string realText(Real a) {
    int top = highest(a);
    if (top < 0) return "0";
    std::string result = std::string(a.negative ? "-" : "") + std::to_string(a.digits.data[top]);
    if (top > 0) {
        result += ".";
        for (int i = top - 1; i >= 0; i--) result += static_cast<char>('0' + a.digits.data[i]);
    }
    return result + "e" + std::to_string((std::int64_t)a.exponent + top);
}
static int digitAt(Real a, std::int64_t power) {
    std::int64_t i = power - a.exponent;
    return i >= 0 && i < PRECISION ? a.digits.data[(int)i] : 0;
}
static int compareAbsolute(Real a, Real b) {
    int ah = highest(a), bh = highest(b);
    if (ah < 0) return bh < 0 ? 0 : -1;
    if (bh < 0) return 1;
    std::int64_t top = (std::int64_t)a.exponent + ah, other = (std::int64_t)b.exponent + bh;
    if (top != other) return top < other ? -1 : 1;
    for (int i = 0; i < PRECISION; i++) {
        int x = digitAt(a, top - i), y = digitAt(b, top - i);
        if (x != y) return x < y ? -1 : 1;
    }
    return 0;
}
static int compare(Real a, Real b) {
    if (a.negative != b.negative) return a.negative ? -1 : 1;
    int result = compareAbsolute(a, b);
    return a.negative ? -result : result;
}
// Сохраняем 40 старших значащих цифр, младшие отбрасываем.
static Real fromWork(const Array<int>& work, std::int64_t base, bool negative) {
    int top = work.length - 1;
    while (top > 0 && work.data[top] == 0) top--;
    if (work.data[top] == 0) return Real{};
    int shift = top + 1 > PRECISION ? top + 1 - PRECISION : 0;
    Real result = Real{};
    for (int i = 0; i < PRECISION && i + shift < work.length; i++) result.digits.data[i] = work.data[i + shift];
    result.negative = negative;
    result.exponent = exponent(base + shift);
    return normalize(result);
}
static Real add(Real a, Real b) {
    if (highest(a) < 0) return fromWork(b.digits, b.exponent, b.negative);
    if (highest(b) < 0) return fromWork(a.digits, a.exponent, a.negative);
    bool subtract = a.negative != b.negative;
    int comparison = compareAbsolute(a, b);
    if (subtract && comparison == 0) return Real{};
    if (subtract && comparison < 0) { Real temporary = a; a = b; b = temporary; }
    std::int64_t topA = (std::int64_t)a.exponent + highest(a), topB = (std::int64_t)b.exponent + highest(b);
    std::int64_t base = (topA > topB ? topA : topB) - 97;
    Array<int> work = makeArray<int>(100);
    int carry = 0;
    if (subtract)
        for (int j = 0; j < PRECISION && (std::int64_t)b.exponent + j < base; j++)
            if (b.digits.data[j] != 0) { carry = 1; break; }
    for (int i = 0; i < work.length; i++) {
        int x = digitAt(a, base + i), y = digitAt(b, base + i);
        if (subtract) {
            int difference = x - y - carry;
            carry = difference < 0 ? 1 : 0;
            work.data[i] = difference < 0 ? difference + 10 : difference;
        } else {
            int sum = x + y + carry;
            work.data[i] = sum % 10;
            carry = sum / 10;
        }
    }
    return fromWork(work, base, a.negative);
}
static Real multiply(Real a, Real b) {
    Array<int> work = makeArray<int>(PRECISION * 2 + 1);
    for (int i = 0; i < PRECISION; i++)
        for (int j = 0; j < PRECISION; j++) work.data[i + j] += a.digits.data[i] * b.digits.data[j];
    for (int i = 0; i + 1 < work.length; i++) {
        work.data[i + 1] += work.data[i] / 10;
        work.data[i] %= 10;
    }
    return fromWork(work, (std::int64_t)a.exponent + b.exponent, a.negative != b.negative);
}
static int compareDigits(const Array<int>& a, const Array<int>& b) {
    for (int i = a.length - 1; i >= 0; i--)
        if (a.data[i] != b.data[i]) return a.data[i] < b.data[i] ? -1 : 1;
    return 0;
}
static Real divide(Real a, Real b) {
    check(highest(b) >= 0);
    if (highest(a) < 0) return Real{};
    Array<int> remainder = makeArray<int>(PRECISION + 1), divisor = makeArray<int>(PRECISION + 1), quotient = makeArray<int>(PRECISION);
    for (int i = 0; i < PRECISION; i++) divisor.data[i] = b.digits.data[i];
    int written = 0, firstPower = 0, position = highest(a);
    bool started = false;
    while (written < PRECISION) {
        for (int j = PRECISION; j > 0; j--) remainder.data[j] = remainder.data[j - 1];
        remainder.data[0] = position >= 0 ? a.digits.data[position] : 0;
        int digit = 0;
        while (compareDigits(remainder, divisor) >= 0) {
            int borrow = 0;
            for (int j = 0; j < remainder.length; j++) {
                int difference = remainder.data[j] - divisor.data[j] - borrow;
                borrow = difference < 0 ? 1 : 0;
                remainder.data[j] = difference < 0 ? difference + 10 : difference;
            }
            digit++;
        }
        if (digit != 0 && !started) { started = true; firstPower = position; }
        if (started) quotient.data[written++] = digit;
        position--;
    }
    Real result = Real{};
    for (int i = 0; i < PRECISION; i++) result.digits.data[i] = quotient.data[PRECISION - 1 - i];
    result.negative = a.negative != b.negative;
    result.exponent = exponent((std::int64_t)a.exponent - b.exponent + firstPower - (PRECISION - 1));
    return normalize(result);
}
int main() {
    Real a = createReal("-256.2e-1"), b = createReal("-9534.07256e15"), c = createReal("9534.07256e15");
    std::cout << ("R1: ") << (realText(a)) << '\n';
    std::cout << ("R2: ") << (realText(b)) << '\n';
    std::cout << ("R3: ") << (realText(c)) << '\n';
    std::cout << ("R1 vs R2: ") << (compare(a, b)) << '\n';
    std::cout << ("R2 vs R3: ") << (compare(b, c)) << '\n';
    std::cout << ("R2 + R2: ") << (realText(add(b, b))) << '\n';
    std::cout << ("R1 + R3: ") << (realText(add(a, c))) << '\n';
    std::cout << ("R2 + R3: ") << (realText(add(b, c))) << '\n';
    std::cout << ("R1 * R2: ") << (realText(multiply(a, b))) << '\n';
    std::cout << ("R2 / R1: ") << (realText(divide(b, a))) << '\n';
    std::cout << ("1 / 3: ") << (realText(divide(createReal("1"), createReal("3")))) << '\n';
}
