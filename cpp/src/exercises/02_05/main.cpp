/*
Cvičení 02 — Úloha 2.5
Název: Nejmenší společný násobek
Body: 3 body

Zadání:
Uživatel zadá dvě čísla z klávesnice. Vypište jejich nejmenší společný násobek. Nejmenší společný
násobek čísel x a y je nejmenší číslo n takové, že x dělí n beze zbytku a zároveň y dělí n beze
zbytku.  [3 body]

Ukázka:

Spocteme nejmensi spolecny nasobek cisel

Zadej prvni cislo: 21

Zadej druhe cislo: 30

Nejmensi spolecny nasobek je 210

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

// Запускаем пример из упражнения и выводим результаты.
int main() {
    std::cout << std::boolalpha;
    std::cout << "Enter the first number:" << '\n';

    int a = read_int();

    std::cout << "Enter the second number:" << '\n';

    int b = read_int();

    // Этот алгоритм рассчитан на положительные a и b. b = 0 вызывает ошибку деления,
    // a = 0 сразу даёт 0; отрицательные значения не подходят для этого способа поиска.
    auto multiple = a;

    // Перебираем a, 2a, 3a, ... — они уже делятся на a. Первый такой кандидат,
    // который делится и на b без остатка, является наименьшим общим кратным.
    while (multiple % b != 0) {
        multiple += a;
    }

    std::cout << "The least common multiple is " << multiple << '\n';
    return 0;
}
