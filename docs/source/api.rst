API reference
=============

.. autoclass:: daachorse.Automaton
   :members:

.. data:: MATCH_KIND_STANDARD
   :type: int
   :canonical: daachorse.MATCH_KIND_STANDARD

   The standard match semantics, which enables ``find()``, ``find_overlapping()``, and
   ``find_overlapping_no_suffix()``. Patterns are reported in the order that follows the normal
   behaviour of the Aho-Corasick algorithm.

.. data:: MATCH_KIND_LEFTMOST_LONGEST
   :type: int
   :canonical: daachorse.MATCH_KIND_LEFTMOST_LONGEST

   The leftmost-longest match semantics, which enables ``find()``. When multiple patterns are
   started from the same positions, the longest pattern will be reported. For example, when
   matching patterns ``ab|a|abcd`` over ``abcd``, ``abcd`` will be reported.

.. data:: MATCH_KIND_LEFTMOST_FIRST
   :type: int
   :canonical: daachorse.MATCH_KIND_LEFTMOST_FIRST

   The leftmost-first match semantics, which enables ``find()``. When multiple patterns are started
   from the same positions, the pattern that is registered earlier will be reported. For example,
   when matching patterns ``ab|a|abcd`` over ``abcd``, ``ab`` will be reported.
