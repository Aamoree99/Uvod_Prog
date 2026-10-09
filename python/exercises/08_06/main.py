"""
Cvičení 08 — Úloha 8.6
Název: Statistické údaje
Body: 1 bod; 2 body; 2 body; 2 body; 2 body (v pořadí podúloh)

Zadání:
Napište funkce, které ze zadaného textového souboru zjistí následující statistické údaje.

a) Funkce vracející počet různých znaků, které se v souboru vyskytují.  [1 bod]

b) Funkce vracející počet slov, které se v souboru vyskytují. Definice slova je uvedena v příkladu
6.1. [2 body]

c) Funkce vracející počet vět, které se v souboru vyskytují. Věta obsahuje alespoň jedno slovo a je
ukončena tečkou, otazníkem nebo vykřičníkem. Věta nemusí začínat velkým písmenem. Pokud první
písmeno, které následuje po tečce je malé, tato tečka neukončuje větu. Věta se může nacházet na
více řádcích. [2 body]

d) Funkce vracející nejdelší slovo, vyskytující se v souboru. V případě, že dvě slova mají shodný
počet písmen, vypíše se první z nich. [2 body]

e) Funkce vracející nejdelší větu, vyskytující se v souboru. Délka vět je měřena počtem všech
znaků, které obsahuje. V případě, že dvě věty mají shodný počet znaků, vypíše se první z nich. [2
body]


Ukázka:

int pismen = PocetRuznychZnaku("aaa.txt"); // 36

int slov = PocetSlov("aaa.txt"); // 26

int vet = PocetVet("aaa.txt"); // 5

string slovo = NejdelsiSlovo("aaa.txt"); // "ukazkovy"

string veta = NejdelsiVeta("aaa.txt");

// "Nadpis textu\\nTohle je 4. ukazkovy textovy soubor.\\n"

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
    print("This exercise is not implemented yet.")

if __name__ == "__main__":
    main()
