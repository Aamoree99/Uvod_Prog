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

package exercises.task_06_03;

public class Main {
    static void check(boolean valid) {
        if (!valid) throw new IllegalArgumentException("Invalid character array bounds");
    }
    static int compare(char[] a, char[] b) {
        int i = 0;
        while (i < a.length && i < b.length) {
            if (a[i] != b[i]) return a[i] < b[i] ? -1 : 1;
            i++;
        }
        return a.length == b.length ? 0 : (a.length < b.length ? -1 : 1);
    }
    static int indexOf(char[] a, char[] b, int start) {
        if (start < 0 || start > a.length) return -1;
        for (int i = start; i <= a.length - b.length; i++) {
            int j = 0;
            while (j < b.length && a[i + j] == b[j]) j++;
            if (j == b.length) return i;
        }
        return -1;
    }
    static char[] substring(char[] a, int start, int length) {
        check(start >= 0 && length >= 0 && start <= a.length - length && length <= 100);
        char[] result = new char[length];
        for (int i = 0; i < length; i++) result[i] = a[start + i];
        return result;
    }
    static char[] insert(char[] a, int index, char[] b) {
        check(index >= 0 && index <= a.length && a.length + b.length <= 100);
        char[] result = new char[a.length + b.length];
        for (int i = 0; i < result.length; i++)
            result[i] = i < index ? a[i] : (i < index + b.length ? b[i - index] : a[i - b.length]);
        return result;
    }
    static char[] remove(char[] a, int index, int length) {
        check(index >= 0 && length >= 0 && index <= a.length - length && a.length - length <= 100);
        char[] result = new char[a.length - length];
        for (int i = 0; i < result.length; i++) result[i] = i < index ? a[i] : a[i + length];
        return result;
    }
    static char[] replace(char[] a, char[] search, char[] replacement) {
        check(search.length > 0);
        char[] work = new char[100];
        int i = 0, position = 0;
        while (i < a.length) {
            int j = 0;
            while (j < search.length && i + j < a.length && a[i + j] == search[j]) j++;
            if (j == search.length) {
                check(position + replacement.length <= 100);
                for (int k = 0; k < replacement.length; k++) work[position++] = replacement[k];
                i += search.length;
            } else {
                check(position < 100);
                work[position++] = a[i++];
            }
        }
        return substring(work, 0, position);
    }
    static void printChars(char[] a) {
        for (int i = 0; i < a.length; i++) System.out.print(a[i]);
        System.out.println();
    }
    public static void main(String[] args) {
        char[] a = {'k', 'l', 'o', 'k', 'a', 'n'}, b = {'k', 'l', 'o', 'b', 'o', 'u', 'k'};
        char[] o = {'o'}, k = {'k'};
        System.out.println("Compare: " + compare(a, b));
        System.out.println("IndexOf: " + indexOf(b, o, 3));
        char[] c = substring(b, 2, 3);
        printChars(c);
        printChars(insert(a, 1, c));
        char[] d = remove(b, 3, 2);
        printChars(d);
        printChars(replace(d, k, c));
    }
}
