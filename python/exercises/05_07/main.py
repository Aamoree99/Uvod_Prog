"""
Cvičení 05 — Úloha 5.7
Název: Vylepšený Count Sort
Body: 3 body

Zadání:
Rozšiřte funkci z příkladu  5.3d pro sestupné setřídění pole pomocí třídícího algoritmu Count Sort.
Hodnoty prvků pole jsou z omezeného intervalu, který může obsahovat i záporná čísla [3 body]


Ukázka:

int[] p1 = { -5, 7, 15, 22, 4, 0, 4, 9, -11 };

CountSort(p1); // 22, 15, 9, 7, 4, 4, 0, -5, -11

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
"""

assignments = 0
comparisons = 0


def reset_counters():
    global assignments, comparisons
    assignments = comparisons = 0


def assign(value):
    global assignments
    assignments += 1
    return value


def is_less(a, b):
    global comparisons
    comparisons += 1
    return a < b


def is_greater(a, b):
    global comparisons
    comparisons += 1
    return a > b


def is_descending(array):
    i = 1
    while i < len(array):
        if array[i] > array[i - 1]:
            return False
        i += 1
    return True


def counting_sort(array):
    if len(array) == 0:
        return
    min_value = max_value = array[0]
    i = 1
    while i < len(array):
        if is_less(array[i], min_value):
            min_value = assign(array[i])
        if is_greater(array[i], max_value):
            max_value = assign(array[i])
        i += 1
    counts = [0] * (max_value - min_value + 1)
    i = 0
    while i < len(array):
        index = array[i] - min_value
        counts[index] = assign(counts[index] + 1)
        i += 1
    position = 0
    index = len(counts) - 1
    while index >= 0:
        number = index + min_value
        while is_greater(counts[index], 0):
            array[position] = assign(number)
            counts[index] = assign(counts[index] - 1)
            position += 1
        index -= 1


def main():
    array = [-5, 7, 15, 22, 4, 0, 4, 9, -11]
    print(f"Original array: {array}")
    counting_sort(array)
    print(f"Sorted array: {array}\nSorted correctly: {is_descending(array)}")
    print(f"Assignments: {assignments}\nComparisons: {comparisons}")

if __name__ == "__main__":
    main()
