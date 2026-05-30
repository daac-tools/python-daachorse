Example usage
=============

Daachorse contains some search options, ranging from standard matching with the Aho-Corasick
algorithm to more advanced matching. All of them run efficiently, powered by the double-array data
structure, and can be easily plugged into your application, as shown below.

Finding overlapping occurrences
-------------------------------

To search for all occurrences of registered patterns that allow for positional overlap in the input
text, use ``find_overlapping()``. When you instantiate a new automaton, unique identifiers are
assigned to each pattern in the input order. The match result has the byte positions of the
occurrence and its identifier.

.. code-block:: python

   >>> import daachorse
   >>> patterns = [b'bcd', b'ab', b'a']
   >>> pma = daachorse.DoubleArrayAhoCorasick(patterns)
   >>> pma.find_overlapping(b'abcd')
   [(0, 1, 2), (0, 2, 1), (1, 4, 0)]

Finding non-overlapping occurrences with standard matching
----------------------------------------------------------

To disallow positional overlap, use ``find()`` instead. It performs the search on the Aho-Corasick
automaton and reports the first matching pattern found at each search position.

.. code-block:: python

   >>> import daachorse
   >>> patterns = [b'bcd', b'ab', b'a']
   >>> pma = daachorse.DoubleArrayAhoCorasick(patterns)
   >>> pma.find(b'abcd')
   [(0, 1, 2), (1, 4, 0)]

Finding non-overlapping occurrences with longest matching
---------------------------------------------------------

To search for the longest pattern without positional overlap in each iteration, use
``MATCH_KIND_LEFTMOST_LONGEST`` during construction.

.. code-block:: python

   >>> import daachorse
   >>> patterns = [b'ab', b'a', b'abcd']
   >>> pma = daachorse.DoubleArrayAhoCorasick(patterns, daachorse.MATCH_KIND_LEFTMOST_LONGEST)
   >>> pma.find(b'abcd')
   [(0, 4, 2)]

Finding non-overlapping occurrences with leftmost-first matching
----------------------------------------------------------------

To search for the earliest registered pattern among those starting from the search position,
use ``MATCH_KIND_LEFTMOST_FIRST``.

This semantics is so-called *the leftmost first match*, a tricky search option. For example,
in the following code, ``ab`` is reported because it is the earliest registered one.

.. code-block:: python

   >>> import daachorse
   >>> patterns = [b'ab', b'a', b'abcd']
   >>> pma = daachorse.DoubleArrayAhoCorasick(patterns, daachorse.MATCH_KIND_LEFTMOST_FIRST)
   >>> pma.find(b'abcd')
   [(0, 2, 0)]

Finding patterns on a string
----------------------------

To build an automaton for strings, use ``CharwiseDoubleArrayAhoCorasick`` instead.

.. code-block:: python

   >>> import daachorse
   >>> patterns = ['全世界', '世界', 'に']
   >>> pma = daachorse.CharwiseDoubleArrayAhoCorasick(patterns)
   >>> pma.find('全世界中に')
   [(0, 3, 0), (4, 5, 2)]
