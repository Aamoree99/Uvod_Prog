/*
Cvičení 07 — Úloha 7.4
Název: Římská čísla
Body: 1 bod; 1 bod (v pořadí podúloh)

Zadání:
Římská čísla jsou příkladem nepoziční číselné soustavy. Hodnoty číslic nezávisí na jejich pozici
v zápise čísla. Hodnoty číslic jsou následující I = 1, V = 5, X = 10, L = 50, C = 100, D = 500, M =
1000. Existují i dvojice číslic, které se považují za jednu číslici: IV  = 4, IX = 9, XL = 40, XC =
90, CD 400, CM 900. Číslice jsou v zápisu čísla uspořádány od nejvyšší hodnoty po nejnižší.
Například číslo 2016 se zapíše jako MMXVI. Například číslo 1949 se zapíše zkráceným zápisem jako
MCMXLIX. Například výraz IM, který měl mít hodnotu 999, odporuje výše uvedeným pravidlům a není
povolen. Napište následující funkce:

a) Funkce DecRoman převede číslo v desítkové soustavě na římské číslo. [1 bod]

b) Funkce RomanDec převede římské číslo na číslo v desítkové soustavě [1 bod].


Ukázka:

string s = DecRoman(1969); // "MCMLXIX"

int i = RomanDec("MMCDXXXVII"); // 2437


Správnost programu si můžete ověřit na online konvertoru:

http://www.convertit.com/Go/ConvertIt/Calculators/Math/Roman_Numerals_Converter.ASP

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

static std::string decimalToRoman(int number);
static int romanValue(char c);
static int romanToDecimal(std::string text);

static std::string decimalToRoman(int number) {
    if (number < 1 || number > 3999) throw std::invalid_argument("Roman range is 1..3999");
    Array<int> values = arrayOf<int>({1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1});
    Array<std::string> symbols = arrayOf<std::string>({"M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"});
    std::string result = "";
    for (int i = 0; i < values.length; i++)
        while (number >= values.data[i]) {
            result += symbols.data[i];
            number -= values.data[i];
        }
    return result;
}
static int romanValue(char c) {
    std::string symbols = "IVXLCDM";
    Array<int> values = arrayOf<int>({1, 5, 10, 50, 100, 500, 1000});
    for (int i = 0; i < static_cast<int>(symbols.size()); i++)
        if (symbols[i] == c) return values.data[i];
    return 0;
}
static int romanToDecimal(std::string text) {
    if (static_cast<int>(text.size()) == 0 || static_cast<int>(text.size()) > 15) return -1;
    int result = 0;
    for (int i = 0; i < static_cast<int>(text.size()); i++) {
        int value = romanValue(text[i]);
        if (value == 0) return -1;
        result += i + 1 < static_cast<int>(text.size()) && value < romanValue(text[i + 1]) ? -value : value;
    }
    if (result < 1 || result > 3999 || !(decimalToRoman(result) == text)) return -1;
    return result;
}
int main() {
    std::cout << ("1969: ") << (decimalToRoman(1969)) << '\n';
    std::cout << ("MMCDXXXVII: ") << (romanToDecimal("MMCDXXXVII")) << '\n';
    std::cout << ("Invalid IM: ") << (romanToDecimal("IM")) << '\n';
    for (int i = 1; i <= 3999; i++)
        if (romanToDecimal(decimalToRoman(i)) != i) throw std::runtime_error("Roman round trip failed");
    std::cout << ("All numbers 1..3999 passed") << '\n';
}
