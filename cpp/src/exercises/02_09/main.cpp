/*
Cvičení 02 — Úloha 2.9
Název: Statistické informace
Body: 1/2 bodu; 1/2 bodu; 1 bod; 1 bod; 1 bod; 1 bod (v pořadí podúloh)

Zadání:
Uživatel zadává posloupnost kladných čísel. Pokud uživatel zadá nekladné číslo, je posloupnost
ukončena (a toto nekladné číslo není její součástí). Vypište:

a) počet čísel v posloupnosti. Pokud posloupnost obsahuje nejvýše jedno číslo, nevypisujte další
části úkolu. [1/2 bodu]

b) součet hodnot čísel [1/2 bodu]

c) hodnotu největšího čísla [1 bod]

d) hodnotu nejmenšího čísla [1 bod]

e) hodnotu druhého největšího číslo [1 bod]

f) hodnotu druhého nejmenšího číslo [1 bod]

Ukázka:

Spocteme statisticke informace o zadanych kladnych cislech.

Zadejte cislo: 5

Zadejte cislo: 6

Zadejte cislo: 11

Zadejte cislo: 22

Zadejte cislo: 15

Zadejte cislo: 0

Delka posloupnosti je 5.

Soucet prvku posloupnosti je 59.

Nejmensi prvek posloupnosti je 5, druhy nejmensi je 6.

Nejvetsi prvek posloupnosti je 22, druhy nejvetsi je 15.

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
#include <limits>
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
    int count = 0;
    int sum = 0;

    // Начальные границы позволяют первому числу заменить максимум и минимум.
    // В этой задаче вводятся только положительные числа, поэтому границы не совпадут с вводом.
    int max = std::numeric_limits<int>::min();
    int sec_max = std::numeric_limits<int>::min();

    int min = std::numeric_limits<int>::max();
    int sec_min = std::numeric_limits<int>::max();

    while ((true)) {
        std::cout << "Enter a number:" << '\n';

        // Читаем одно целое число на строке. trim убирает пробелы, parse разбирает число;
        // Предполагается корректный ввод.
        int a = read_int();

        // Неположительное число завершает ввод и не входит ни в сумму, ни в количество.
        if (a <= 0) {
            break;
        }

        count += 1;
        sum += a;

        // Храним два наибольших элемента. Новый максимум сначала сдвигает старый на второе
        // место. Иначе кандидат может улучшить только второй максимум. Порядок присваиваний важен.
        // Повторы учитываются: для 9, 9, 5 оба максимума равны 9.
        if (a > max) {
            sec_max = max;
            max = a;
        } else if (a > sec_max) {
            sec_max = a;
        }

        // С минимумами делаем то же самое, но сравнения направлены в другую сторону.
        if (a < min) {
            sec_min = min;
            min = a;
        } else if (a < sec_min) {
            sec_min = a;
        }
    }

    std::cout << "The sequence length is " << count << "." << '\n';

    // При нуле или одном элементе по условию выводим только длину последовательности.
    if (count > 1) {
        std::cout << "The sum of the sequence is " << sum << "." << '\n';
        std::cout << "The smallest sequence element is " << min << ", the second smallest is " << sec_min << "." << '\n';
        std::cout << "The largest sequence element is " << max << ", the second largest is " << sec_max << "." << '\n';
    }
    return 0;
}
