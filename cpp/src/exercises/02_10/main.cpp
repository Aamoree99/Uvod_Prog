/*
Cvičení 02 — Úloha 2.10
Název: Druh posloupnosti
Body: 4 body

Zadání:
Uživatel zadává posloupnost kladných čísel. Pokud uživatel zadá nekladné číslo, je posloupnost
ukončena (a toto nekladné číslo není její součástí). Vypište jakého z následujících druhů je
posloupnost: krátká, rostoucí, neklesající, konstantní, nerostoucí, klesající, není monotónní.

a) Posloupnost je krátká, pokud obsahuje nejvýše jedno číslo. Ostatní druhy posloupností vyžadují
alespoň dvě čísla.

b) Posloupnost je konstantní, pokud hodnoty všech čísel jsou si rovny. Př.: 17, 17, 17, 17, 17.

c) Posloupnost je rostoucí, pokud pro každé číslo posloupnosti platí, že jeho hodnota je menší než
hodnota následujícího čísla. Př.: 2, 6, 19, 37, 38, 40, 95.

d) Posloupnost je klesající, pokud pro každé číslo posloupnosti platí, že jeho hodnota je větší než
hodnota následujícího čísla. Př.: 85, 67, 66, 14, 9, 5, 2, 1

e) Posloupnost je neklesající, pokud pro každé číslo posloupnosti platí, že jeho hodnota je menší
než hodnota následujícího čísla, nebo se hodnotě následujícího čísla rovná. V posloupnosti se musí
vyskytovat alespoň jedna dvojice po sobě následujících čísel, jejichž hodnoty jsou si rovny
(posloupnost není rostoucí) a alespoň jedna dvojice po sobě následujících čísel, jejichž hodnoty si
nejsou rovny (posloupnost není konstantní). Př.: 4, 11, 11, 26, 31, 45, 88.

f) Posloupnost je nerostoucí, pokud pro každé číslo posloupnosti platí, že jeho hodnota je větší
než hodnota následujícího čísla, nebo se hodnotě následujícího čísla rovná. V posloupnosti se musí
vyskytovat alespoň jedna dvojice po sobě následujících čísel, jejichž hodnoty jsou si rovny
(posloupnost není klesající) a alespoň jedna dvojice po sobě následujících čísel, jejichž hodnoty
si nejsou rovny (posloupnost není konstantní). Př.: 55, 49, 49, 49, 35, 20, 12, 9, 9, 6, 2.

g) V ostatních případech posloupnost není monotónní. Př.: 60, 20, 8, 8, 12, 5

                                                                                           [4 body]

Ukázka:

Zjistime druh posloupnosti.

Zadejte cislo: 15

Zadejte cislo: 8

Zadejte cislo: 6

Zadejte cislo: 6

Zadejte cislo: 2

Zadejte cislo: 0

Posloupnost je nerostouci.

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
    std::cout << "Determine the sequence type." << '\n';

    int count = 0;
    int previous = 0;

    // Флаги запоминают, встречались ли рост, спад и равенство соседних элементов.
    // Нам не нужно хранить всю последовательность — достаточно предыдущего числа.
    bool has_up = false;
    bool has_down = false;
    bool has_equal = false;

    while ((true)) {
        std::cout << "Enter a number:" << '\n';

        int current = read_int();

        // Это признак конца ввода; само число в последовательность не включаем.
        if (current <= 0) {
            break;
        }

        count += 1;

        // У первого элемента ещё нет предыдущего, поэтому сравниваем начиная со второго.
        if (count > 1) {
            if (current > previous) {
                has_up = true;
            } else if (current < previous) {
                has_down = true;
            } else {
                has_equal = true;
            }
        }

        // После сравнения текущее число становится предыдущим для следующего шага.
        previous = current;
    }

    // Классифицируем по сочетаниям флагов. Рост вместе со спадом означает немонотонность;
    // равенство без роста и спада — постоянную последовательность.
    if (count <= 1) {
        std::cout << "The sequence is too short." << '\n';
    } else if (has_equal == true && has_up == false && has_down == false) {
        std::cout << "The sequence is constant." << '\n';
    } else if (has_up == true && has_down == false && has_equal == false) {
        std::cout << "The sequence is strictly increasing." << '\n';
    } else if (has_down == true && has_up == false && has_equal == false) {
        std::cout << "The sequence is strictly decreasing." << '\n';
    } else if (has_up == true && has_equal == true && has_down == false) {
        std::cout << "The sequence is nondecreasing." << '\n';
    } else if (has_down == true && has_equal == true && has_up == false) {
        std::cout << "The sequence is nonincreasing." << '\n';
    } else {
        std::cout << "The sequence is not monotonic." << '\n';
    }
    return 0;
}
