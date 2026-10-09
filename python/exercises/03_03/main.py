"""
Cvičení 03 — Úloha 3.3
Název: Aritmetický výraz
Body: 1 bod; 1 bod; 1 bod; 1 bod (v pořadí podúloh)

Zadání:
Napište následující funkce bez použití funkcí z Math. Návratové hodnoty funkcí budou vždy reálná
čísla. Reálná čísla jsou schopna reprezentovat vyšší číselné hodnoty než čísla celá

a) Funkci, která pro zadané celé číslo spočte jeho faktoriál. [1 bod]

b) Funkci, která pro zadané reálné číslo spočte jeho nezápornou celočíselnou mocninu [1 bod]

c) Funkci, která pro zadané reálné číslo n spočte hodnotu e^n, která je přibližně dána níže
uvedeným vztahem. [1 bod]

e^n ≈ Σ_{k=0}^{100} n^k / k!



d) Funkci, která pro zadaná celá čísla n a k spočte jejich kombinační číslo. Pro dvojice n a k
nesplňující podmínku n ≥ k ≥ 0 je kombinační číslo rovno 0, pro ostatní dvojice je kombinační číslo
dané níže uvedeným vztahem. [1 bod]

(n nad k) = n! / (k! (n − k)!)

e^n ≈ Σ_{k=0}^{100} n^k / k!


Ukázka:

x = faktorial(11); // 39916800

y = mocnina(5, 3); // 125.0

z = mocnina(2.69, 7); //1019.215

a = vyraz(5.14); // 170.715

b = kombinace(49, 6); // 13983816.0

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

def factorial(n):
    if n < 0:
        return 0.0
    result = 1.0
    i = 1
    while i <= n:
        result *= i
        i += 1
    return result


def power(number, exponent):
    result = 1.0
    i = 0
    while i < exponent:
        result *= number
        i += 1
    return result


def exponential_series(number):
    total = 0.0
    k = 0
    while k <= 100:
        total += power(number, k) / factorial(k)
        k += 1
    return total


def combination(n, k):
    if n < k or k < 0:
        return 0.0
    return factorial(n) / (factorial(k) * factorial(n - k))


def main():
    print(f"11! = {factorial(11):.0f}")
    print(f"5^3 = {power(5, 3):.1f}")
    print(f"2.69^7 = {power(2.69, 7):.3f}")
    print(f"e^5.14 ≈ {exponential_series(5.14):.3f}")
    print(f"C(49, 6) = {combination(49, 6):.1f}")

if __name__ == "__main__":
    main()
