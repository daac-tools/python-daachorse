use pyo3::{exceptions::PyValueError, prelude::*, types::PyUnicode};

use daachorse::{
    charwise::{CharwiseDoubleArrayAhoCorasick, CharwiseDoubleArrayAhoCorasickBuilder},
    MatchKind,
};

/// An Aho-Corasick automaton using the compact double-array data structure.
///
/// # Arguments
///
/// * `patterns` - List of string patterns.
/// * `match_kind` - A search option of the Aho-Corasick automaton. (default:
///   `MATCH_KIND_STANDARD`)
///
/// # Examples
///
/// ```python
/// >>> import daachorse
/// >>> patterns = ['bcd', 'ab', 'a']
/// >>> pma = daachorse.Automaton(patterns)
/// >>> pma.find('abcd')
/// [(0, 1, 2), (1, 4, 0)]
/// ```
#[pyclass]
struct Automaton {
    pma: CharwiseDoubleArrayAhoCorasick,
    match_kind: MatchKind,
    patterns: Vec<Py<PyUnicode>>,
}

#[pymethods]
impl Automaton {
    #[new]
    #[args(match_kind = "0")]
    fn new(py: Python, patterns: Vec<Py<PyUnicode>>, match_kind: u8) -> PyResult<Self> {
        let raw_patterns: PyResult<Vec<String>> = patterns
            .iter()
            .map(|pat| pat.as_ref(py).extract())
            .collect();
        let raw_patterns = raw_patterns?;
        let match_kind = MatchKind::from(match_kind);
        Ok(Self {
            pma: py
                .allow_threads(|| {
                    CharwiseDoubleArrayAhoCorasickBuilder::new()
                        .match_kind(match_kind)
                        .build(raw_patterns)
                })
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            match_kind,
            patterns,
        })
    }

    /// Returns a list of non-overlapping matches in the given haystack.
    ///
    /// # Arguments
    ///
    /// * `haystack` - String to search for.
    ///
    /// # Example 1: Standard semantics
    ///
    /// ```python
    /// >>> import daachorse
    /// >>> patterns = ['bcd', 'ab', 'a']
    /// >>> pma = daachorse.Automaton(patterns)
    /// >>> pma.find('abcd')
    /// [(0, 1, 2), (1, 4, 0)]
    /// ```
    ///
    /// # Example 2: Leftmost longest semantics
    ///
    /// ```python
    /// >>> import daachorse
    /// >>> patterns = ['ab', 'a', 'abcd']
    /// >>> pma = daachorse.Automaton(patterns, daachorse.MATCH_KIND_LEFTMOST_LONGEST)
    /// >>> pma.find('abcd')
    /// [(0, 4, 2)]
    /// ```
    ///
    /// # Example 3: Leftmost first semantics
    ///
    /// ```python
    /// >>> import daachorse
    /// >>> patterns = ['ab', 'a', 'abcd']
    /// >>> pma = daachorse.Automaton(patterns, daachorse.MATCH_KIND_LEFTMOST_FIRST)
    /// >>> pma.find('abcd')
    /// [(0, 2, 0)]
    /// ```
    fn find(self_: PyRef<Self>, haystack: &str) -> PyResult<Vec<(usize, usize, usize)>> {
        let mut pos_map = vec![0; haystack.len() + 1];
        let mut len_in_chars = 0;
        let match_kind = self_.match_kind;
        let py = self_.py();
        let pma = &self_.pma;
        Ok(py.allow_threads(|| unsafe {
            for (i, (j, _)) in haystack.char_indices().enumerate() {
                debug_assert!(j < pos_map.len());
                *pos_map.get_unchecked_mut(j) = i;
                len_in_chars = i;
            }
            *pos_map.last_mut().unwrap_unchecked() = len_in_chars + 1;
            match match_kind {
                MatchKind::Standard => pma
                    .find_iter(haystack)
                    .map(|m| {
                        (
                            *pos_map.get_unchecked(m.start()),
                            *pos_map.get_unchecked(m.end()),
                            m.value(),
                        )
                    })
                    .collect(),
                MatchKind::LeftmostLongest | MatchKind::LeftmostFirst => pma
                    .leftmost_find_iter(haystack)
                    .map(|m| {
                        (
                            *pos_map.get_unchecked(m.start()),
                            *pos_map.get_unchecked(m.end()),
                            m.value(),
                        )
                    })
                    .collect(),
            }
        }))
    }

    /// Returns a list of non-overlapping match strings in the given haystack.
    ///
    /// # Arguments
    ///
    /// * `haystack` - String to search for.
    ///
    /// # Example 1: Standard semantics
    ///
    /// ```python
    /// >>> import daachorse
    /// >>> patterns = ['bcd', 'ab', 'a']
    /// >>> pma = daachorse.Automaton(patterns)
    /// >>> pma.find_as_strings('abcd')
    /// ['a', 'bcd']
    /// ```
    ///
    /// # Example 2: Leftmost longest semantics
    ///
    /// ```python
    /// >>> import daachorse
    /// >>> patterns = ['ab', 'a', 'abcd']
    /// >>> pma = daachorse.Automaton(patterns, daachorse.MATCH_KIND_LEFTMOST_LONGEST)
    /// >>> pma.find_as_strings('abcd')
    /// ['abcd']
    /// ```
    ///
    /// # Example 3: Leftmost first semantics
    ///
    /// ```python
    /// >>> import daachorse
    /// >>> patterns = ['ab', 'a', 'abcd']
    /// >>> pma = daachorse.Automaton(patterns, daachorse.MATCH_KIND_LEFTMOST_FIRST)
    /// >>> pma.find_as_strings('abcd')
    /// ['ab']
    /// ```
    fn find_as_strings(self_: PyRef<Self>, haystack: &str) -> PyResult<Vec<Py<PyUnicode>>> {
        let match_kind = self_.match_kind;
        let py = self_.py();
        let pma = &self_.pma;
        let pattern_ids: Vec<_> = py.allow_threads(|| match match_kind {
            MatchKind::Standard => pma.find_iter(haystack).map(|m| m.value()).collect(),
            MatchKind::LeftmostLongest | MatchKind::LeftmostFirst => pma
                .leftmost_find_iter(haystack)
                .map(|m| m.value())
                .collect(),
        });
        Ok(pattern_ids
            .into_iter()
            .map(|i| unsafe { self_.patterns.get_unchecked(i).clone_ref(py) })
            .collect())
    }

    /// Returns a list of overlapping matches in the given haystack.
    ///
    /// # Arguments
    ///
    /// * `haystack` - String to search for.
    ///
    /// # Errors
    ///
    /// If you specify a LEFTMOST option, this function raises an error.
    ///
    /// # Examples
    ///
    /// ```python
    /// >>> import daachorse
    /// >>> patterns = ['bcd', 'ab', 'a']
    /// >>> pma = daachorse.Automaton(patterns)
    /// >>> pma.find_overlapping('abcd')
    /// [(0, 1, 2), (0, 2, 1), (1, 4, 0)]
    /// ```
    fn find_overlapping(
        self_: PyRef<Self>,
        haystack: &str,
    ) -> PyResult<Vec<(usize, usize, usize)>> {
        if self_.match_kind != MatchKind::Standard {
            return Err(PyValueError::new_err("match_kind must be STANDARD"));
        }
        let py = self_.py();
        let pma = &self_.pma;
        Ok(py.allow_threads(|| {
            let mut pos_map = vec![0; haystack.len() + 1];
            let mut len_in_chars = 0;
            unsafe {
                for (i, (j, _)) in haystack.char_indices().enumerate() {
                    debug_assert!(j < pos_map.len());
                    *pos_map.get_unchecked_mut(j) = i;
                    len_in_chars = i;
                }
                *pos_map.last_mut().unwrap_unchecked() = len_in_chars + 1;
                pma.find_overlapping_iter(haystack)
                    .map(|m| {
                        (
                            *pos_map.get_unchecked(m.start()),
                            *pos_map.get_unchecked(m.end()),
                            m.value(),
                        )
                    })
                    .collect()
            }
        }))
    }

    /// Returns a list of overlapping match strings in the given haystack.
    ///
    /// # Arguments
    ///
    /// * `haystack` - String to search for.
    ///
    /// # Errors
    ///
    /// If you specify a LEFTMOST option, this function raises an error.
    ///
    /// # Examples
    ///
    /// ```python
    /// >>> import daachorse
    /// >>> patterns = ['bcd', 'ab', 'a']
    /// >>> pma = daachorse.Automaton(patterns)
    /// >>> pma.find_overlapping_as_strings('abcd')
    /// ['a', 'ab', 'bcd']
    /// ```
    fn find_overlapping_as_strings(
        self_: PyRef<Self>,
        haystack: &str,
    ) -> PyResult<Vec<Py<PyUnicode>>> {
        if self_.match_kind != MatchKind::Standard {
            return Err(PyValueError::new_err("match_kind must be STANDARD"));
        }
        let py = self_.py();
        let pma = &self_.pma;
        let pattern_ids: Vec<_> = py.allow_threads(|| {
            pma.find_overlapping_iter(haystack)
                .map(|m| m.value())
                .collect()
        });
        Ok(pattern_ids
            .into_iter()
            .map(|i| unsafe { self_.patterns.get_unchecked(i).clone_ref(py) })
            .collect())
    }

    /// Returns a list of overlapping matches without suffixes in the given haystack iterator.
    ///
    /// The Aho-Corasick algorithm reads through the haystack from left to right and reports
    /// matches when it reaches the end of each pattern. In the overlapping match, more than one
    /// pattern can be returned per report.
    ///
    /// This iterator returns the first match on each report.
    ///
    /// # Arguments
    ///
    /// * `haystack` - String to search for.
    ///
    /// # Errors
    ///
    /// If you specify a LEFTMOST option, this function raises an error.
    ///
    /// # Examples
    ///
    /// ```python
    /// >>> import daachorse
    /// >>> patterns = ['bcd', 'cd', 'abc']
    /// >>> pma = daachorse.Automaton(patterns)
    /// >>> pma.find_overlapping_no_suffix('abcd')
    /// [(0, 3, 2), (1, 4, 0)]
    /// ```
    fn find_overlapping_no_suffix(
        self_: PyRef<Self>,
        haystack: &str,
    ) -> PyResult<Vec<(usize, usize, usize)>> {
        if self_.match_kind != MatchKind::Standard {
            return Err(PyValueError::new_err("match_kind must be STANDARD"));
        }
        let py = self_.py();
        let pma = &self_.pma;
        Ok(py.allow_threads(|| {
            let mut pos_map = vec![0; haystack.len() + 1];
            let mut len_in_chars = 0;
            unsafe {
                for (i, (j, _)) in haystack.char_indices().enumerate() {
                    debug_assert!(j < pos_map.len());
                    *pos_map.get_unchecked_mut(j) = i;
                    len_in_chars = i;
                }
                *pos_map.last_mut().unwrap_unchecked() = len_in_chars + 1;
                pma.find_overlapping_no_suffix_iter(haystack)
                    .map(|m| {
                        (
                            *pos_map.get_unchecked(m.start()),
                            *pos_map.get_unchecked(m.end()),
                            m.value(),
                        )
                    })
                    .collect()
            }
        }))
    }

    /// Returns a list of overlapping match strings without suffixes in the given haystack iterator.
    ///
    /// The Aho-Corasick algorithm reads through the haystack from left to right and reports
    /// matches when it reaches the end of each pattern. In the overlapping match, more than one
    /// pattern can be returned per report.
    ///
    /// This iterator returns the first match on each report.
    ///
    /// # Arguments
    ///
    /// * `haystack` - String to search for.
    ///
    /// # Errors
    ///
    /// If you specify a LEFTMOST option, this function raises an error.
    ///
    /// # Examples
    ///
    /// ```python
    /// >>> import daachorse
    /// >>> patterns = ['bcd', 'cd', 'abc']
    /// >>> pma = daachorse.Automaton(patterns)
    /// >>> pma.find_overlapping_no_suffix_as_strings('abcd')
    /// ['abc', 'bcd']
    /// ```
    fn find_overlapping_no_suffix_as_strings(
        self_: PyRef<Self>,
        haystack: &str,
    ) -> PyResult<Vec<Py<PyUnicode>>> {
        if self_.match_kind != MatchKind::Standard {
            return Err(PyValueError::new_err("match_kind must be STANDARD"));
        }
        let py = self_.py();
        let pma = &self_.pma;
        let pattern_ids: Vec<_> = py.allow_threads(|| {
            pma.find_overlapping_no_suffix_iter(haystack)
                .map(|m| m.value())
                .collect()
        });
        Ok(pattern_ids
            .into_iter()
            .map(|i| unsafe { self_.patterns.get_unchecked(i).clone_ref(py) })
            .collect())
    }
}

#[pymodule]
fn daachorse(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Automaton>()?;
    m.add("MATCH_KIND_STANDARD", MatchKind::Standard as u8)?;
    m.add(
        "MATCH_KIND_LEFTMOST_LONGEST",
        MatchKind::LeftmostLongest as u8,
    )?;
    m.add("MATCH_KIND_LEFTMOST_FIRST", MatchKind::LeftmostFirst as u8)?;
    Ok(())
}
