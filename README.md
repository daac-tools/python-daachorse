# python-daachorse

[daachorse](https://github.com/daac-tools/daachorse) is a fast implementation of the Aho-Corasick algorithm using the compact double-array data structure.
This is a Python wrapper.

## Installation

To use daachorse, run the following command:

```
$ pip install daachorse
```

## Example usage

Daachorse contains some search options,
ranging from basic matching with the Aho-Corasick algorithm to trickier matching.
All of them will run very fast based on the double-array data structure and
can be easily plugged into your application as shown below.

### Finding overlapped occurrences

To search for all occurrences of registered patterns
that allow for positional overlap in the input text,
use `find_overlapping()`. When you instantiate a new automaton,
unique identifiers are assigned to each pattern in the input order.
The match result has the character positions of the occurrence and its identifier.

```python
>> import daachorse
>> patterns = ['bcd', 'ab', 'a']
>> pma = daachorse.Automaton(patterns)
>> pma.find_overlapping('abcd')
[(0, 1, 2), (0, 2, 1), (1, 4, 0)]
```

### Finding non-overlapped occurrences with standard matching

If you do not want to allow positional overlap, use `find()` instead.
It performs the search on the Aho-Corasick automaton
and reports patterns first found in each iteration.

```python
>> import daachorse
>> patterns = ['bcd', 'ab', 'a']
>> pma = daachorse.Automaton(patterns)
>> pma.find('abcd')
[(0, 1, 2), (1, 4, 0)]
```

### Finding non-overlapped occurrences with longest matching

If you want to search for the longest pattern without positional overlap in each iteration,
use `MATCH_KIND_LEFTMOST_LONGEST` in the construction.

```python
>> import daachorse
>> patterns = ['ab', 'a', 'abcd']
>> pma = daachorse.Automaton(patterns, daachorse.MATCH_KIND_LEFTMOST_LONGEST)
>> pma.find('abcd')
[(0, 4, 2)]
```

### Finding non-overlapped occurrences with leftmost-first matching

If you want to find the the earliest registered pattern
among ones starting from the search position,
use `MATCH_KIND_LEFTMOST_FIRST`.

This is so-called *the leftmost first match*, a bit tricky search option.
For example, in the following code,
`ab` is reported because it is the earliest registered one.

```python
>> import daachorse
>> patterns = ['ab', 'a', 'abcd']
>> pma = daachorse.Automaton(patterns, daachorse.MATCH_KIND_LEFTMOST_FIRST)
>> pma.find('abcd')
[(0, 2, 0)]
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

For softwares under `tests/data`, follow the license terms of each software.
