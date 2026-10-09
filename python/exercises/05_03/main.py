"""
Cvičení 05 — Úloha 5.3
Název: Sestupné třídění
Body: 3+1 body; 3+1 body; 2+1 body; 3+2 body (v pořadí podúloh)

Zadání:
Napište třídící funkce pro sestupné třídění. Body navíc lze získat za použití pomocných funkcí
z příkladu 5.2 pro porovnávání a přiřazování hodnot prvků. Pomocné funkce nepoužívejte pro práci
s řídící proměnnou cyklu. Výjimkou je Count sort, u kterého pomocné funkce se používají pro řídící
proměnnou cyklu v situacích, ve kterých je řídící proměnná cyklu brána jako prvek pole.

a) Selection sort [3+1 body]

b) Insertion sort [3+1  body]

c) Bubble sort [2+1  body]

d) Count sort [3+2 body]


Ukázka:

int[] pole = { 5, 7, 1, 5, 6, 8, 9, 0, 1, 2};

SelectionSort(pole); // 9, 8, 7, 6, 5, 5, 2, 1, 1, 0

Console.WriteLine("Prirazeni: {0}, Porovnani: {1}", prirazeni, porovnani); // 30,  55

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

MAX_NUMBER = 1000

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


def selection_sort(array):
    i = 0
    while i < len(array):
        max_index = i
        j = i + 1
        while j < len(array):
            if is_greater(array[j], array[max_index]):
                max_index = j
            j += 1
        temporary = assign(array[i])
        array[i] = assign(array[max_index])
        array[max_index] = assign(temporary)
        i += 1


def insertion_sort(array):
    i = 1
    while i < len(array):
        key = assign(array[i])
        j = i
        while j > 0:
            if is_less(array[j - 1], key):
                array[j] = assign(array[j - 1])
                j -= 1
            else:
                break
        array[j] = assign(key)
        i += 1


def bubble_sort(array):
    i = 0
    while i < len(array):
        j = 0
        while j + 1 < len(array) - i:
            if is_less(array[j], array[j + 1]):
                temporary = assign(array[j])
                array[j] = assign(array[j + 1])
                array[j + 1] = assign(temporary)
            j += 1
        i += 1


def counting_sort(array):
    if len(array) == 0:
        return
    max_value = 0
    i = 0
    while i < len(array):
        if array[i] < 0 or array[i] > MAX_NUMBER:
            raise ValueError("Counting sort value is outside the supported range")
        if is_greater(array[i], max_value):
            max_value = assign(array[i])
        i += 1
    counts = [0] * (MAX_NUMBER + 1)
    i = 0
    while i < len(array):
        index = array[i]
        counts[index] = assign(counts[index] + 1)
        i += 1
    position = 0
    number = max_value
    while number >= 0:
        while is_greater(counts[number], 0):
            array[position] = assign(number)
            counts[number] = assign(counts[number] - 1)
            position += 1
        number -= 1


def main():
    original = [5, 7, 1, 5, 6, 8, 9, 0, 1, 2]
    algorithms = (selection_sort, insertion_sort, bubble_sort, counting_sort)
    names = ("Selection sort", "Insertion sort", "Bubble sort", "Counting sort")
    print(f"Original array: {original}")
    i = 0
    while i < 4:
        array = original.copy()
        reset_counters()
        algorithms[i](array)
        print(f"{names[i]}\nSorted array: {array}\nAssignments: {assignments}\nComparisons: {comparisons}")
        i += 1

if __name__ == "__main__":
    main()
