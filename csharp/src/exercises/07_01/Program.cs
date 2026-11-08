/*
Cvičení 07 — Úloha 7.1
Název: Dlouhá celá čísla
Body: 1 bod; 1 bod; 1 bod; 1 bod; 1 bod; 2 body; 3 body (v pořadí podúloh)

Zadání:
Napište funkce pracující s dlouhými celými čísly, které používají způsob uložení cifer v poli big
endian (na přednášce byl použit způsob little endian).

a) Funkci Vytvor, která převede textový řetězec na číslo. [1 bod]

b) Funkci Vypis, která vypíše číslo na obrazovku bez zbytečných nul. [1 bod]

c) Funkce Compare porovná dvě čísla. Je-li první číslo menší než druhé číslo, je vrácena hodnota
-1. Je-li první číslo větší než druhé číslo, je vrácena hodnota 1. Jsou-li si obě čísla rovna, je
vrácena hodnota 0. [1 bod]

d) Funkce Soucet sečte dvě čísla. [1 bod]

e) Funkce Rozdil odečte menší číslo od většího. [1 bod]

f) Funkce Soucin vynásobí dvě čísla. [2 body]

g) Funkce Podil vydělí číslo číslem. [3 body]


Ukázka:

int[] c1 = Vytvor("560111202232646");

int[] c2 = Vytvor("560100600690986");

Vypis(c1); // 560111202232646

int i = Compare(c1, c2); // 1

int[] c3 = Soucet(c1, c2);  // 1120211802923632

int[] c4 = Rozdil(c1, c2);  // 10601541660

int[] c5 = Soucin(c1, c4); // 5938042244702081581032360

int[] c6 = Podil(c1, c4); // 52832

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
    const int MAX_DIGITS = 100;
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

    public static void Main() {
        int[] a = createNumber("560111202232646"), b = createNumber("560100600690986");
        int[] difference = subtract(a, b, 10);
        Console.WriteLine("First: " + numberText(a));
        Console.WriteLine("Second: " + numberText(b));
        Console.WriteLine("Comparison: " + compare(a, b));
        Console.WriteLine("Sum: " + numberText(add(a, b, 10)));
        Console.WriteLine("Difference: " + numberText(difference));
        Console.WriteLine("Product: " + numberText(multiply(a, difference, 10)));
        Console.WriteLine("Quotient: " + numberText(divide(a, difference, 10)[0]));
    }
}
