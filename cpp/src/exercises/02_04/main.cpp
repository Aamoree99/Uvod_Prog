/*
Cvičení 02 — Úloha 2.4
Název: Faktoriál
Body: 1 bod

Zadání:
Uživatel zadá číslo. Vypište faktoriál tohoto čísla. Faktoriál čísla 0 je definován jako 1,
faktoriál pro číslo n>0 je definovaný jako součin všech čísel od 1 do n. Pro záporná čísla není
faktoriál definován. Př.: 6!  = 1*2*3*4*5*6 =720. [1 bod]

Ukázka:

Vypocteme faktorial kladneho cisla

Zadejte cislo: 6

Faktorial je 720

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

#include <iostream>
#include <string>
#include <cctype>
#include <stdexcept>

// Читаем одно целое число из строки и проверяем ввод.
int read_int() {
    std::string line;
    if (!std::getline(std::cin, line)) throw std::runtime_error("End of input");
    std::size_t end;
    int number = std::stoi(line, &end);
    while (end < line.size() && std::isspace(static_cast<unsigned char>(line[end]))) ++end;
    if (end != line.size()) throw std::runtime_error("Invalid integer");
    return number;
}

int main() {
    std::cout << std::boolalpha;
    std::cout << "Zadejte cislo:" << '\n';

    // trim убирает перевод строки, parse преобразует текст в int. Ввод должен быть целым числом.
    int a = read_int();

    // Факториал определён для неотрицательных целых. В Rust 34! помещается в u128, а 35! уже нет; сохраняем этот предел.
    if (a < 0) {
        std::cout << "Faktorial zaporneho cisla neni definovan." << '\n';
    } else if (a > 34) {
        std::cout << "Hodnota je prilis vysoka pro datovy typ u128." << '\n';
    } else {
        // Начинаем произведение с 1. Для 0 цикл не выполняется, поэтому 0! = 1.
        // 34! содержит 39 цифр. Массив цифр позволяет считать точно без внешних библиотек.
        int digits[40] = {1};
        int length = 1;
        int number = a;
        while (number >= 1) {
            int carry = 0;
            int i = 0;
            while (i < length) {
                int product = digits[i] * number + carry;
                digits[i] = product % 10;
                carry = product / 10;
                ++i;
            }
            while (carry > 0) {
                digits[length++] = carry % 10;
                carry /= 10;
            }
            --number;
        }
        std::cout << "Faktorial je ";
        int i = length - 1;
        while (i >= 0) std::cout << digits[i--];
        std::cout << '\n';
    }
    return 0;
}
