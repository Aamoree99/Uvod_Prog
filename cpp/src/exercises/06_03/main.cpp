/*
Cvičení 06 — Úloha 6.3
Název: Naše vlastní řetězce
Body: 2 body; 2 body; 1 bod; 2 body; 2 body; 4 body (v pořadí podúloh)

Zadání:
Napište funkce pracující s polem znaků char[], které budou mít stejnou funkčnost jako funkce pro
textové řetězce string. (Aniž byste pole znaků char[], na textový řetězec string převedli).

a) Funkce lexikografického porovnání řetězců s1 a s2: Compare(s1, s2). [2 body]

b) Funkce vracející index prvního výskytu hledaného řetězce s2 v řetězci s1. Řetězec s1 se
prohledává od zadané pozice start: IndexOf(s1, s2, start). [2 body]

c) Funkce vracející podřetězec zadaného řetězce s1. Podřetězec začíná na zadané pozici a má zadanou
délku: Substring(s1, start, delka). [1 bod]

d) Funkce vracející řetězec vzniklý vložením řetězce s2 do řetězce s1 na pozici zadanou indexem:
Insert(s1, index, s2). [2 body]

e) Funkce vracející řetězec vzniklý odstraněním podřetězce ze zadaného řetězce s1. Z řetězce s1 se
odstraní podřetězec určený indexem (v rámci řetězce s1) svého prvního znaku a svojí délkou:
Remove(s1, index, delka). [2 body]

f) Funkce vracející řetězec vzniklý z řetězce s1 nahrazením všech výskytů řetězce s2 za řetězec s3:
Replace (s1, s2, s3). [4 body]


Ukázka:

char[] s1 = "klokan".ToCharArray();

char[] s2 = "klobouk".ToCharArray();

int i1 = Compare(s1, s2); // 1

int i2 = IndexOf(s2, "o".ToCharArray(), 3); // 4

char[] s3 = Substring(s2, 2, 3); // 'o''b''o'

char[] s4 = Insert(s1, 1, s3); // 'k''o''b''o''l''o''k''a''n'

char[] s5 = Remove(s2, 3, 2); // 'k''l''o''u''k''

char[] s6 = Replace(s5, "k".ToCharArray(), s3);

// 'o''b''o''l''o''u''o''b''o'

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

static void check(bool valid);
static int compare(const Array<char>& a, const Array<char>& b);
static int indexOf(const Array<char>& a, const Array<char>& b, int start);
static Array<char> substring(const Array<char>& a, int start, int length);
static Array<char> insert(const Array<char>& a, int index, const Array<char>& b);
static Array<char> remove(const Array<char>& a, int index, int length);
static Array<char> replace(const Array<char>& a, const Array<char>& search, const Array<char>& replacement);
static void printChars(const Array<char>& a);

static void check(bool valid) {
    if (!valid) throw std::invalid_argument("Invalid character array bounds");
}
static int compare(const Array<char>& a, const Array<char>& b) {
    int i = 0;
    while (i < a.length && i < b.length) {
        if (a.data[i] != b.data[i]) return a.data[i] < b.data[i] ? -1 : 1;
        i++;
    }
    return a.length == b.length ? 0 : (a.length < b.length ? -1 : 1);
}
static int indexOf(const Array<char>& a, const Array<char>& b, int start) {
    if (start < 0 || start > a.length) return -1;
    for (int i = start; i <= a.length - b.length; i++) {
        int j = 0;
        while (j < b.length && a.data[i + j] == b.data[j]) j++;
        if (j == b.length) return i;
    }
    return -1;
}
static Array<char> substring(const Array<char>& a, int start, int length) {
    check(start >= 0 && length >= 0 && start <= a.length - length && length <= 100);
    Array<char> result = makeArray<char>(length);
    for (int i = 0; i < length; i++) result.data[i] = a.data[start + i];
    return result;
}
static Array<char> insert(const Array<char>& a, int index, const Array<char>& b) {
    check(index >= 0 && index <= a.length && a.length + b.length <= 100);
    Array<char> result = makeArray<char>(a.length + b.length);
    for (int i = 0; i < result.length; i++)
        result.data[i] = i < index ? a.data[i] : (i < index + b.length ? b.data[i - index] : a.data[i - b.length]);
    return result;
}
static Array<char> remove(const Array<char>& a, int index, int length) {
    check(index >= 0 && length >= 0 && index <= a.length - length && a.length - length <= 100);
    Array<char> result = makeArray<char>(a.length - length);
    for (int i = 0; i < result.length; i++) result.data[i] = i < index ? a.data[i] : a.data[i + length];
    return result;
}
static Array<char> replace(const Array<char>& a, const Array<char>& search, const Array<char>& replacement) {
    check(search.length > 0);
    Array<char> work = makeArray<char>(100);
    int i = 0, position = 0;
    while (i < a.length) {
        int j = 0;
        while (j < search.length && i + j < a.length && a.data[i + j] == search.data[j]) j++;
        if (j == search.length) {
            check(position + replacement.length <= 100);
            for (int k = 0; k < replacement.length; k++) work.data[position++] = replacement.data[k];
            i += search.length;
        } else {
            check(position < 100);
            work.data[position++] = a.data[i++];
        }
    }
    return substring(work, 0, position);
}
static void printChars(const Array<char>& a) {
    for (int i = 0; i < a.length; i++) std::cout << (a.data[i]);
    std::cout << '\n';
}
int main() {
    Array<char> a = arrayOf<char>({'k', 'l', 'o', 'k', 'a', 'n'}), b = arrayOf<char>({'k', 'l', 'o', 'b', 'o', 'u', 'k'});
    Array<char> o = arrayOf<char>({'o'}), k = arrayOf<char>({'k'});
    std::cout << ("Compare: ") << (compare(a, b)) << '\n';
    std::cout << ("IndexOf: ") << (indexOf(b, o, 3)) << '\n';
    Array<char> c = substring(b, 2, 3);
    printChars(c);
    printChars(insert(a, 1, c));
    Array<char> d = remove(b, 3, 2);
    printChars(d);
    printChars(replace(d, k, c));
}
