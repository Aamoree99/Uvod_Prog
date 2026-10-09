"""
Cvičení 05 — Úloha 5.5
Název: Binární vyhledávání
Body: 2 body; +1 bod (v pořadí podúloh)

Zadání:
Napište funkci hledání zadané hodnoty v sestupně setříděném poli za použití algoritmu binárního
vyhledávání:

a) Je vrácen index libovolného prvku s hledanou hodnotou. [2 body]

b) Je vrácen index prvního prvku v poli, který má hledanou hodnotu. [+1 bod]


Ukázka

int[] p1 = { 12, 9, 6, 6, 6, 5, 3, 1, 1 };

int index = BinarniVyhledavani(p1, 6);

// (a) index = 2, 3, nebo 4 (b) index = 2

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

def binary_search(array, target):
    left, right = 0, len(array)
    while left < right:
        middle = left + (right - left) // 2
        if array[middle] == target:
            return middle
        if array[middle] > target:
            left = middle + 1
        else:
            right = middle
    return -1


def binary_search_first(array, target):
    left, right = 0, len(array)
    result = -1
    while left < right:
        middle = left + (right - left) // 2
        if array[middle] == target:
            result = middle
            right = middle
        elif array[middle] > target:
            left = middle + 1
        else:
            right = middle
    return result


def main():
    array = [12, 9, 6, 6, 6, 5, 3, 1, 1]
    print(f"Array: {array}\nTarget: 6")
    print(f"A) Any matching index: {binary_search(array, 6)}")
    print(f"B) First matching index: {binary_search_first(array, 6)}")
    print(f"Search for missing number 10: {binary_search(array, 10)}")

if __name__ == "__main__":
    main()
