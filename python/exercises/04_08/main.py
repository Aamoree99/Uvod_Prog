"""
Cvičení 04 — Úloha 4.8
Název: Zaplatit mincemi
Body: 4 body; 2 body (v pořadí podúloh)

Zadání:
a) Napište funkci, která pro zadané pole obsahující sestupně setříděné nominální hodnoty mincí
platících v ČR spočte minimální počet kusů mincí, které je nutno použít pro zaplacení zadané
částky.  [4 body].

b) Najděte sadu mincí (jinou než v ukázce) a částku, pro kterou funkce nenajde optimální řešení.
Nalezená sada pravděpodobně nebude platit v žádné zemi na světě. [2 body]


Ukázka:

int[] mince = { 50, 20, 10, 5, 2, 1 };

pocet = ZaplatitMincemi(mince, 97); // 5

pocet = ZaplatitMincemi(mince, 88); // 6

pocet = ZaplatitMincemi(mince, 26); // 3

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

def pay_with_coins(coins, amount):
    count = 0
    i = 0
    while amount > 0:
        if i == len(coins):
            raise ValueError("Amount cannot be paid")
        pieces = amount // coins[i]
        count += pieces
        amount -= pieces * coins[i]
        i += 1
    return count


def optimal_coin_count(coins, index, amount):
    if amount == 0:
        return 0
    if index >= len(coins):
        return float("inf")
    count = amount // coins[index]
    minimum = float("inf")
    while count >= 0:
        rest = optimal_coin_count(coins, index + 1, amount - count * coins[index])
        if count + rest < minimum:
            minimum = count + rest
        count -= 1
    return minimum


def main():
    coins = [50, 20, 10, 5, 2, 1]
    amounts = (97, 88, 26)
    i = 0
    while i < len(amounts):
        print(f"The minimum number of coins needed to pay {amounts[i]} CZK is {pay_with_coins(coins, amounts[i])}.")
        i += 1
    strange = [4, 3, 1]
    print(f"Coin set: {strange}; amount: 6")
    print(f"Greedy algorithm: {pay_with_coins(strange, 6)} coins")
    print(f"Optimal solution: {optimal_coin_count(strange, 0, 6)} coins")

if __name__ == "__main__":
    main()
