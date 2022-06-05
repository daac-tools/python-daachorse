import daachorse
import ahocorasick_rs

from tests import dataset


def run_daachorse(pma, haystack):
    pma.find_overlapping(haystack)


def run_ahocorasick_rs(pma, haystack):
    pma.find_matches_as_indexes(haystack, True)


def test_daachorse_words_100000_bench(benchmark):
    patterns = dataset.load_words_100000()
    haystack = dataset.load_sherlock()
    pma = daachorse.Automaton(patterns)
    benchmark(run_daachorse, pma, haystack)


def test_ahocorasick_rs_word_100000_bench(benchmark):
    patterns = dataset.load_words_100000()
    haystack = dataset.load_sherlock()
    pma = ahocorasick_rs.AhoCorasick(patterns)
    benchmark(run_ahocorasick_rs, pma, haystack)
