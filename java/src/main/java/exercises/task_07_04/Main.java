/*
Cvičení 07 — Úloha 7.4
Název: Římská čísla
Body: 1 bod; 1 bod (v pořadí podúloh)

Zadání:
Římská čísla jsou příkladem nepoziční číselné soustavy. Hodnoty číslic nezávisí na jejich pozici
v zápise čísla. Hodnoty číslic jsou následující I = 1, V = 5, X = 10, L = 50, C = 100, D = 500, M =
1000. Existují i dvojice číslic, které se považují za jednu číslici: IV  = 4, IX = 9, XL = 40, XC =
90, CD 400, CM 900. Číslice jsou v zápisu čísla uspořádány od nejvyšší hodnoty po nejnižší.
Například číslo 2016 se zapíše jako MMXVI. Například číslo 1949 se zapíše zkráceným zápisem jako
MCMXLIX. Například výraz IM, který měl mít hodnotu 999, odporuje výše uvedeným pravidlům a není
povolen. Napište následující funkce:

a) Funkce DecRoman převede číslo v desítkové soustavě na římské číslo. [1 bod]

b) Funkce RomanDec převede římské číslo na číslo v desítkové soustavě [1 bod].


Ukázka:

string s = DecRoman(1969); // "MCMLXIX"

int i = RomanDec("MMCDXXXVII"); // 2437


Správnost programu si můžete ověřit na online konvertoru:

http://www.convertit.com/Go/ConvertIt/Calculators/Math/Roman_Numerals_Converter.ASP

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

package exercises.task_07_04;

public class Main {
    static String decimalToRoman(int number) {
        if (number < 1 || number > 3999) throw new IllegalArgumentException("Roman range is 1..3999");
        int[] values = {1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1};
        String[] symbols = {"M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"};
        String result = "";
        for (int i = 0; i < values.length; i++)
            while (number >= values[i]) {
                result += symbols[i];
                number -= values[i];
            }
        return result;
    }
    static int romanValue(char c) {
        String symbols = "IVXLCDM";
        int[] values = {1, 5, 10, 50, 100, 500, 1000};
        for (int i = 0; i < symbols.length(); i++)
            if (symbols.charAt(i) == c) return values[i];
        return 0;
    }
    static int romanToDecimal(String text) {
        if (text.length() == 0 || text.length() > 15) return -1;
        int result = 0;
        for (int i = 0; i < text.length(); i++) {
            int value = romanValue(text.charAt(i));
            if (value == 0) return -1;
            result += i + 1 < text.length() && value < romanValue(text.charAt(i + 1)) ? -value : value;
        }
        if (result < 1 || result > 3999 || !decimalToRoman(result).equals(text)) return -1;
        return result;
    }
    public static void main(String[] args) {
        System.out.println("1969: " + decimalToRoman(1969));
        System.out.println("MMCDXXXVII: " + romanToDecimal("MMCDXXXVII"));
        System.out.println("Invalid IM: " + romanToDecimal("IM"));
        for (int i = 1; i <= 3999; i++)
            if (romanToDecimal(decimalToRoman(i)) != i) throw new IllegalStateException("Roman round trip failed");
        System.out.println("All numbers 1..3999 passed");
    }
}
