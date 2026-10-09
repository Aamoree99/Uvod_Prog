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
    enum Move { Rock, Scissors, Paper }
    enum RoundResult { Draw, Player, Computer }
    private static final Random RNG=new Random();
    // Возвращаем случайное число от 0 до 2 для выбора хода.
    static int randomMove() { return RNG.nextInt(3); }
    // Читаем строку и убираем окружающие пробелы.
    static String readLine() { return INPUT.nextLine().trim().toLowerCase(Locale.ROOT); }
    // Приостанавливаем выполнение на заданное число миллисекунд.
    static void pause(int ms) { try { Thread.sleep(ms); } catch (InterruptedException e) { Thread.currentThread().interrupt(); throw new RuntimeException(e); } }
    // ANSI-последовательности очищают экран и перемещают курсор в левый верхний угол.
    static void clearScreen() { System.out.print("\033[2J\033[1;1H"); System.out.flush(); }
    // Возвращаем английское название выбранного хода.
    static String moveText(Move move) {
        if (move==Move.Rock) return "Rock";
        if (move==Move.Scissors) return "Scissors";
        return "Paper";
    }
    // Возвращаем символ для выбранного хода.
    static String moveEmoji(Move move) {
        if (move==Move.Rock) return "🪨";
        if (move==Move.Scissors) return "✂️";
        return "📄";
    }
    // Многострочные рисунки соответствуют исходной Rust-версии.
    static String moveAscii(Move move) {
        if (move==Move.Rock) return "\n    _______\n---'   ____)\n      (_____)\n      (_____)\n      (____)\n---.__(___)\n";
        if (move==Move.Scissors) return "\n    _______\n---'   ____)____\n          ______)\n       __________)\n      (____)\n---.__(___)\n";
        return "\n    _______\n---'   ____)____\n          ______)\n          _______)\n         _______)\n---.__________)\n";
    }
    // Три кадра с той же паузой 600 мс, что в Rust.
    static void animateMoves() {
        clearScreen(); System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "             ROCK!%n"); System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "%s%n", moveAscii(Move.Rock)); pause(600);
        clearScreen(); System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "             SCISSORS!%n"); System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "%s%n", moveAscii(Move.Scissors)); pause(600);
        clearScreen(); System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "             PAPER!%n"); System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "%s%n", moveAscii(Move.Paper)); pause(600);
    }
    // Неизвестное слово даёт отсутствующий ход; вызывающая функция повторяет запрос.
    static Move readPlayerMove() {
        System.out.printf(Locale.ROOT, "Enter your move [rock / scissors / paper]: "); System.out.flush();
        String input=readLine();
        if (input.equals("rock") || input.equals("r")) return Move.Rock;
        if (input.equals("scissors") || input.equals("s")) return Move.Scissors;
        if (input.equals("paper") || input.equals("p")) return Move.Paper;
        return null;
    }
    // Случайное целое 0, 1 или 2 определяет ход компьютера.
    static Move computerMove() {
        int number=randomMove();
        if (number==0) return Move.Rock;
        if (number==1) return Move.Scissors;
        return Move.Paper;
    }
    // Возвращаем исход раунда по ходам игрока и компьютера.
    static RoundResult roundResult(Move player,Move computer) {
        if (player==computer) return RoundResult.Draw;
        if ((player==Move.Rock && computer==Move.Scissors) || (player==Move.Scissors && computer==Move.Paper) || (player==Move.Paper && computer==Move.Rock)) return RoundResult.Player;
        return RoundResult.Computer;
    }
    // Выводим текущие очки игрока и компьютера.
    static void showScore(int playerScore,int computerScore) {
        System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "========================================%n");
        System.out.printf(Locale.ROOT, "        PLAYER %s : %s COMPUTER%n", playerScore, computerScore);
        System.out.printf(Locale.ROOT, "========================================%n"); System.out.printf(Locale.ROOT, "%n");
    }
    // Выводим названия, символы и рисунки обоих ходов.
    static void showMoves(Move player,Move computer) {
        System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "PLAYER                         COMPUTER%n");
        System.out.printf(Locale.ROOT, "%s %s                     %s %s%n", moveEmoji(player), moveText(player), moveText(computer), moveEmoji(computer));
        System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "PLAYER:%n"); System.out.printf(Locale.ROOT, "%s%n", moveAscii(player)); System.out.printf(Locale.ROOT, "COMPUTER:%n"); System.out.printf(Locale.ROOT, "%s%n", moveAscii(computer));
    }
    // Выводим правила игры и допустимые варианты ввода.
    static void showIntro() {
        clearScreen(); System.out.printf(Locale.ROOT, "========================================%n"); System.out.printf(Locale.ROOT, "         ROCK - SCISSORS - PAPER%n");
        System.out.printf(Locale.ROOT, "========================================%n"); System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "The first to win two rounds wins the game.%n"); System.out.printf(Locale.ROOT, "%n");
        System.out.printf(Locale.ROOT, "You can enter:%n"); System.out.printf(Locale.ROOT, "  rock / r%n"); System.out.printf(Locale.ROOT, "  scissors / s%n"); System.out.printf(Locale.ROOT, "  paper / p%n"); System.out.printf(Locale.ROOT, "%n");
    }
    // Ничья не меняет счёт. Игра заканчивается после двух побед одной стороны.
    static void playGame() {
        int playerScore=0,computerScore=0;
        showIntro();
        while (playerScore < 2 && computerScore < 2) {
            Move player;
            while (true) {
                Move move=readPlayerMove();
                if (move != null) { player=move; break; }
                System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "Invalid move.%n"); System.out.printf(Locale.ROOT, "%n");
            }
            Move computer=computerMove();
            animateMoves(); clearScreen(); showMoves(player,computer);
            RoundResult result=roundResult(player,computer);
            if (result==RoundResult.Draw) System.out.printf(Locale.ROOT, "This round is a draw.%n");
            else if (result==RoundResult.Player) { System.out.printf(Locale.ROOT, "The player won this round.%n"); playerScore += 1; }
            else { System.out.printf(Locale.ROOT, "The computer won this round.%n"); computerScore += 1; }
            showScore(playerScore,computerScore);
            if (playerScore < 2 && computerScore < 2) pause(1200);
        }
        System.out.printf(Locale.ROOT, "%n");
        if (playerScore==2) System.out.printf(Locale.ROOT, "THE PLAYER WINS THE GAME!%n");
        else System.out.printf(Locale.ROOT, "THE COMPUTER WINS THE GAME!%n");
        System.out.printf(Locale.ROOT, "%n");
    }
    // Читаем ответ: y или yes означает повторную игру.
    static boolean wantsToPlayAgain() {
        System.out.printf(Locale.ROOT, "Play again? [y/n]: "); System.out.flush();
        String input=readLine();
        return input.equals("y") || input.equals("yes");
    }
    // Запускаем пример из упражнения и выводим результаты.
    public static void main(String[] args) {
        while (true) {
            playGame();
            if (!wantsToPlayAgain()) break;
        }
        System.out.printf(Locale.ROOT, "%n"); System.out.printf(Locale.ROOT, "Thanks for playing.%n");
    }
}
