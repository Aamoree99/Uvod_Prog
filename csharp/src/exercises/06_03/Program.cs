/*
Cvičení 06 — Úloha 6.3
Název: Naše vlastní řetězce
Body: 2 body; 2 body; 1 bod; 2 body; 2 body; 4 body (v pořadí podúloh)

Zadání:
Napište funkce pracující s polem znaků char[], které budou mít stejnou funkčnost jako funkce pro
textové řetězce string. (Aniž byste pole znaků char[], na textový řetězec string převedli).

a) Funkce lexikografického porovnání řetězců s1 a s2: Compare(s1, s2). [2 body]

b) Funkce vracející index prvního výskytu hledaného řetězce s2 v řetězci s1. Řetězec s1 se
prohledává od zadané pozice start: IndexOf(s1, s2, start). [2 body]

c) Funkce vracející podřetězec zadaného řetězce s1. Podřetězec začíná na zadané pozici a má zadanou
délku: Substring(s1, start, delka). [1 bod]

d) Funkce vracející řetězec vzniklý vložením řetězce s2 do řetězce s1 na pozici zadanou indexem:
Insert(s1, index, s2). [2 body]

e) Funkce vracející řetězec vzniklý odstraněním podřetězce ze zadaného řetězce s1. Z řetězce s1 se
odstraní podřetězec určený indexem (v rámci řetězce s1) svého prvního znaku a svojí délkou:
Remove(s1, index, delka). [2 body]

f) Funkce vracející řetězec vzniklý z řetězce s1 nahrazením všech výskytů řetězce s2 za řetězec s3:
Replace (s1, s2, s3). [4 body]


Ukázka:

char[] s1 = "klokan".ToCharArray();

char[] s2 = "klobouk".ToCharArray();

int i1 = Compare(s1, s2); // 1

int i2 = IndexOf(s2, "o".ToCharArray(), 3); // 4

char[] s3 = Substring(s2, 2, 3); // 'o''b''o'

char[] s4 = Insert(s1, 1, s3); // 'k''o''b''o''l''o''k''a''n'

char[] s5 = Remove(s2, 3, 2); // 'k''l''o''u''k''

char[] s6 = Replace(s5, "k".ToCharArray(), s3);

// 'o''b''o''l''o''u''o''b''o'

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
    static void check(bool valid) {
        if (!valid) throw new ArgumentException("Invalid character array bounds");
    }
    static int compare(char[] a, char[] b) {
        int i = 0;
        while (i < a.Length && i < b.Length) {
            if (a[i] != b[i]) return a[i] < b[i] ? -1 : 1;
            i++;
        }
        return a.Length == b.Length ? 0 : (a.Length < b.Length ? -1 : 1);
    }
    static int indexOf(char[] a, char[] b, int start) {
        if (start < 0 || start > a.Length) return -1;
        for (int i = start; i <= a.Length - b.Length; i++) {
            int j = 0;
            while (j < b.Length && a[i + j] == b[j]) j++;
            if (j == b.Length) return i;
        }
        return -1;
    }
    static char[] substring(char[] a, int start, int length) {
        check(start >= 0 && length >= 0 && start <= a.Length - length && length <= 100);
        char[] result = new char[length];
        for (int i = 0; i < length; i++) result[i] = a[start + i];
        return result;
    }
    static char[] insert(char[] a, int index, char[] b) {
        check(index >= 0 && index <= a.Length && a.Length + b.Length <= 100);
        char[] result = new char[a.Length + b.Length];
        for (int i = 0; i < result.Length; i++)
            result[i] = i < index ? a[i] : (i < index + b.Length ? b[i - index] : a[i - b.Length]);
        return result;
    }
    static char[] remove(char[] a, int index, int length) {
        check(index >= 0 && length >= 0 && index <= a.Length - length && a.Length - length <= 100);
        char[] result = new char[a.Length - length];
        for (int i = 0; i < result.Length; i++) result[i] = i < index ? a[i] : a[i + length];
        return result;
    }
    static char[] replace(char[] a, char[] search, char[] replacement) {
        check(search.Length > 0);
        char[] work = new char[100];
        int i = 0, position = 0;
        while (i < a.Length) {
            int j = 0;
            while (j < search.Length && i + j < a.Length && a[i + j] == search[j]) j++;
            if (j == search.Length) {
                check(position + replacement.Length <= 100);
                for (int k = 0; k < replacement.Length; k++) work[position++] = replacement[k];
                i += search.Length;
            } else {
                check(position < 100);
                work[position++] = a[i++];
            }
        }
        return substring(work, 0, position);
    }
    static void printChars(char[] a) {
        for (int i = 0; i < a.Length; i++) Console.Write(a[i]);
        Console.WriteLine();
    }
    public static void Main() {
        char[] a = {'k', 'l', 'o', 'k', 'a', 'n'}, b = {'k', 'l', 'o', 'b', 'o', 'u', 'k'};
        char[] o = {'o'}, k = {'k'};
        Console.WriteLine("Compare: " + compare(a, b));
        Console.WriteLine("IndexOf: " + indexOf(b, o, 3));
        char[] c = substring(b, 2, 3);
        printChars(c);
        printChars(insert(a, 1, c));
        char[] d = remove(b, 3, 2);
        printChars(d);
        printChars(replace(d, k, c));
    }
}
