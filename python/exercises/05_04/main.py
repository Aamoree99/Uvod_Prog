"""
Cvičení 05 — Úloha 5.4
Název: Testování rychlostí algoritmů
Body: 2 body; 2 body (v pořadí podúloh)

Zadání:
Otestujte třídící algoritmy z příkladu 5.3 na polích různé velikosti (1000, 10 000 a 30 000 prvků),
naplněných náhodnými hodnotami z intervalu 0 až velikost pole. Všechny třídící algoritmy budou
pracovat nad kopií stejného pole. Výsledky přehledně prezentujte.

a) s použitím počtu operací [2 body]

b) s použitím měření času [2 body]


Ukázka – část (a):

Prvku 1000

Selection sort – porovnani: 500500, prirazeni: 3000

Insertion sort – porovnani: 259065, prirazeni: 260069

Bubble sort – porovnani: 974025, prirazeni: 774213

Count sort – porovnani: 2995, prirazeni: 3006

Prvku 10000

...

Prvku 30000

...


Ukázka – část (b):

Prvku 1000

...

Prvku 10000

...

Prvku 30000

Selection sort – doba behu 11194 ms

Insertion sort – doba behu 9300 ms

Bubble sort – doba behu 39415 ms

Count sort – doba behu 4 ms

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

import random
import time

MAX_NUMBER = 30000

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
    counts = [0] * (MAX_NUMBER + 1)
    max_value = 0
    i = 0
    while i < len(array):
        value = array[i]
        if value < 0 or value > MAX_NUMBER:
            raise ValueError("Counting sort value is outside the supported range")
        if is_greater(value, max_value):
            max_value = assign(value)
        counts[value] = assign(counts[value] + 1)
        i += 1
    position = 0
    number = max_value
    while number >= 0:
        while is_greater(counts[number], 0):
            array[position] = assign(number)
            counts[number] = assign(counts[number] - 1)
            position += 1
        number -= 1


def is_descending(array):
    i = 1
    while i < len(array):
        if array[i] > array[i - 1]:
            return False
        i += 1
    return True


def fill_random(array):
    i = 0
    while i < len(array):
        array[i] = random.randint(0, len(array))
        i += 1


def benchmark(algorithm, name, original):
    # Копирование, генерация и проверка находятся за границами замера.
    array = original.copy()
    reset_counters()
    start = time.perf_counter()
    algorithm(array)
    milliseconds = (time.perf_counter() - start) * 1000
    correct = is_descending(array)
    print(f"{name:<17} | {comparisons:>12} | {assignments:>12} | {milliseconds:>12.3f} | {'OK' if correct else 'FAILED'}", flush=True)
    if not correct:
        raise RuntimeError("Sorting failed")


def main():
    sizes = (1000, 10000, 30000)
    algorithms = (selection_sort, insertion_sort, bubble_sort, counting_sort)
    names = ("Selection sort", "Insertion sort", "Bubble sort", "Counting sort")
    i = 0
    while i < len(sizes):
        original = [0] * sizes[i]
        fill_random(original)
        print(f"\nARRAY SIZE: {sizes[i]}", flush=True)
        print("Algorithm         |  Comparisons |  Assignments |    Time (ms) | Sorted", flush=True)
        j = 0
        while j < 4:
            benchmark(algorithms[j], names[j], original)
            j += 1
        i += 1
    print("Benchmark completed.")

if __name__ == "__main__":
    main()
