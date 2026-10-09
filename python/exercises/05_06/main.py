"""
Cvičení 05 — Úloha 5.6
Název: Vylepšený Bubble Sort
Body: 2 body; 1 bod (v pořadí podúloh)

Zadání:
Algoritmus Bubble Sort třídící sestupně zadané pole lze implementovat ve dvou verzích. V základní
verzi algoritmu se ve vnořeném cyklu prochází pole až do posledního prvku.  Ve vylepšené verzi
algoritmu se ve vnořeném cyklu prochází pole jen do místa poslední výměny, která proběhla
v předchozí iteraci vnějšího cyklu.

a) Napište vylepšenou verzi Bubble sortu. Pokud jste vylepšenou verzi napsali již v příklad 5.3c,
napište nyní verzi základní. [2 body]

b) Změřte (dle příkladu 5.4) počty operací a dobu běhu vylepšené a základní verze. [1 bod]

Ukázka:

Prvku 1000

Bubble sort – porovnani: 944055, prirazeni: 763212

Bubble sort2 – porovnani: 496415, prirazeni: 763212

Bubble sort – doba behu 46 ms

Bubble sort2 – doba behu 30 ms

Prvku 10000

...

Prvku 30000

...

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


def swap_elements(array, index):
    temporary = assign(array[index])
    array[index] = assign(array[index + 1])
    array[index + 1] = assign(temporary)


def bubble_sort(array):
    i = 0
    while i < len(array) - 1:
        j = 0
        while j + 1 < len(array):
            if is_less(array[j], array[j + 1]):
                swap_elements(array, j)
            j += 1
        i += 1


def improved_bubble_sort(array):
    limit = len(array)
    while limit > 1:
        last_swap = 0
        j = 0
        while j + 1 < limit:
            if is_less(array[j], array[j + 1]):
                swap_elements(array, j)
                last_swap = j + 1
            j += 1
        limit = last_swap


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
    example = [5, 7, 1, 5, 6, 8, 9, 0, 1, 2]
    basic, improved = example.copy(), example.copy()
    bubble_sort(basic)
    improved_bubble_sort(improved)
    print(f"Original: {example}\nBasic: {basic}\nImproved: {improved}")
    sizes = (1000, 10000, 30000)
    i = 0
    while i < len(sizes):
        original = [0] * sizes[i]
        fill_random(original)
        print(f"\nARRAY SIZE: {sizes[i]}", flush=True)
        print("Algorithm         |  Comparisons |  Assignments |    Time (ms) | Sorted", flush=True)
        benchmark(bubble_sort, "Basic Bubble", original)
        benchmark(improved_bubble_sort, "Improved Bubble", original)
        i += 1
    print("Benchmark completed.")

if __name__ == "__main__":
    main()
