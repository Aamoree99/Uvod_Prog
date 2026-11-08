/*
Cvičení 06 — Úloha 6.1
Název: Práce s textovými řetězci
Body: 2 body; 2 body; 2 body; 1 bod; 1 bod; 4 body (v pořadí podúloh)

Zadání:
Slovo je řetězec složený výhradně z písmen, který je maximální délky, nejde tedy prodloužit o další
písmeno. Dvě slova jsou od sebe oddělena jedním nebo více znaky, které nejsou písmeny. Pokud se
před slovem či za slovem nachází znak, tak tento znak není písmenem. Napište následující funkce pro
práci s textovými řetězci bez použití funkcí Split a Distinct. Původní řetězec nesmíte modifikovat,
ani od něj vytvářet kopie:

a) Funkce vracející počet různých písmen anglické abecedy, které zadaný řetězec obsahuje. Velikost
písmen se nerozlišuje. [2 body]

b) Funkce vracející počet slov v zadaném řetězci. [2 body]

c) Funkce vracející slovo první výskyt slova na zadané pozice v zadaném řetězci. Pokud se na zadané
pozici nachází písmeno, je vráceno slovo, jehož součástí toto písmeno je. V tomto případě bude
nalezené slovo často začínat před zadanou pozicí. Pokud se na zadané pozici nenachází písmeno, je
vráceno první slovo nacházející se napravo od zadané pozice v zadaném řetězci. Pokud není nalezeno
žádné slovo, je vrácen prázdný řetězec. [2 body]

d) Funkce vracející nejdelší slovo v zadaném řetězci. Pokud jsou dvě slova stejně dlouhá, je
vráceno slovo, které se v řetězci vyskytuje jako první. Pokud není nalezeno žádné slovo, je vrácen
prázdný řetězec. [1 bod]

e) Funkce vracející lexikograficky (abecedně) největší slovo v zadaném řetězci. Pokud není nalezeno
žádné slovo, je vrácen prázdný řetězec. [1 bod]

f) Funkce vracející nejčastěji se vyskytující slovo v zadaném řetězci. Pokud se vyskytují dvě slova
stejně často, je vráceno slovo, které se v řetězci vyskytuje jako první. Pokud není nalezeno žádné
slovo, je vrácen prázdný řetězec. Dejte pozor na situace, kdy jedno slovo je podslovem jiného
slova. V řetězci "Ja a Jana" má slovo "a" pouze jeden výskyt. [4 body]


Ukázka:

string s1 = "Alena ma 2 ruce.";

int ruznych = RuznychPismen(s1); // 8

int pocet = PocetSlov(s1); // 3

string s2 = PrvniSlovo(s1, 2); // "Alena"

string s3 = PrvniSlovo(s1, s1.Length - 1); // ""

string s4 = NejdelsiSlovo(s1); // "Alena"

string s5 = NejvetsiSlovo(s1); // "ruce"

string s6 = "Mama ma 2 ruce a v ruce hul";

string s7 = NejcastejsiSlovo(s1 + s6); // "ruce"

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

package exercises.task_06_01;

public class Main {
    static boolean isLetter(char c) { return c >= 'A' && c <= 'Z' || c >= 'a' && c <= 'z'; }
    static char lower(char c) { return c >= 'A' && c <= 'Z' ? (char)(c + 32) : c; }
    static int wordStart(String s, int p) {
        while (p < s.length() && !isLetter(s.charAt(p))) p++;
        return p;
    }
    static int wordEnd(String s, int p) {
        while (p < s.length() && isLetter(s.charAt(p))) p++;
        return p;
    }
    // Сравниваем диапазоны исходного текста без копирования слов.
    static int compareRanges(String s, int a, int ae, int b, int be) {
        while (a < ae && b < be) {
            char x = lower(s.charAt(a++)), y = lower(s.charAt(b++));
            if (x != y) return x < y ? -1 : 1;
        }
        return (ae - a) - (be - b);
    }
    static int distinctLetters(String s) {
        int count = 0;
        for (int i = 0; i < s.length(); i++)
            if (isLetter(s.charAt(i))) {
                int j = 0;
                while (j < i && lower(s.charAt(j)) != lower(s.charAt(i))) j++;
                if (j == i) count++;
            }
        return count;
    }
    static int countWords(String s) {
        int count = 0, p = 0;
        while (p < s.length()) {
            p = wordStart(s, p);
            if (p < s.length()) count++;
            p = wordEnd(s, p);
        }
        return count;
    }
    static String firstWord(String s, int p) {
        if (p < 0 || p >= s.length()) return "";
        while (p > 0 && isLetter(s.charAt(p)) && isLetter(s.charAt(p - 1))) p--;
        int start = wordStart(s, p);
        return s.substring(start, wordEnd(s, start));
    }
    // mode: 0 — самое длинное, 1 — наибольшее, 2 — самое частое.
    static String bestWord(String s, int mode) {
        int best = 0, bestEnd = 0, maximum = 0, p = 0;
        while (p < s.length()) {
            int start = wordStart(s, p), end = wordEnd(s, start);
            p = end;
            if (start == end) break;
            int count = 0;
            if (mode == 2) {
                int q = 0;
                while (q < s.length()) {
                    int a = wordStart(s, q), b = wordEnd(s, a);
                    q = b;
                    if (compareRanges(s, start, end, a, b) == 0) count++;
                }
            }
            if (mode == 0 && end - start > bestEnd - best ||
                mode == 1 && compareRanges(s, start, end, best, bestEnd) > 0 ||
                mode == 2 && count > maximum) {
                best = start;
                bestEnd = end;
                maximum = count;
            }
        }
        return s.substring(best, bestEnd);
    }
    public static void main(String[] args) {
        String s = "Alena ma 2 ruce.";
        System.out.println("Distinct letters: " + distinctLetters(s));
        System.out.println("Words: " + countWords(s));
        System.out.println("Word at 2: " + firstWord(s, 2));
        System.out.println("Last position: " + firstWord(s, s.length() - 1));
        System.out.println("Longest: " + bestWord(s, 0));
        System.out.println("Greatest: " + bestWord(s, 1));
        System.out.println("Most frequent: " + bestWord(s + "Mama ma 2 ruce a v ruce hul", 2));
    }
}
