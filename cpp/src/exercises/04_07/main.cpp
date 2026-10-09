/*
Cvičení 04 — Úloha 4.7
Název: Skalární součin
Body: 2 body

Zadání:
Napište funkci, která spočte skalární součin dvou stejně dlouhých vektorů. Vektor o n složkách je
reprezentovaný pomocí pole délky n. Skalární součin pro dva vektory délky n se počítá jako součet n
součinů. Jednotlivé součiny vzniknou vynásobením odpovídajících složek prvního a druhého vektoru [2
body].


Ukázka:

int[] u = { 2, 0, -5, 1, 4 };

int[] v = { 7, 4, 2, -2, 0 };

skalar = SkalarniSoucin(u, v); // 2

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
#include <climits>

// Векторы имеют одинаковую длину; ссылка sum сохраняет накопленную сумму.
void scalar_product(const int first[], const int second[], int length, int& sum) {
    int i = 0;
    while (i < length) {
        sum += first[i] * second[i];
        i += 1;
    }
}
// Запускаем пример из упражнения и выводим результаты.
int main() {
    const int first[] = {2, 0, -5, 1, 4};
    const int second[] = {7, 4, 2, -2, 0};
    int sum = 0;
    scalar_product(first, second, 5, sum);
    std::cout << "The scalar product is " << sum << '\n';
}
