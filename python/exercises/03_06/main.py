"""
Cvičení 03 — Úloha 3.6
Název: Kámen, nůžky papír
Body: 5 bodů

Zadání:
Napište program, který si s uživatelem zahraje hru kámen, nůžky, papír. Hraje se na dvě vítězná
kola. Použijte výčtový typ. V každém kole si  uživatel zvolí jeden z výše uvedených
předmětů předmětů, počítač svůj předmět vylosuje náhodně. Kámen poráží nůžky, nůžky poráží papír,
papír poráží kámen. Pokud si uživatel i počítač zvolili stejný předmět, dané kolo se opakuje do té
doby, dokud nebudou zvoleny různé předměty. [5 bodů]


Ukázka:

Vitejte ve hre kamen – nuzky – papir

Hrajeme na dve vitezna kola

Zadejte svuj tah: 6

Tento tah je neplatny

Zadejte svuj tah: kamen

Hrac zahral Kamen, pocitac zahral Kamen

Toto kolo je nerozhodne

Celkove skore: hrac 0 – pocitac 0

Zadejte svuj tah: papir

Hrac zahral Papir, pocitac zahral Nuzky

Toto kolo vyhral pocitac

Celkove skore: hrac 0 – pocitac 1

Zadejte svuj tah: papir

Hrac zahral Papir, pocitac zahral Kamen

Toto kolo vyhral hrac

Celkove skore: hrac 1 – pocitac 1

Zadejte svuj tah: nuzky

Hrac zahral Nuzky, pocitac zahral Papir

Toto kolo vyhral hrac

Celkove skore: hrac 2 – pocitac 1

Celou hru vyhral hrac

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


def round_result(player, computer):
    if player == computer:
        return "Draw"
    if (player == 0 and computer == 1) or (player == 1 and computer == 2) or (player == 2 and computer == 0):
        return "Player"
    return "Computer"


def read_player_move():
    text = input("Enter your move [rock / scissors / paper]: ").strip().lower()
    if text == "rock" or text == "r":
        return 0
    if text == "scissors" or text == "s":
        return 1
    if text == "paper" or text == "p":
        return 2
    return -1


def play_game():
    names = ("Rock", "Scissors", "Paper")
    player_score = computer_score = 0
    print("The first to win two rounds wins the game.")
    while player_score < 2 and computer_score < 2:
        player = read_player_move()
        if player == -1:
            print("Invalid move.")
            continue
        computer = random.randrange(3)
        print(f"Player: {names[player]} | Computer: {names[computer]}")
        result = round_result(player, computer)
        print(f"Round result: {result}")
        if result == "Player":
            player_score += 1
        elif result == "Computer":
            computer_score += 1
        print(f"PLAYER {player_score} : {computer_score} COMPUTER")
    print("THE PLAYER WINS THE GAME!" if player_score == 2 else "THE COMPUTER WINS THE GAME!")


def main():
    print("ROCK - SCISSORS - PAPER")
    try:
        while True:
            play_game()
            answer = input("Play again? [y/n]: ").strip().lower()
            if answer != "y" and answer != "yes":
                break
    except EOFError:
        print("End of input.")
    print("Thanks for playing.")

if __name__ == "__main__":
    main()
