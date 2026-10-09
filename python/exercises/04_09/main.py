"""
Cvičení 04 — Úloha 4.9
Název: Sportka
Body: 10 bodů

Zadání:
Napište funkci, která umožní uživateli si zahrát sportku. Výsledkem funkce bude finanční částka
vyhraná (častěji však prohraná) uživatelem. Uživatel vyplní na sázence 6 různých čísel z čísel 1 až
49 a počet slosování, na které chce čísla vsadit. Cena jednoho slosování je 20 Kč. Pokud uživatel
zadá na sázence neplatnou kombinaci čísel, je mu vrácen vklad. V každém slosování se losují dva
tahy. V každém tahu se ze 49 čísel vylosuje 6 řádných čísel a jedno číslo dodatkové. Tahy jsou
navzájem nezávislé. V každém tahu uživatel obdrží nejvyšší z výher dle tabulky. Výhry z obou tahů
se sčítají. [10 bodů]

Tabulka výher

1. pořadí: 15 382 198 Kč (uhodnuto 6 řádných čísel)

2. pořadí: 815 723 Kč (uhodnuto 5 řádných čísel a dodatkové číslo)

3. pořadí: 24 971 Kč (uhodnuto 5 řádných čísel)

4. pořadí: 619 Kč (uhodnuta 4 řádná čísla)

5. pořadí: 113 Kč (uhodnuta 3 řádná čísla)

Zdroj: http://www.sazka.cz/cz/loterie-a-hry/sportka/jak-hrat/charakteristika/


Ukázka:

int[] sazenka = { 7, 9, 22, 6, 14, 48 };

vys = Sportka(sazenka, 5000); // -74030

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


def validate_ticket(ticket):
    if len(ticket) != 6:
        return False
    i = 0
    while i < 6:
        if ticket[i] < 1 or ticket[i] > 49:
            return False
        j = i + 1
        while j < 6:
            if ticket[i] == ticket[j]:
                return False
            j += 1
        i += 1
    return True


def generate_draw():
    numbers = [0] * 7
    i = 0
    while i < 7:
        number = random.randint(1, 49)
        duplicate = False
        j = 0
        while j < i:
            if numbers[j] == number:
                duplicate = True
                break
            j += 1
        if not duplicate:
            numbers[i] = number
            i += 1
    return numbers


def count_matches(ticket, draw):
    matches = 0
    bonus = False
    i = 0
    while i < 6:
        j = 0
        while j < 6:
            if ticket[i] == draw[j]:
                matches += 1
            j += 1
        if ticket[i] == draw[6]:
            bonus = True
        i += 1
    return matches, bonus


def calculate_prize(matches, bonus):
    if matches == 6:
        return 15382198
    if matches == 5:
        return 815723 if bonus else 24971
    if matches == 4:
        return 619
    if matches == 3:
        return 113
    return 0


def sportka(ticket, draws):
    if not validate_ticket(ticket) or draws <= 0:
        return 0
    balance = -draws * 20
    i = 0
    while i < draws:
        round_index = 0
        while round_index < 2:
            matches, bonus = count_matches(ticket, generate_draw())
            balance += calculate_prize(matches, bonus)
            round_index += 1
        i += 1
    return balance


def main():
    ticket = [7, 9, 22, 6, 14, 48]
    draws = 5000
    result = sportka(ticket, draws)
    print(f"Your ticket: {ticket}\nNumber of drawings: {draws}")
    print(f"Total cost: {draws * 20} CZK\nTotal winnings: {result + draws * 20} CZK\nNet result: {result} CZK")
    print(f"Invalid ticket result: {sportka([7, 9, 22, 6, 14, 14], draws)} CZK")

if __name__ == "__main__":
    main()
