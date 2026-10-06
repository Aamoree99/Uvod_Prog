/*
Cvičení 03 — Úloha 3.6
Název: Kámen, nůžky papír
Body: 5 bodů

Zadání:
Napište program, který si s uživatelem zahraje hru kámen, nůžky, papír. Hraje se na dvě vítězná
kola. Použijte výčtový typ. V každém kole si  uživatel zvolí jeden z výše uvedených
předmětů předmětů, počítač svůj předmět vylosuje náhodně. Kámen poráží nůžky, nůžky poráží papír,
papír poráží kámen. Pokud si uživatel i počítač zvolili stejný předmět, dané kolo se opakuje do té
doby, dokud nebudou zvoleny různé předměty. [5 bodů]

Ukázka:

Vitejte ve hre kamen – nuzky – papir

Hrajeme na dve vitezna kola

Zadejte svuj tah: 6

Tento tah je neplatny

Zadejte svuj tah: kamen

Hrac zahral Kamen, pocitac zahral Kamen

Toto kolo je nerozhodne

Celkove skore: hrac 0 – pocitac 0

Zadejte svuj tah: papir

Hrac zahral Papir, pocitac zahral Nuzky

Toto kolo vyhral pocitac

Celkove skore: hrac 0 – pocitac 1

Zadejte svuj tah: papir

Hrac zahral Papir, pocitac zahral Kamen

Toto kolo vyhral hrac

Celkove skore: hrac 1 – pocitac 1

Zadejte svuj tah: nuzky

Hrac zahral Nuzky, pocitac zahral Papir

Toto kolo vyhral hrac

Celkove skore: hrac 2 – pocitac 1

Celou hru vyhral hrac

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

package exercises.task_03_06;

import java.util.*;
import java.math.BigInteger;

// Каждый файл Main.java компилируется отдельно в свой каталог.
public class Main {
    private static final Scanner INPUT = new Scanner(System.in);
    // Возможные ходы и результаты раунда представлены перечислениями.
    enum Tah { Kamen, Nuzky, Papir }
    enum Vysledek { Nerozhodne, Hrac, Pocitac }
    private static final Random RNG=new Random();
    static int random_move() { return RNG.nextInt(3); }
    static String read_line() { return INPUT.nextLine().trim().toLowerCase(Locale.ROOT); }
    static void pause(int ms) { try { Thread.sleep(ms); } catch (InterruptedException e) { Thread.currentThread().interrupt(); throw new RuntimeException(e); } }
    // ANSI-последовательности очищают экран и перемещают курсор в левый верхний угол.
    static void vycisti_obrazovku() { System.out.print("\033[2J\033[1;1H"); System.out.flush(); }
    static String tah_na_text(Tah tah) {
        if (tah==Tah.Kamen) return "Kamen";
        if (tah==Tah.Nuzky) return "Nuzky";
        return "Papir";
    }
    static String tah_emoji(Tah tah) {
        if (tah==Tah.Kamen) return "🪨";
        if (tah==Tah.Nuzky) return "✂️";
        return "📄";
    }
    // Многострочные рисунки соответствуют исходной Rust-версии.
    static String tah_ascii(Tah tah) {
        if (tah==Tah.Kamen) return "\n    _______\n---'   ____)\n      (_____)\n      (_____)\n      (____)\n---.__(___)\n";
        if (tah==Tah.Nuzky) return "\n    _______\n---'   ____)____\n          ______)\n       __________)\n      (____)\n---.__(___)\n";
        return "\n    _______\n---'   ____)____\n          ______)\n          _______)\n         _______)\n---.__________)\n";
    }
    // Три кадра с той же паузой 600 мс, что в Rust.
    static void animace() {
        vycisti_obrazovku(); System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "             KAMEN!%n"); System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "%s%n", tah_ascii(Tah.Kamen)); pause(600);
        vycisti_obrazovku(); System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "             NUZKY!%n"); System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "%s%n", tah_ascii(Tah.Nuzky)); pause(600);
        vycisti_obrazovku(); System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "             PAPIR!%n"); System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "%s%n", tah_ascii(Tah.Papir)); pause(600);
    }
    // Неизвестное слово даёт отсутствующий ход; вызывающая функция повторяет запрос.
    static Tah nacti_tah_hrace() {
        System.out.printf(Locale.ROOT, "Zadejte svuj tah [kamen / nuzky / papir]: "); System.out.flush();
        String vstup=read_line();
        if (vstup.equals("kamen") || vstup.equals("k")) return Tah.Kamen;
        if (vstup.equals("nuzky") || vstup.equals("n")) return Tah.Nuzky;
        if (vstup.equals("papir") || vstup.equals("p")) return Tah.Papir;
        return null;
    }
    // Случайное целое 0, 1 или 2 определяет ход компьютера.
    static Tah tah_pocitace() {
        int cislo=random_move();
        if (cislo==0) return Tah.Kamen;
        if (cislo==1) return Tah.Nuzky;
        return Tah.Papir;
    }
    static Vysledek vysledek_kola(Tah hrac,Tah pocitac) {
        if (hrac==pocitac) return Vysledek.Nerozhodne;
        if ((hrac==Tah.Kamen && pocitac==Tah.Nuzky) || (hrac==Tah.Nuzky && pocitac==Tah.Papir) || (hrac==Tah.Papir && pocitac==Tah.Kamen)) return Vysledek.Hrac;
        return Vysledek.Pocitac;
    }
    static void zobraz_skore(int skore_hrac,int skore_pocitac) {
        System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "========================================%n");
        System.out.printf(Locale.ROOT, "        HRAC %s : %s POCITAC%n", skore_hrac, skore_pocitac);
        System.out.printf(Locale.ROOT, "========================================%n"); System.out.printf(Locale.ROOT, "%n");
    }
    static void zobraz_tahy(Tah hrac,Tah pocitac) {
        System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "HRAC                         POCITAC%n");
        System.out.printf(Locale.ROOT, "%s %s                     %s %s%n", tah_emoji(hrac), tah_na_text(hrac), tah_na_text(pocitac), tah_emoji(pocitac));
        System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "HRAC:%n"); System.out.printf(Locale.ROOT, "%s%n", tah_ascii(hrac)); System.out.printf(Locale.ROOT, "POCITAC:%n"); System.out.printf(Locale.ROOT, "%s%n", tah_ascii(pocitac));
    }
    static void zobraz_uvod() {
        vycisti_obrazovku(); System.out.printf(Locale.ROOT, "========================================%n"); System.out.printf(Locale.ROOT, "         KAMEN - NUZKY - PAPIR%n");
        System.out.printf(Locale.ROOT, "========================================%n"); System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "Hrajeme na dve vitezna kola.%n"); System.out.printf(Locale.ROOT, "%n");
        System.out.printf(Locale.ROOT, "Muzete zadat:%n"); System.out.printf(Locale.ROOT, "  kamen / k%n"); System.out.printf(Locale.ROOT, "  nuzky / n%n"); System.out.printf(Locale.ROOT, "  papir / p%n"); System.out.printf(Locale.ROOT, "%n");
    }
    // Ничья не меняет счёт. Игра заканчивается после двух побед одной стороны.
    static void hraj_hru() {
        int skore_hrac=0,skore_pocitac=0;
        zobraz_uvod();
        while (skore_hrac < 2 && skore_pocitac < 2) {
            Tah hrac;
            while (true) {
                Tah tah=nacti_tah_hrace();
                if (tah != null) { hrac=tah; break; }
                System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "Tento tah je neplatny.%n"); System.out.printf(Locale.ROOT, "%n");
            }
            Tah pocitac=tah_pocitace();
            animace(); vycisti_obrazovku(); zobraz_tahy(hrac,pocitac);
            Vysledek vysledek=vysledek_kola(hrac,pocitac);
            if (vysledek==Vysledek.Nerozhodne) System.out.printf(Locale.ROOT, "Toto kolo je nerozhodne.%n");
            else if (vysledek==Vysledek.Hrac) { System.out.printf(Locale.ROOT, "Toto kolo vyhral hrac.%n"); skore_hrac += 1; }
            else { System.out.printf(Locale.ROOT, "Toto kolo vyhral pocitac.%n"); skore_pocitac += 1; }
            zobraz_skore(skore_hrac,skore_pocitac);
            if (skore_hrac < 2 && skore_pocitac < 2) pause(1200);
        }
        System.out.printf(Locale.ROOT, "%n");
        if (skore_hrac==2) System.out.printf(Locale.ROOT, "CELKOVYM VITEZEM JE HRAC!%n");
        else System.out.printf(Locale.ROOT, "CELKOVYM VITEZEM JE POCITAC!%n");
        System.out.printf(Locale.ROOT, "%n");
    }
    static boolean chce_hrat_znovu() {
        System.out.printf(Locale.ROOT, "Hrat znovu? [a/n]: "); System.out.flush();
        String vstup=read_line();
        return vstup.equals("a") || vstup.equals("ano");
    }
    public static void main(String[] args) {
        while (true) {
            hraj_hru();
            if (!chce_hrat_znovu()) break;
        }
        System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "Diky za hru.%n");
    }
}
