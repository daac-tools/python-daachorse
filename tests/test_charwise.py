import pytest
import pickle

import daachorse


def test_default_find() -> None:
    haystack = 'this is a テスト'
    patterns = ['t', 'hi', 'h', 'this', 'テス']
    pma = daachorse.CharwiseDoubleArrayAhoCorasick(patterns)

    assert [
        (0, 1, 0),
        (1, 2, 2),
        (10, 12, 4),
    ] == pma.find(haystack)


def test_standard_find() -> None:
    haystack = 'this is a テスト'
    patterns = ['t', 'hi', 'h', 'this', 'テス']
    pma = daachorse.CharwiseDoubleArrayAhoCorasick(patterns, daachorse.MATCH_KIND_STANDARD)

    assert [
        (0, 1, 0),
        (1, 2, 2),
        (10, 12, 4),
    ] == pma.find(haystack)


def test_leftmost_longest_find() -> None:
    haystack = 'this is a テスト'
    patterns = ['t', 'hi', 'h', 'this', 'テス']
    pma = daachorse.CharwiseDoubleArrayAhoCorasick(patterns, daachorse.MATCH_KIND_LEFTMOST_LONGEST)

    assert [
        (0, 4, 3),
        (10, 12, 4),
    ] == pma.find(haystack)


def test_leftmost_first_find() -> None:
    haystack = 'this is a テスト'
    patterns = ['t', 'hi', 'h', 'this', 'テス']
    pma = daachorse.CharwiseDoubleArrayAhoCorasick(patterns, daachorse.MATCH_KIND_LEFTMOST_FIRST)

    assert [
        (0, 1, 0),
        (1, 3, 1),
        (10, 12, 4),
    ] == pma.find(haystack)


def test_find_overlapping() -> None:
    haystack = 'this is a テスト'
    patterns = ['t', 'hi', 'h', 'this', 'テス']
    pma = daachorse.CharwiseDoubleArrayAhoCorasick(patterns)

    assert [
        (0, 1, 0),
        (1, 2, 2),
        (1, 3, 1),
        (0, 4, 3),
        (10, 12, 4),
    ] == pma.find_overlapping(haystack)


def test_find_overlapping_invalid_option() -> None:
    haystack = 'this is a テスト'
    patterns = ['t', 'hi', 'h', 'this', 'テス']

    pma = daachorse.CharwiseDoubleArrayAhoCorasick(patterns, daachorse.MATCH_KIND_LEFTMOST_LONGEST)
    with pytest.raises(ValueError):
        pma.find_overlapping(haystack)

    pma = daachorse.CharwiseDoubleArrayAhoCorasick(patterns, daachorse.MATCH_KIND_LEFTMOST_FIRST)
    with pytest.raises(ValueError):
        pma.find_overlapping(haystack)


def test_serialization() -> None:
    haystack = 'this is a テスト'
    patterns = ['t', 'hi', 'h', 'this', 'テス']
    pma = daachorse.CharwiseDoubleArrayAhoCorasick(patterns)
    serialized = pma.serialize()
    pma = daachorse.CharwiseDoubleArrayAhoCorasick.deserialize(serialized)

    assert [
        (0, 1, 0),
        (1, 2, 2),
        (1, 3, 1),
        (0, 4, 3),
        (10, 12, 4),
    ] == pma.find_overlapping(haystack)


def test_pickle() -> None:
    haystack = 'this is a テスト'
    patterns = ['t', 'hi', 'h', 'this', 'テス']
    pma = daachorse.CharwiseDoubleArrayAhoCorasick(patterns)
    pma = pickle.loads(pickle.dumps(pma))

    assert [
        (0, 1, 0),
        (1, 2, 2),
        (1, 3, 1),
        (0, 4, 3),
        (10, 12, 4),
    ] == pma.find_overlapping(haystack)
