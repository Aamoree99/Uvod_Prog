/*
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
*/

package exercises.task_06_04;

public class Main {
    static boolean wins(int kind, int number, int result) {
        if (kind == 0) return result == number;
        if (result == 0) return false;
        if (kind == 1) return result % 2 == 1;
        if (kind == 2) return result % 2 == 0;
        if (kind == 3) return result <= 18;
        if (kind == 4) return result >= 19;
        return result >= (kind - 5) * 12 + 1 && result <= (kind - 4) * 12;
    }
    static int multiplier(int kind) { return kind == 0 ? 35 : (kind < 5 ? 1 : 2); }
    static int parseKind(String token) {
        String[] names = {"odd", "even", "low", "high", "dozen1", "dozen2", "dozen3", "licha", "suda", "nizka", "vysoka"};
        int[] kinds = {1, 2, 3, 4, 5, 6, 7, 1, 2, 3, 4};
        for (int i = 0; i < names.length; i++)
            if (token.equals(names[i])) return kinds[i];
        return -1;
    }
    static boolean quit(String token) { return token.equals("quit") || token.equals("exit") || token.equals("konec"); }
    // Токенизация по индексам; не используем коллекции и foreach.
    static int skipSpace(String text, int p) {
        while (p < text.length() && text.charAt(p) <= ' ') p++;
        return p;
    }
    static int tokenEnd(String text, int p) {
        while (p < text.length() && text.charAt(p) > ' ') p++;
        return p;
    }
    static long parseAmount(String token) {
        if (token.length() == 0) return -1;
        long value = 0;
        for (int i = 0; i < token.length(); i++) {
            int digit = token.charAt(i) - '0';
            if (digit < 0 || digit > 9 || value > (Long.MAX_VALUE - digit) / 10) return -1;
            value = value * 10 + digit;
        }
        return value;
    }
    static boolean canAdd(long a, long b) {
        return !(b > 0 && a > Long.MAX_VALUE - b || b < 0 && a < Long.MIN_VALUE - b);
    }
    public static void main(String[] args) throws Exception {
        java.io.BufferedReader input = new java.io.BufferedReader(new java.io.InputStreamReader(System.in));
        java.util.Random random = new java.util.Random();
        long balance = 0;
        System.out.println("EUROPEAN ROULETTE\nTYPE AMOUNT [TYPE AMOUNT ...], done, quit");
        System.out.println("Types: 0..36, odd/even/low/high, dozen1/dozen2/dozen3 (licha/suda/nizka/vysoka)");
        boolean finished = false;
        while (!finished) {
            System.out.println("Current balance: " + balance);
            String text = input.readLine();
            if (text == null) break;
            if (text.length() > 4096) { System.out.println("Input too long"); continue; }
            int[] kinds = new int[100], numbers = new int[100];
            long[] amounts = new long[100];
            int count = 0, p = 0;
            while (p < text.length()) {
                p = skipSpace(text, p);
                int end = tokenEnd(text, p);
                if (p == end) break;
                String token = text.substring(p, end).toLowerCase(java.util.Locale.ROOT);
                p = end;
                if (quit(token)) { finished = true; break; }
                if (token.equals("done") || token.equals("hotovo")) break;
                int kind = parseKind(token), number = 0;
                if (kind < 0) {
                    long parsed = parseAmount(token);
                    if (parsed < 0 || parsed > 36) continue;
                    kind = 0;
                    number = (int)parsed;
                }
                p = skipSpace(text, p);
                end = tokenEnd(text, p);
                token = text.substring(p, end).toLowerCase(java.util.Locale.ROOT);
                p = end;
                if (quit(token)) { finished = true; break; }
                long amount = parseAmount(token);
                if (amount <= 0) continue;
                if (count == 100) { System.out.println("Maximum 100 bets"); break; }
                kinds[count] = kind;
                numbers[count] = number;
                amounts[count++] = amount;
            }
            if (finished) break;
            if (count == 0) { System.out.println("No valid bets"); continue; }
            int result = random.nextInt(37);
            System.out.println("Roulette number: " + result);
            long change = 0;
            for (int i = 0; i < count; i++) {
                boolean won = wins(kinds[i], numbers[i], result);
                int factor = multiplier(kinds[i]);
                if (won && amounts[i] > Long.MAX_VALUE / factor) { System.out.println("PAYOUT OVERFLOW"); continue; }
                long profit = won ? amounts[i] * factor : -amounts[i];
                if (!canAdd(change, profit)) { System.out.println("BALANCE OVERFLOW"); continue; }
                change += profit;
                System.out.println("Kind: " + kinds[i] + " Number: " + numbers[i] + " Amount: " + amounts[i] + " | " + (won ? "WIN " : "LOSE ") + profit);
            }
            if (!canAdd(balance, change)) { System.out.println("BALANCE OVERFLOW"); break; }
            balance += change;
            System.out.println("Round result: " + change);
        }
        System.out.println("Final balance: " + balance);
    }
}
