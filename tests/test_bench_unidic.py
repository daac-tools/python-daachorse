import daachorse
import ahocorasick_rs

from tests import dataset


def run_daachorse(pma, haystack):
    pma.find_overlapping(haystack)


def run_ahocorasick_rs(pma, haystack):
    pma.find_matches_as_indexes(haystack, True)


def test_daachorse_unidic_bench(benchmark):
    patterns = dataset.load_unidic()
    haystack = dataset.load_wagahaiwa_nekodearu()
    pma = daachorse.Automaton(patterns)
    benchmark(run_daachorse, pma, haystack)


def test_ahocorasick_rs_unidic_bench(benchmark):
    patterns = dataset.load_unidic()
    haystack = dataset.load_wagahaiwa_nekodearu()
    pma = ahocorasick_rs.AhoCorasick(patterns)
    benchmark(run_ahocorasick_rs, pma, haystack)
