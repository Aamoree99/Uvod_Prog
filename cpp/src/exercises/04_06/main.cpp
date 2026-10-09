/*
Cvičení 04 — Úloha 4.6
Název: Nejčastější prvek.
Body: 3 body

Zadání:
Napište funkci, která v zadaném poli najde nejčastěji se vyskytující hodnotu prvku. Vyskytují-li se
dvě hodnoty stejně často, přednost má první vyskytnuvši se hodnota. [3 body]


Ukázka:

int[] pole = { 2, 7, 6, 6, 6, 1, 7, 7, 2};

modus = NejcastejsiPrvek(pole); // 7

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

// Читаем массив и возвращаем частоту и элемент через ссылки.
void most_frequent_element(const int array[], int length, int& frequency, int& most_frequent) {
    int i = 0;
    while (i < length) {
        int occurrence_count = 0, j = 0;
        while (j < length) {
            if (array[i] == array[j]) occurrence_count += 1;
            j += 1;
        }
        // При равенстве частот сохраняем первый встретившийся элемент.
        if (occurrence_count > frequency) {
            frequency = occurrence_count;
            most_frequent = array[i];
        }
        i += 1;
    }
}
// Запускаем пример из упражнения и выводим результаты.
int main() {
    const int array[] = {2, 7, 6, 6, 6, 1, 7, 7, 2};
    int frequency = 0, most_frequent = 0;
    most_frequent_element(array, 9, frequency, most_frequent);
    std::cout << "Array: [2, 7, 6, 6, 6, 1, 7, 7, 2]\n";
    std::cout << "The most frequent number is " << most_frequent << ", occurring " << frequency << " times\n";
}
