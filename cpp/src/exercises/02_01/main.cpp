/*
Cvičení 02 — Úloha 2.1
Název: Dvě čísla sestupně
Body: 1 bod

Zadání:
Uživatel zadá dvě čísla z klávesnice. Setřiďte tato číslo ve vnitřní paměti. Vypište tato čísla
setříděná sestupně dle jejich hodnot. [1 bod]

Ukázka:

Setridime dve cisla sestupne

Zadejte prvni cislo: 5

Zadejte druhe cislo: 9

Setridena cisla jsou 9, 5

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

// Стандартный ввод нужен для чтения с клавиатуры. В этом примере вводим целые числа по одному на строке.

int main() {
    std::cout << std::boolalpha;
    std::cout << "Setridime dve cisla sestupne" << '\n';

    std::cout << "Zadejte prvni cislo:" << '\n';
    // Читаем строку со стандартного ввода и убираем окружающие пробелы;
    // преобразуем её в целое число. Некорректный ввод вызывает ошибку.

    int a = read_int();

    std::cout << "Zadejte druhe cislo:" << '\n';
    // Следующее число читается отдельно, без накопления предыдущего ввода.

    int b = read_int();

    // Для убывающего порядка первое число должно быть не меньше второго.
    // Временная переменная сохраняет a, чтобы оно не потерялось при обмене.
    if (a < b) {
        auto temp = a;
        a = b;
        b = temp;
    }

    std::cout << "Setridena cisla jsou " << a << ", " << b << '\n';
    return 0;
}
