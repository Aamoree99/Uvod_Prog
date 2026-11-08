/*
Cvičení 07 — Úloha 7.3
Název: Zlomky
Body: 1 bod; 1 bod; 2 body; 2 body; 1 bod; 1 bod (v pořadí podúloh)

Zadání:
Napište funkce pracující se zlomky, které mají čitatele i jmenovatele reprezentované dlouhým celým
číslem. Využijte funkce z příkladu 7.1. Výsledky funkcí d až f budou v základním tvaru.

a) Funkce VytvorZlomek načte zlomek z řetězce. [1 bod]

b) Funkce VypisZlomek vypíše zlomek na obrazovku. [1 bod]

c) Funkce ZakladniTvar převede zlomek na základní tvar. [2 body]

d) Funkce SoucetZlomku sečte dva zlomky. [2 body]

e) Funkce SoucinZlomku vynásobí dva zlomky [1 bod]

f) Funkce PodilZlomku vydělí zlomek zlomkem. [1 bod]


Ukázka:

Zlomek z1 = VytvorZlomek("-56890/1223366");

Zlomek z2 = VytvorZlomek("2565689841/2365");

VypisZlomek(z1); VypisZlomek(z2);

ZakladniTvar(ref z1); // -28445/611683

ZakladniTvar(ref z2); // 233244531/215

VypisZlomek(z1); VypisZlomek(z2);

Zlomek z3 = SoucetZlomku(z1, z2); // 142671708339998/131511845

Zlomek z4 = SoucinZlomku(z1, z2); // -1326928136859/26302369

Zlomek z5 = PodilZlomku(z2, z1); // -142671714455673/6115675

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

package exercises.task_07_03;

public class Main {
    static final int MAX_DIGITS = 100;
    static void check(boolean valid) {
        if (!valid) throw new IllegalArgumentException("Invalid number, base or arithmetic overflow");
    }
    // Цифры расположены от старших к младшим. Встроенная длинная арифметика не используется.
    static int[] createNumber(String text) {
        check(text.length() > 0 && text.length() <= MAX_DIGITS);
        int[] result = new int[MAX_DIGITS];
        for (int i = 0; i < text.length(); i++) {
            char c = text.charAt(i);
            check(c >= '0' && c <= '9');
            result[MAX_DIGITS - text.length() + i] = c - '0';
        }
        return result;
    }
    static String numberText(int[] a) {
        int i = 0;
        while (i + 1 < a.length && a[i] == 0) i++;
        String result = "";
        while (i < a.length) result += (char)('0' + a[i++]);
        return result;
    }
    static boolean isZero(int[] a) {
        for (int i = 0; i < a.length; i++)
            if (a[i] != 0) return false;
        return true;
    }
    static void validate(int[] a, int base) {
        check(base >= 2 && base <= 10 && a.length == MAX_DIGITS);
        for (int i = 0; i < a.length; i++) check(a[i] >= 0 && a[i] < base);
    }
    static int compare(int[] a, int[] b) {
        check(a.length == b.length);
        for (int i = 0; i < a.length; i++)
            if (a[i] != b[i]) return a[i] < b[i] ? -1 : 1;
        return 0;
    }
    static int[] add(int[] a, int[] b, int base) {
        validate(a, base);
        validate(b, base);
        int[] result = new int[MAX_DIGITS];
        int carry = 0;
        for (int i = MAX_DIGITS - 1; i >= 0; i--) {
            int sum = a[i] + b[i] + carry;
            result[i] = sum % base;
            carry = sum / base;
        }
        check(carry == 0);
        return result;
    }
    static void subtractInPlace(int[] a, int[] b, int base) {
        int borrow = 0;
        for (int i = a.length - 1; i >= 0; i--) {
            int difference = a[i] - b[i] - borrow;
            borrow = difference < 0 ? 1 : 0;
            if (difference < 0) difference += base;
            a[i] = difference;
        }
    }
    static int[] subtract(int[] a, int[] b, int base) {
        validate(a, base);
        validate(b, base);
        boolean aLarger = compare(a, b) >= 0;
        int[] result = (aLarger ? a : b).clone();
        subtractInPlace(result, aLarger ? b : a, base);
        return result;
    }
    static int[] multiply(int[] a, int[] b, int base) {
        validate(a, base);
        validate(b, base);
        int[] work = new int[MAX_DIGITS * 2];
        for (int i = MAX_DIGITS - 1; i >= 0; i--)
            for (int j = MAX_DIGITS - 1; j >= 0; j--) work[i + j + 1] += a[i] * b[j];
        for (int i = work.length - 1; i > 0; i--) {
            work[i - 1] += work[i] / base;
            work[i] %= base;
        }
        int[] result = new int[MAX_DIGITS];
        for (int i = 0; i < MAX_DIGITS; i++) {
            check(work[i] == 0);
            result[i] = work[MAX_DIGITS + i];
        }
        return result;
    }
    static int[][] divide(int[] a, int[] b, int base) {
        validate(a, base);
        validate(b, base);
        check(!isZero(b));
        int[] quotient = new int[MAX_DIGITS], remainder = new int[MAX_DIGITS + 1], divisor = new int[MAX_DIGITS + 1];
        for (int i = 0; i < MAX_DIGITS; i++) divisor[i + 1] = b[i];
        for (int i = 0; i < MAX_DIGITS; i++) {
            for (int j = 0; j < MAX_DIGITS; j++) remainder[j] = remainder[j + 1];
            remainder[MAX_DIGITS] = a[i];
            while (compare(remainder, divisor) >= 0) {
                subtractInPlace(remainder, divisor, base);
                quotient[i]++;
            }
        }
        int[] rest = new int[MAX_DIGITS];
        for (int i = 0; i < MAX_DIGITS; i++) rest[i] = remainder[i + 1];
        return new int[][] {quotient, rest};
    }

    // Только запись данных; все действия со дробями — отдельные процедуры.
    static class Fraction {
        int[] numerator, denominator;
        boolean negative;
    }
    static Fraction fraction(int[] n, int[] d, boolean negative) {
        check(!isZero(d));
        Fraction result = new Fraction();
        result.numerator = n;
        result.denominator = d;
        result.negative = negative && !isZero(n);
        return result;
    }
    static Fraction createFraction(String text) {
        check(text.length() > 0);
        int slash = 0;
        while (slash < text.length() && text.charAt(slash) != '/') slash++;
        int start = text.charAt(0) == '-' || text.charAt(0) == '+' ? 1 : 0;
        check(slash > start && slash + 1 < text.length());
        return fraction(createNumber(text.substring(start, slash)), createNumber(text.substring(slash + 1)), text.charAt(0) == '-');
    }
    static String fractionText(Fraction f) {
        return (f.negative ? "-" : "") + numberText(f.numerator) + "/" + numberText(f.denominator);
    }
    static Fraction reduce(Fraction f) {
        int[] a = f.numerator, b = f.denominator;
        while (!isZero(b)) {
            int[] rest = divide(a, b, 10)[1];
            a = b;
            b = rest;
        }
        return fraction(divide(f.numerator, a, 10)[0], divide(f.denominator, a, 10)[0], f.negative);
    }
    static Fraction addFractions(Fraction a, Fraction b) {
        int[] x = multiply(a.numerator, b.denominator, 10), y = multiply(b.numerator, a.denominator, 10);
        int[] d = multiply(a.denominator, b.denominator, 10);
        if (a.negative == b.negative) return reduce(fraction(add(x, y, 10), d, a.negative));
        return reduce(fraction(subtract(x, y, 10), d, compare(x, y) >= 0 ? a.negative : b.negative));
    }
    static Fraction multiplyFractions(Fraction a, Fraction b) {
        return reduce(fraction(multiply(a.numerator, b.numerator, 10), multiply(a.denominator, b.denominator, 10), a.negative != b.negative));
    }
    static Fraction divideFractions(Fraction a, Fraction b) {
        return reduce(fraction(multiply(a.numerator, b.denominator, 10), multiply(a.denominator, b.numerator, 10), a.negative != b.negative));
    }
    public static void main(String[] args) {
        Fraction a = createFraction("-56890/1223366"), b = createFraction("2565689841/2365");
        System.out.println("Z1: " + fractionText(a));
        System.out.println("Z2: " + fractionText(b));
        a = reduce(a);
        b = reduce(b);
        System.out.println("Reduced Z1: " + fractionText(a));
        System.out.println("Reduced Z2: " + fractionText(b));
        System.out.println("Sum: " + fractionText(addFractions(a, b)));
        System.out.println("Product: " + fractionText(multiplyFractions(a, b)));
        System.out.println("Z2 / Z1: " + fractionText(divideFractions(b, a)));
    }
}
