/*
Cvičení 02 — Úloha 2.8
Název: Armstrongova čísla
Body: 4 body

Zadání:
Vypište všechna Armstrongova čísla menší než 100 000. Armstrongovo číslo řádu n je n-ciferné číslo,
které je rovno součtu n-tých mocnin svých cifer. Například jakékoliv jednociferné číslo je
Armstrongovo číslo řádu 1, číslo 153 = 1*1*1+5*5*5+3*3*3 je Armstrongovo číslo řádu 3. [4 body]

Ukázka:

Vsechna Armstrongova cisla mensi nez 100000: 1, 2, . . . 93084

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

// Запускаем пример из упражнения и выводим результаты.
int main() {
    std::cout << std::boolalpha;
    int limit = 100000;
    // Перебираем положительные числа меньше 100000, как в примере условия.
    int number = 1;

    while (number < limit) {
        // Работаем с копией, чтобы исходное number сохранилось для сравнения.
        int remaining_digits = number;
        int digits = 0;

        // Целочисленное деление на 10 удаляет последнюю цифру. Первый цикл считает цифры.
        while (remaining_digits > 0) {
            digits += 1;
            remaining_digits /= 10;
        }

        remaining_digits = number;

        int sum = 0;

        while (remaining_digits > 0) {
            // Остаток % 10 извлекает последнюю цифру. Во втором цикле возводим каждую цифру
            // в степень digits повторным умножением и складываем результаты.
            auto digit = remaining_digits % 10;

            int power = 1;
            int count = 0;

            while (count < digits) {
                power *= digit;
                count += 1;
            }

            sum += power;

            remaining_digits /= 10;
        }

        // Если сумма степеней цифр совпала с исходным числом, оно является числом Армстронга.
        if (sum == number) {
            std::cout << number << " ";
        }

        number += 1;
    }

    std::cout << '\n';
    return 0;
}
