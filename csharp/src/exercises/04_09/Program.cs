/*
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
*/

using System;

internal static class Program
{
    static Random random = new Random();
    static bool validateTicket(int[] ticket) {
        if (ticket.Length != 6) return false;
        for (int i = 0; i < 6; i++) {
            if (ticket[i] < 1 || ticket[i] > 49) return false;
            for (int j = 0; j < i; j++)
                if (ticket[i] == ticket[j]) return false;
        }
        return true;
    }
    static int[] generateDraw() {
        int[] draw = new int[7];
        int i = 0;
        while (i < 7) {
            int number = random.Next(49) + 1;
            bool duplicate = false;
            for (int j = 0; j < i; j++)
                if (draw[j] == number) duplicate = true;
            if (!duplicate) draw[i++] = number;
        }
        return draw;
    }
    static long calculatePrize(int matches, bool bonus) {
        if (matches == 6) return 15382198;
        if (matches == 5) return bonus ? 815723 : 24971;
        if (matches == 4) return 619;
        if (matches == 3) return 113;
        return 0;
    }
    static long sportka(int[] ticket, int draws) {
        if (!validateTicket(ticket) || draws <= 0) return 0;
        long balance = -(long)draws * 20;
        for (int i = 0; i < draws; i++)
            for (int round = 0; round < 2; round++) {
                int[] draw = generateDraw();
                int matches = 0;
                bool bonus = false;
                for (int j = 0; j < 6; j++) {
                    for (int k = 0; k < 6; k++)
                        if (ticket[j] == draw[k]) matches++;
                    if (ticket[j] == draw[6]) bonus = true;
                }
                balance += calculatePrize(matches, bonus);
            }
        return balance;
    }
    public static void Main() {
        int[] ticket = {7, 9, 22, 6, 14, 48};
        int draws = 5000;
        long result = sportka(ticket, draws);
        Console.WriteLine("Total cost: " + draws * 20 + " CZK");
        Console.WriteLine("Total winnings: " + (result + draws * 20) + " CZK");
        Console.WriteLine("Net result: " + result + " CZK");
        int[] invalid = {7, 9, 22, 6, 14, 14};
        Console.WriteLine("Invalid ticket: " + sportka(invalid, draws));
    }
}
