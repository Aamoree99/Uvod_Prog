"""Примеры из условий и проверки краевых случаев: python3 -m unittest discover -s tests."""

import ast
import contextlib
from decimal import Decimal, localcontext, ROUND_DOWN
import importlib.util
import io
from pathlib import Path
import random
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]


def load(name):
    spec = importlib.util.spec_from_file_location('exercise_' + name, ROOT / 'exercises' / name / 'main.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class ExerciseTests(unittest.TestCase):
    def test_inventory_and_headers(self):
        rust = ROOT.parent / 'rust' / 'src' / 'bin'
        paths = list((ROOT / 'exercises').glob('*/main.py'))
        self.assertEqual(len(paths), 78)
        self.assertEqual({p.parent.name for p in paths}, {p.parent.name for p in rust.glob('*/main.rs')})
        for path in paths:
            source = (rust / path.parent.name / 'main.rs').read_text()
            header = source[:source.rfind('*/')].removeprefix('/*').strip()
            self.assertEqual(ast.get_docstring(ast.parse(path.read_text())), header, path)

    def test_runner(self):
        spec = importlib.util.spec_from_file_location('runner', ROOT / 'main.py')
        runner = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(runner)
        with tempfile.TemporaryDirectory() as directory:
            directory = Path(directory)
            for name, body in (('2_10', 'print("OK")'), ('2_2', 'raise SystemExit(3)'), ('zapocet_E', 'print("This exercise is not implemented yet.")')):
                path = directory / name
                path.mkdir()
                (path / 'main.py').write_text('"""Název: Test"""\n' + body + '\n')
            self.assertEqual([p.name for p in runner.discover_exercises(directory)], ['2_2', '2_10', 'zapocet_E'])
            with patch.object(runner, 'EXERCISES', directory), patch.object(sys, 'argv', ['main.py']), contextlib.redirect_stdout(io.StringIO()) as output, contextlib.redirect_stderr(io.StringIO()) as errors:
                self.assertEqual(runner.main(), 1)
            self.assertIn('Completed successfully: 1', output.getvalue())
            self.assertIn('Stubs executed: 1', output.getvalue())
            self.assertIn('Runtime errors: 1', output.getvalue())
            self.assertIn('exit code 3', errors.getvalue())
        for args in (['--help'], ['--list'], ['--bin', '03_03'], ['--from', '12_01']):
            result = subprocess.run([sys.executable, str(ROOT / 'main.py'), *args], cwd='/tmp', text=True, capture_output=True)
            self.assertEqual(result.returncode, 0, result.stderr)
        for args in (['--bin', 'missing'], ['--from', 'missing'], ['--bin', '01_01', '--from', '01_01']):
            result = subprocess.run([sys.executable, str(ROOT / 'main.py'), *args], text=True, capture_output=True)
            self.assertNotEqual(result.returncode, 0)

    def test_lcm(self):
        m = load('02_05')
        self.assertEqual(m.least_common_multiple(21, 30), 210)
        self.assertEqual(m.least_common_multiple(5, 0), 0)
        self.assertEqual(m.least_common_multiple(-21, 30), 210)

    def test_arithmetic(self):
        m = load('03_01')
        self.assertAlmostEqual(m.price_with_vat(56.20, 21), 68.002)
        self.assertEqual([m.price_with_vat_rate(115, i) for i in range(3)], [139.15, 132.25, 126.5])
        m = load('03_02')
        self.assertAlmostEqual(m.calculate_bmi(1.7, 61.5), 21.280276816609)
        self.assertEqual(m.classify_bmi(1.67, 80), 'overweight')
        m = load('03_03')
        self.assertEqual(m.factorial(11), 39916800)
        self.assertEqual(m.power(5, 3), 125)
        self.assertAlmostEqual(m.power(2.69, 7), 1019.215, places=3)
        self.assertAlmostEqual(m.exponential_series(5.14), 170.715768, places=5)
        self.assertAlmostEqual(m.combination(49, 6), 13983816)
        m = load('03_04')
        self.assertAlmostEqual(m.add((5.2, 3.25), (1.6, -4))[0], 6.8)
        self.assertEqual(m.add((5.2, 3.25), (1.6, -4))[1], -0.75)
        self.assertAlmostEqual(m.multiply((5.2, 3.25), (1.6, -4))[0], 21.32)
        self.assertAlmostEqual(m.multiply((5.2, 3.25), (1.6, -4))[1], -15.6)

    def test_geometry_and_calendar(self):
        m = load('03_05')
        b1, b2, b3 = (7.2, -2), (-2.9, -6.23), (2, 5)
        self.assertTrue(m.approximately_equal(5, 4.99999999999992))
        self.assertAlmostEqual(m.segment_length(b1, b2), 10.95, places=2)
        self.assertTrue(m.is_triangle(b1, b2, b3))
        self.assertFalse(m.is_triangle((-1, -2.5), b3, (3, 7.5)))
        self.assertAlmostEqual(m.triangle_perimeter((b1, b2, b3)), 31.923, places=3)
        t2 = ((3, 7.5), (3, -2.6), (5.2, -2.6))
        self.assertAlmostEqual(m.triangle_perimeter(t2), 22.637, places=3)
        self.assertTrue(m.is_right_triangle(t2))
        self.assertFalse(m.is_right_triangle(((0, 0), (0, 0), (0, 0))))
        self.assertEqual(load('03_07').date_after_days((16, 7, 2015), 500), (27, 11, 2016))
        self.assertEqual(load('03_07').date_after_days((28, 2, 2000), 1), (29, 2, 2000))
        self.assertEqual(load('03_08').day_of_week((15, 3, 1966)), 1)
        self.assertEqual(load('03_08').day_of_week((1, 1, 1899)), 6)

    def test_arrays_and_coins(self):
        self.assertEqual(load('04_01').sort_descending(5, 7), (7, 5))
        a = [5, -2, 0, -1, 7, -12, 2, 4]
        load('04_02').absolute_array(a)
        self.assertEqual(a, [5, 2, 0, 1, 7, 12, 2, 4])
        self.assertEqual(load('04_03').round_array([5.12, -3.5996, 156, 7.9, 2.5, -2.5]), [5, -4, 156, 8, 3, -3])
        m = load('04_04')
        a = [5, 7, 2, 1, 0, 6]
        m.reverse_array(a)
        self.assertEqual(a, [6, 0, 1, 2, 7, 5])
        m.reverse_array([])
        self.assertEqual(load('04_05').array_maxima([2, 5, 7, 1, 4, 6, 9, 8, 2]), (9, 8, 7))
        self.assertEqual(load('04_06').most_frequent_element([2, 7, 6, 6, 6, 1, 7, 7, 2]), (3, 7))
        self.assertEqual(load('04_07').scalar_product([2, 0, -5, 1, 4], [7, 4, 2, -2, 0]), 2)
        m = load('04_08')
        self.assertEqual([m.pay_with_coins([50, 20, 10, 5, 2, 1], n) for n in (97, 88, 26)], [5, 6, 3])
        self.assertEqual(m.pay_with_coins([4, 3, 1], 6), 3)
        self.assertEqual(m.optimal_coin_count([4, 3, 1], 0, 6), 2)
        m = load('zapocet_E')
        self.assertEqual(m.second_maximum([2, 5, 7, 1, 4, 6, 9, 8, 2]), 8)
        self.assertEqual(m.second_maximum([9, 9, 5]), 9)
        self.assertEqual(m.second_maximum([-5, -2, -9]), -5)
        self.assertIsNone(m.second_maximum([1]))

    def test_sorting_and_search(self):
        expected = [9, 8, 7, 6, 5, 5, 2, 1, 1, 0]
        for name in ('05_03', '05_04', '05_06', '05_07'):
            m = load(name)
            for algorithm in ('selection_sort', 'insertion_sort', 'bubble_sort', 'improved_bubble_sort', 'counting_sort'):
                if not hasattr(m, algorithm):
                    continue
                for original in ([5, 7, 1, 5, 6, 8, 9, 0, 1, 2], [], [1], [2, 2, 2]):
                    a = original.copy()
                    getattr(m, algorithm)(a)
                    self.assertEqual(a, expected if len(original) == 10 else original)
        m = load('05_07')
        a = [-5, 7, 15, 22, 4, 0, 4, 9, -11]
        m.counting_sort(a)
        self.assertEqual(a, [22, 15, 9, 7, 4, 4, 0, -5, -11])
        m = load('05_03')
        with self.assertRaises(ValueError):
            m.counting_sort([-1])
        m = load('05_05')
        a = [12, 9, 6, 6, 6, 5, 3, 1, 1]
        self.assertEqual(m.binary_search_first(a, 6), 2)
        self.assertEqual(a[m.binary_search(a, 6)], 6)
        self.assertEqual(m.binary_search(a, 10), -1)
        self.assertEqual(m.binary_search([], 6), -1)

    def test_strings(self):
        m = load('06_01')
        s = 'Alena ma 2 ruce.'
        self.assertEqual(m.count_distinct_letters(s), 8)
        self.assertEqual(m.count_words(s), 3)
        self.assertEqual(m.first_word(s, 2), 'Alena')
        self.assertEqual(m.first_word(s, len(s) - 1), '')
        self.assertEqual(m.longest_word(s), 'Alena')
        self.assertEqual(m.greatest_word(s), 'ruce')
        self.assertEqual(m.most_frequent_word(s + 'Mama ma 2 ruce a v ruce hul'), 'ruce')
        self.assertEqual(m.most_frequent_word('Ja a Jana'), 'Ja')
        self.assertEqual(m.most_frequent_word(''), '')
        m = load('06_02')
        original = m.to_units('Toto je tajny text: 1726😀')
        self.assertEqual(m.caesar_cipher(m.caesar_cipher(original, 20), -20), original)
        self.assertEqual(m.caesar_cipher([65535, 0], 1), [0, 1])
        m = load('06_03')
        a, b = list('klokan'), list('klobouk')
        self.assertEqual(m.compare(a, b), 1)
        self.assertEqual(m.index_of(b, ['o'], 3), 4)
        c = m.substring(b, 2, 3)
        self.assertEqual(c, list('obo'))
        self.assertEqual(m.insert(a, 1, c), list('kobolokan'))
        d = m.remove(b, 3, 2)
        self.assertEqual(d, list('klouk'))
        self.assertEqual(m.replace(d, ['k'], c), list('obolouobo'))
        self.assertEqual(m.index_of([], [], 0), 0)
        with self.assertRaises(ValueError):
            m.substring(b, -1, 2)

    def test_games(self):
        m = load('03_06')
        for player in range(3):
            for computer in range(3):
                expected = 'Draw' if player == computer else ('Player' if (computer - player) % 3 == 1 else 'Computer')
                self.assertEqual(m.round_result(player, computer), expected)
        with patch('builtins.input', side_effect=['invalid', 'rock', 'rock', 'n']), patch.object(m.random, 'randrange', return_value=1), contextlib.redirect_stdout(io.StringIO()) as output:
            m.main()
        self.assertIn('Invalid move.', output.getvalue())
        self.assertIn('THE PLAYER WINS THE GAME!', output.getvalue())
        m = load('04_09')
        self.assertFalse(m.validate_ticket([7, 9, 22, 6, 14, 14]))
        self.assertEqual(m.sportka([7, 9, 22, 6, 14, 14], 5000), 0)
        self.assertEqual(m.count_matches([1, 2, 3, 4, 5, 7], [1, 2, 3, 4, 5, 6, 7]), (5, True))
        self.assertEqual([m.calculate_prize(n, b) for n, b in ((6, False), (5, True), (5, False), (4, False), (3, False), (2, True))], [15382198, 815723, 24971, 619, 113, 0])
        with patch.object(m, 'generate_draw', return_value=[1, 2, 3, 4, 5, 6, 7]):
            self.assertEqual(m.sportka([1, 2, 3, 4, 5, 6], 1), 2 * 15382198 - 20)
        m = load('06_04')
        bets, count, quit_found = m.parse_bets('5 20 1 25 9 30 Suda 100 vysoka 200 hotovo')
        self.assertEqual((count, quit_found), (5, False))
        with contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(m.play_round(bets, count, 24), 225)  # В условии опечатка: 222.
        self.assertFalse(m.bet_wins((m.EVEN, 0, 10), 0))
        self.assertTrue(m.bet_wins((m.NUMBER, 0, 10), 0))
        self.assertTrue(m.parse_bets('5 20 quit')[2])
        self.assertTrue(m.parse_bets('odd quit')[2])
        result = subprocess.run([sys.executable, str(ROOT / 'main.py'), '--bin', '06_04'], input='quit\n', text=True, capture_output=True)
        self.assertEqual(result.returncode, 0)
        self.assertIn('Final balance: 0', result.stdout)

    def test_big_integers(self):
        m = load('07_01')
        a, b = m.create_number('560111202232646'), m.create_number('560100600690986')
        self.assertEqual(m.number_text(m.add(a, b)), '1120211802923632')
        difference = m.subtract(a, b)
        self.assertEqual(m.number_text(difference), '10601541660')
        self.assertEqual(m.number_text(m.multiply(a, difference)), '5938042244702081581032360')
        self.assertEqual(m.number_text(m.divide(a, difference)), '52832')
        # Встроенные целые используются только как независимый оракул в тестах.
        rng = random.Random(11)
        for name, base in (('07_01', 10), ('07_05', 2), ('07_05', 5), ('07_05', 7), ('07_05', 10)):
            m = load(name)
            for _ in range(15):
                x, y = rng.randrange(100000), rng.randrange(1, 100000)
                def digits(value):
                    result = [0] * m.MAX_DIGITS
                    i = m.MAX_DIGITS - 1
                    while value:
                        result[i] = value % base
                        value //= base
                        i -= 1
                    return result
                a, b = digits(x), digits(y)
                self.assertEqual(m.add(a, b, base), digits(x + y))
                self.assertEqual(m.subtract(a, b, base), digits(abs(x - y)))
                self.assertEqual(m.multiply(a, b, base), digits(x * y))
                self.assertEqual(m.divide_with_remainder(a, b, base), (digits(x // y), digits(x % y)))
            with self.assertRaises(ZeroDivisionError):
                m.divide_with_remainder(a, m.zero(), base)
            with self.assertRaises(OverflowError):
                m.add([base - 1] * m.MAX_DIGITS, digits(1), base)

    def test_long_reals(self):
        m = load('07_02')
        a, b, c = m.create_number('-256.2e-1'), m.create_number('-9534.07256e15'), m.create_number('9534.07256e15')
        self.assertEqual(m.number_text(a), '-2.562e1')
        self.assertEqual(m.compare(a, b), 1)  # В условии ошибочно указано -1.
        self.assertEqual(m.number_text(m.add(b, b)), '-1.906814512e19')
        self.assertEqual(m.number_text(m.add(a, c)), '9.53407255999999997438e18')
        self.assertEqual(m.number_text(m.add(b, c)), '0')
        self.assertEqual(m.number_text(m.multiply(a, b)), '2.442629389872e20')
        self.assertTrue(m.number_text(m.divide(b, a)).startswith('3.7213397970335675253'))
        self.assertEqual(m.number_text(m.divide(m.create_number('1'), m.create_number('3'))), '3.' + '3' * 39 + 'e-1')
        self.assertEqual(m.number_text(m.add(m.create_number('1e100'), m.create_number('-1'))), '9.' + '9' * 39 + 'e99')
        # Decimal используется только в тесте как независимый числовой оракул.
        with localcontext() as context:
            context.prec = 40
            context.rounding = ROUND_DOWN
            samples = ('-12.5', '2.5', '1e100', '-1', '1e-100', '123456789e20')
            for first in samples:
                for second in samples:
                    x, y = m.create_number(first), m.create_number(second)
                    self.assertEqual(Decimal(m.number_text(m.add(x, y))), Decimal(first) + Decimal(second))
                    self.assertEqual(Decimal(m.number_text(m.multiply(x, y))), Decimal(first) * Decimal(second))
                    self.assertEqual(Decimal(m.number_text(m.divide(x, y))), Decimal(first) / Decimal(second))
        with self.assertRaises(ZeroDivisionError):
            m.divide(a, m.zero())
        for text in ('', '.', '1e', '1.2.3', '1e1_0'):
            with self.assertRaises(ValueError):
                m.create_number(text)

    def test_fractions_and_numeral_systems(self):
        m = load('07_03')
        a = m.reduce_fraction(m.create_fraction('-56890/1223366'))
        b = m.reduce_fraction(m.create_fraction('2565689841/2365'))
        self.assertEqual(m.fraction_text(a), '-28445/611683')
        self.assertEqual(m.fraction_text(b), '233244531/215')
        self.assertEqual(m.fraction_text(m.add_fractions(a, b)), '142671708339998/131511845')
        self.assertEqual(m.fraction_text(m.multiply_fractions(a, b)), '-1326928136859/26302369')
        self.assertEqual(m.fraction_text(m.divide_fractions(b, a)), '-142671714455673/6115675')
        self.assertEqual(m.fraction_text(m.add_fractions(m.create_fraction('-1/3'), m.create_fraction('1/3'))), '0/1')
        m = load('07_04')
        self.assertEqual(m.decimal_to_roman(1969), 'MCMLXIX')
        self.assertEqual(m.roman_to_decimal('MMCDXXXVII'), 2437)
        for n in range(1, 4000):
            self.assertEqual(m.roman_to_decimal(m.decimal_to_roman(n)), n)
        for text in ('IM', 'IIII', 'VX', 'IC', 'ABC', ''):
            self.assertEqual(m.roman_to_decimal(text), -1)
        m = load('07_05')
        a = m.create_number('560111202232646')
        b = m.convert(a, 10, 2)
        self.assertEqual(m.number_text(b), '1111111010110101100001101001111100110100101000110')
        c = m.convert(b, 2, 5)
        self.assertEqual(m.number_text(c), '1041403330220232421041')
        self.assertEqual(m.number_text(m.add(c, c, 5)), '2133312210441020342132')
        d = m.convert(c, 5, 7)
        self.assertEqual(m.number_text(d), '225656461511451251')
        self.assertEqual(m.number_text(m.multiply(d, d, 7)), '55402656051563425266136012062265231')
        self.assertEqual(m.convert(d, 7, 10), a)


if __name__ == '__main__':
    unittest.main()
