"""
Cvičení 06 — Úloha 6.4
Název: Ruleta
Body: 10 bodů

Zadání:
Napište program, který umožní uživateli si zahrát evropskou ruletu. Hra probíhá v sázkových kolech.
Uživatel může v každém sázkovém kole uzavírat libovolný počet (a vzájemné kombinace) sázek dle
tabulky sázek a výher. Uživatel v rámci sázkového kola zadá všechny své sázky formou jednoho
textového řetězce. Formát zadávání sázek navrhněte sami, můžete se inspirovat formátem navrženým
v ukázce. Neplatné části řetězce ignorujte. Následně je náhodně vybráno jedno z čísel 0 až 36.
Uživateli je oznámeno, jak dopadly jeho jednotlivé sázky, jsou mu proplaceny případné výhry.
Uživatel může hru v jakémkoliv kole ukončit, případné sázky jsou anulovány. [10 bodů]



Tabulka sázek a výher

a) jednotlivé číslo (0 až 36), výhra 35x vsazená suma

b) liché číslo (1,3,5…35) nebo sudé číslo (2,4,6, …., 36), výhra 1x vsazená suma

c) malé číslo (1 až 18) nebo vysoké číslo (19 až 36), výhra 1x vsazená suma

d) tucet: první (1 až 12), druhý (13 až 24), třetí (25 až 36), výhra 2x vsazená suma


Ukázka:

Vitejte ve hre ruleta:

Sazka: cislo 0 az 36, licha, suda, nizka, vysoka, 1. az 3. tucet. Za sazku vlozte vsazenou sumu,
sazky lze kombinovat.

Sazka? 5 20 1 25 9 30 Suda 100 vysoka 200 hotovo

Na rulete padlo cislo 24.

Sazka na sude cislo ve vysi 100 byla vyhrana. Konto +100.

Sazka na vysoke cislo ve vysi 200 byla vyhrana. Konto +200.

Sazka na cislo 1 ve vysi 25 byla prohrana. Konto -25.

Sazka na cislo 5 ve vysi 20 byla prohrana. Konto -20.

Sazka na cislo 9 ve vysi 30 byla prohrana. Konto -30.

Stav konta je 222.

Sazka? Super jedeme dal 2 tucet 100, 15 50, suda 200

Na rulete padlo cislo 8.

Sazka na sude cislo ve vysi 200 byla vyhran. Konto +200.

Sazka na 2. tucet ve vysi 100 byla prohrana. Konto -100.

Sazka na cislo 15 ve vysi 50 byla prohrana. Konto -50.

Stav konta je 275.

Sazka? Konec, uz me to staci

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

MAX_BETS = 100
MAX_INPUT = 4096
NUMBER, ODD, EVEN, LOW, HIGH, DOZEN1, DOZEN2, DOZEN3 = range(8)
MIN_BALANCE = -(2 ** 63)
MAX_BALANCE = 2 ** 63 - 1


def next_token(text, position):
    while position < len(text) and text[position] in " \t\n\r":
        position += 1
    start = position
    while position < len(text) and text[position] not in " \t\n\r":
        position += 1
    return text[start:position], position


def parse_bet_type(token):
    token = token.lower()
    names = ("odd", "even", "low", "high", "dozen1", "dozen2", "dozen3", "licha", "suda", "nizka", "vysoka")
    kinds = (ODD, EVEN, LOW, HIGH, DOZEN1, DOZEN2, DOZEN3, ODD, EVEN, LOW, HIGH)
    i = 0
    while i < len(names):
        if token == names[i]:
            return kinds[i], 0
        i += 1
    try:
        number = int(token)
        if 0 <= number <= 36:
            return NUMBER, number
    except ValueError:
        pass
    return -1, 0


def parse_bets(text):
    bets = [None] * MAX_BETS
    position = count = 0
    while position < len(text):
        token, position = next_token(text, position)
        if token == "":
            break
        if token.lower() in ("quit", "exit", "konec"):
            return bets, 0, True
        if token.lower() in ("done", "hotovo"):
            break
        kind, number = parse_bet_type(token)
        if kind == -1:
            continue
        amount_token, position = next_token(text, position)
        if amount_token.lower() in ("quit", "exit", "konec"):
            return bets, 0, True
        if amount_token == "":
            break
        try:
            amount = int(amount_token)
        except ValueError:
            continue
        if amount <= 0 or amount > MAX_BALANCE:
            continue
        if count >= MAX_BETS:
            print("Maximum number of bets reached.")
            break
        bets[count] = (kind, number, amount)
        count += 1
    return bets, count, False


def bet_wins(bet, result):
    kind, number, _ = bet
    if kind == NUMBER:
        return result == number
    if kind == ODD:
        return result != 0 and result % 2 == 1
    if kind == EVEN:
        return result != 0 and result % 2 == 0
    if kind == LOW:
        return 1 <= result <= 18
    if kind == HIGH:
        return 19 <= result <= 36
    if kind == DOZEN1:
        return 1 <= result <= 12
    if kind == DOZEN2:
        return 13 <= result <= 24
    if kind == DOZEN3:
        return 25 <= result <= 36
    return False


def payout_multiplier(kind):
    if kind == NUMBER:
        return 35
    if ODD <= kind <= HIGH:
        return 1
    if DOZEN1 <= kind <= DOZEN3:
        return 2
    return 0


def play_round(bets, count, result):
    change = 0
    names = ("Number", "Odd", "Even", "Low (1-18)", "High (19-36)", "First dozen", "Second dozen", "Third dozen")
    print(f"Roulette number: {result}")
    i = 0
    while i < count:
        kind, number, amount = bets[i]
        won = bet_wins(bets[i], result)
        profit = amount * payout_multiplier(kind) if won else -amount
        name = f"Number {number}" if kind == NUMBER else names[kind]
        print(f"Bet: {name} | Amount: {amount}", end="")
        if profit > MAX_BALANCE or not MIN_BALANCE <= change + profit <= MAX_BALANCE:
            print(" | PAYOUT OVERFLOW" if won else " | BALANCE OVERFLOW")
        else:
            change += profit
            print(f" | {'WIN' if won else 'LOSE'} | {profit:+}")
        i += 1
    return change


def main():
    print("EUROPEAN ROULETTE")
    print("Types: 0..36 (35:1), odd/even/low/high (1:1), dozen1/dozen2/dozen3 (2:1)")
    print("Format: TYPE AMOUNT [TYPE AMOUNT ...]. Commands: done, quit")
    print("Example: 5 20 1 25 9 30 even 100 high 200. Invalid bets are ignored.")
    balance = 0
    while True:
        print(f"Current balance: {balance}")
        try:
            text = input("Your bets: ")
        except EOFError:
            print("End of input.")
            break
        if len(text.encode('utf-8')) + 1 > MAX_INPUT:
            print("Input is too long. Round cancelled.")
            continue
        bets, count, quit_found = parse_bets(text)
        if quit_found:
            print("Game finished.")
            break
        if count == 0:
            print("No valid bets. Try again.")
            continue
        change = play_round(bets, count, random.randint(0, 36))
        if not MIN_BALANCE <= balance + change <= MAX_BALANCE:
            print("Balance overflow. Game finished.")
            break
        balance += change
        print(f"Round result: {change:+}")
    print(f"FINAL RESULT\nFinal balance: {balance}")

if __name__ == "__main__":
    main()
