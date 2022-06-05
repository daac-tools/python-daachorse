use pyo3::{exceptions::PyValueError, prelude::*};

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
}

#[pymethods]
impl Automaton {
    #[new]
    #[args(match_kind = "0")]
    fn new(py: Python, patterns: Vec<String>, match_kind: u8) -> PyResult<Self> {
        let match_kind = MatchKind::from(match_kind);
        Ok(Self {
            pma: py
                .allow_threads(|| {
                    CharwiseDoubleArrayAhoCorasickBuilder::new()
                        .match_kind(match_kind)
                        .build(patterns)
                })
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            match_kind,
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
    fn find(&self, haystack: &str) -> PyResult<Vec<(usize, usize, usize)>> {
        let mut pos_map = vec![0; haystack.len() + 1];
        let mut len_in_chars = 0;
        unsafe {
            for (i, (j, _)) in haystack.char_indices().enumerate() {
                debug_assert!(j < pos_map.len());
                *pos_map.get_unchecked_mut(j) = i;
                len_in_chars = i;
            }
            *pos_map.last_mut().unwrap_unchecked() = len_in_chars + 1;
        }
        Ok(match self.match_kind {
            MatchKind::Standard => self
                .pma
                .find_iter(haystack)
                .map(|m| (pos_map[m.start()], pos_map[m.end()], m.value()))
                .collect(),
            MatchKind::LeftmostLongest | MatchKind::LeftmostFirst => self
                .pma
                .leftmost_find_iter(haystack)
                .map(|m| (pos_map[m.start()], pos_map[m.end()], m.value()))
                .collect(),
        })
    }

    /// Returns a list of overlapping matches in the given haystack.
    ///
    /// # Arguments
    ///
    /// * `haystack` - String to search for.
    ///
    /// # Errors
    ///
    /// When you specify a LEFTMOST option, this function raises an error.
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
    fn find_overlapping(&self, haystack: &str) -> PyResult<Vec<(usize, usize, usize)>> {
        if self.match_kind != MatchKind::Standard {
            return Err(PyValueError::new_err("match_kind must be STANDARD"));
        }
        let mut pos_map = vec![0; haystack.len() + 1];
        let mut len_in_chars = 0;
        unsafe {
            for (i, (j, _)) in haystack.char_indices().enumerate() {
                debug_assert!(j < pos_map.len());
                *pos_map.get_unchecked_mut(j) = i;
                len_in_chars = i;
            }
            *pos_map.last_mut().unwrap_unchecked() = len_in_chars + 1;
        }
        Ok(self
            .pma
            .find_overlapping_iter(haystack)
            .map(|m| (pos_map[m.start()], pos_map[m.end()], m.value()))
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
    /// When you specify a LEFTMOST option, this function raises an error.
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
    fn find_overlapping_no_suffix(&self, haystack: &str) -> PyResult<Vec<(usize, usize, usize)>> {
        if self.match_kind != MatchKind::Standard {
            return Err(PyValueError::new_err("match_kind must be STANDARD"));
        }
        let mut pos_map = vec![0; haystack.len() + 1];
        let mut len_in_chars = 0;
        unsafe {
            for (i, (j, _)) in haystack.char_indices().enumerate() {
                debug_assert!(j < pos_map.len());
                *pos_map.get_unchecked_mut(j) = i;
                len_in_chars = i;
            }
            *pos_map.last_mut().unwrap_unchecked() = len_in_chars + 1;
        }
        Ok(self
            .pma
            .find_overlapping_no_suffix_iter(haystack)
            .map(|m| (pos_map[m.start()], pos_map[m.end()], m.value()))
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
