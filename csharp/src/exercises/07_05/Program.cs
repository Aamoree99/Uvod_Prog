/*
Cvičení 07 — Úloha 7.5
Název: Dlouhá celá čísla v obecné číselné soustavě
Body: 1/2 bodu; 1/2 bodu; 1/2 bodu; 1/2 bodu; 3 body (v pořadí podúloh)

Zadání:
Funkce z příkladu 7.1 upravte tak, aby pracovaly s čísly v zadané číselné soustavě:

a) Funkce Soucet sečte dvě čísla. [1/2 bodu]

b) Funkce Rozdil odečte menší číslo od většího. [1/2 bodu]

c) Funkce Soucin vynásobí dvě čísla. [1/2 bodu]

d) Funkce Podil vydělí číslo číslem. [1/2 bodu]

e) Napište funkci Prevod, která převede číslo používající způsob uložení cifer big endian z jedné
číselné soustavy do druhé číselné soustavy. Při praktických pokusech používejte číselné soustavy o
maximálním základu 10, snáze se v nich zobrazují výsledky. [3 body]


Ukázka:

int[] c1 = Vytvor("560111202232646");

int[] c2 = Prevod(c1, 10, 2);

// 1111111010110101100001101001111100110100101000110

int[] c3 = Prevod(c2, 2, 5); // 1041403330220232421041

int[] c4 = Soucet(c3, c3, 5); // 2133312210441020342132

int[] c5 = Rozdil(c4, c3, 5); // 1041403330220232421041

int[] c6 = Prevod(c5, 5, 7); // 225656461511451251

int[] c7 = Soucin(c6, c6, 7);

// 55402656051563425266136012062265231

int[] c8 = Podil(c7, c6, 7); // 225656461511451251

int[] c9 = Prevod(c8, 7, 10); // 560111202232646

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
    const int MAX_DIGITS = 128;
    static void check(bool valid) {
        if (!valid) throw new ArgumentException("Invalid number, radix or arithmetic overflow");
    }
    // Цифры расположены от старших к младшим. Встроенная длинная арифметика не используется.
    static int[] createNumber(string text) {
        check(text.Length > 0 && text.Length <= MAX_DIGITS);
        int[] result = new int[MAX_DIGITS];
        for (int i = 0; i < text.Length; i++) {
            char c = text[i];
            check(c >= '0' && c <= '9');
            result[MAX_DIGITS - text.Length + i] = c - '0';
        }
        return result;
    }
    static string numberText(int[] a) {
        int i = 0;
        while (i + 1 < a.Length && a[i] == 0) i++;
        string result = "";
        while (i < a.Length) result += (char)('0' + a[i++]);
        return result;
    }
    static bool isZero(int[] a) {
        for (int i = 0; i < a.Length; i++)
            if (a[i] != 0) return false;
        return true;
    }
    static void validate(int[] a, int radix) {
        check(radix >= 2 && radix <= 10 && a.Length == MAX_DIGITS);
        for (int i = 0; i < a.Length; i++) check(a[i] >= 0 && a[i] < radix);
    }
    static int compare(int[] a, int[] b) {
        check(a.Length == b.Length);
        for (int i = 0; i < a.Length; i++)
            if (a[i] != b[i]) return a[i] < b[i] ? -1 : 1;
        return 0;
    }
    static int[] add(int[] a, int[] b, int radix) {
        validate(a, radix);
        validate(b, radix);
        int[] result = new int[MAX_DIGITS];
        int carry = 0;
        for (int i = MAX_DIGITS - 1; i >= 0; i--) {
            int sum = a[i] + b[i] + carry;
            result[i] = sum % radix;
            carry = sum / radix;
        }
        check(carry == 0);
        return result;
    }
    static void subtractInPlace(int[] a, int[] b, int radix) {
        int borrow = 0;
        for (int i = a.Length - 1; i >= 0; i--) {
            int difference = a[i] - b[i] - borrow;
            borrow = difference < 0 ? 1 : 0;
            if (difference < 0) difference += radix;
            a[i] = difference;
        }
    }
    static int[] subtract(int[] a, int[] b, int radix) {
        validate(a, radix);
        validate(b, radix);
        bool aLarger = compare(a, b) >= 0;
        int[] result = (int[])(aLarger ? a : b).Clone();
        subtractInPlace(result, aLarger ? b : a, radix);
        return result;
    }
    static int[] multiply(int[] a, int[] b, int radix) {
        validate(a, radix);
        validate(b, radix);
        int[] work = new int[MAX_DIGITS * 2];
        for (int i = MAX_DIGITS - 1; i >= 0; i--)
            for (int j = MAX_DIGITS - 1; j >= 0; j--) work[i + j + 1] += a[i] * b[j];
        for (int i = work.Length - 1; i > 0; i--) {
            work[i - 1] += work[i] / radix;
            work[i] %= radix;
        }
        int[] result = new int[MAX_DIGITS];
        for (int i = 0; i < MAX_DIGITS; i++) {
            check(work[i] == 0);
            result[i] = work[MAX_DIGITS + i];
        }
        return result;
    }
    static int[][] divide(int[] a, int[] b, int radix) {
        validate(a, radix);
        validate(b, radix);
        check(!isZero(b));
        int[] quotient = new int[MAX_DIGITS], remainder = new int[MAX_DIGITS + 1], divisor = new int[MAX_DIGITS + 1];
        for (int i = 0; i < MAX_DIGITS; i++) divisor[i + 1] = b[i];
        for (int i = 0; i < MAX_DIGITS; i++) {
            for (int j = 0; j < MAX_DIGITS; j++) remainder[j] = remainder[j + 1];
            remainder[MAX_DIGITS] = a[i];
            while (compare(remainder, divisor) >= 0) {
                subtractInPlace(remainder, divisor, radix);
                quotient[i]++;
            }
        }
        int[] rest = new int[MAX_DIGITS];
        for (int i = 0; i < MAX_DIGITS; i++) rest[i] = remainder[i + 1];
        return new int[][] {quotient, rest};
    }

    // Схема Горнера сразу над цифрами целевой системы.
    static int[] convert(int[] a, int fromBase, int toBase) {
        validate(a, fromBase);
        check(toBase >= 2 && toBase <= 10);
        int[] result = new int[MAX_DIGITS];
        for (int i = 0; i < MAX_DIGITS; i++) {
            int carry = a[i];
            for (int j = MAX_DIGITS - 1; j >= 0; j--) {
                int value = result[j] * fromBase + carry;
                result[j] = value % toBase;
                carry = value / toBase;
            }
            check(carry == 0);
        }
        return result;
    }
    public static void Main() {
        int[] c1 = createNumber("560111202232646");
        int[] c2 = convert(c1, 10, 2);
        int[] c3 = convert(c2, 2, 5);
        int[] c4 = add(c3, c3, 5);
        int[] c5 = subtract(c4, c3, 5);
        int[] c6 = convert(c5, 5, 7);
        int[] c7 = multiply(c6, c6, 7);
        int[][] division = divide(c7, c6, 7);
        int[] c8 = division[0], c9 = convert(c8, 7, 10);
        int[][] numbers = {c1, c2, c3, c4, c5, c6, c7, c8, c9};
        int[] bases = {10, 2, 5, 5, 5, 7, 7, 7, 10};
        for (int i = 0; i < numbers.Length; i++)
            Console.WriteLine("C" + (i + 1) + " (radix " + bases[i] + "): " + numberText(numbers[i]));
        check(compare(c1, c9) == 0 && isZero(division[1]));
        Console.WriteLine("ALL CHECKS PASSED");
    }
}
