"""
Cvičení 02 — Úloha 2.6
Název: Prvočísla
Body: 3 body

Zadání:
Vypište všechna prvočísla menší než 1000 [3 body]


Ukázka:

Vsechna prvocisla mensi nez 1000: 2, 3, 5, . . . . 991, 997

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

def main():
    limit = 100000
    print("Sieve of Eratosthenes; extended limit: 100000.")
    is_prime = [True] * limit
    is_prime[0] = is_prime[1] = False
    number = 2
    while number * number < limit:
        if is_prime[number]:
            multiple = number * number
            while multiple < limit:
                is_prime[multiple] = False
                multiple += number
        number += 1
    number = 2
    count = 0
    while number < limit:
        if is_prime[number]:
            print(f"{number:>6}", end="")
            count += 1
            if count % 10 == 0:
                print()
        number += 1
    print(f"\nTotal prime count: {count}")

if __name__ == "__main__":
    main()
