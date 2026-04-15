use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyBytes, PyString},
};

/// A byte-wise Aho-Corasick automaton using the compact double-array data structure.
///
/// Examples:
///     >>> import daachorse
///     >>> patterns = [b'bcd', b'ab', b'a']
///     >>> pma = daachorse.DoubleArrayAhoCorasick(patterns)
///     >>> pma.find(b'abcd')
///     [(0, 1, 2), (1, 4, 0)]
///
/// :param patterns: List of bytes patterns.
/// :param match_kind: A search option of the Aho-Corasick automaton.
/// :type patterns: list[bytes]
/// :type match_kind: int
/// :rtype: daachorse.DoubleArrayAhoCorasick
#[pyclass(module = "daachorse")]
struct DoubleArrayAhoCorasick {
    pma: ::daachorse::DoubleArrayAhoCorasick<u32>,
}

#[pymethods]
impl DoubleArrayAhoCorasick {
    #[new]
    #[pyo3(signature = (patterns, /, match_kind = 0))]
    fn new(py: Python, patterns: Vec<Py<PyBytes>>, match_kind: u8) -> PyResult<Self> {
        let raw_patterns: PyResult<Vec<Vec<u8>>> =
            patterns.iter().map(|pat| pat.extract(py)).collect();
        let raw_patterns = raw_patterns?;
        let match_kind = ::daachorse::MatchKind::from(match_kind);
        Ok(Self {
            pma: py
                .detach(|| {
                    ::daachorse::DoubleArrayAhoCorasickBuilder::new()
                        .match_kind(match_kind)
                        .build(raw_patterns)
                })
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    /// Returns a list of non-overlapping matches in the given haystack.
    ///
    /// Example 1: Standard semantics
    ///     >>> import daachorse
    ///     >>> patterns = [b'bcd', b'ab', b'a']
    ///     >>> pma = daachorse.DoubleArrayAhoCorasick(patterns)
    ///     >>> pma.find(b'abcd')
    ///     [(0, 1, 2), (1, 4, 0)]
    ///
    /// Example 2: Leftmost longest semantics
    ///     >>> import daachorse
    ///     >>> patterns = [b'ab', b'a', b'abcd']
    ///     >>> pma = daachorse.DoubleArrayAhoCorasick(patterns, daachorse.MATCH_KIND_LEFTMOST_LONGEST)
    ///     >>> pma.find(b'abcd')
    ///     [(0, 4, 2)]
    ///
    /// Example 3: Leftmost first semantics
    ///     >>> import daachorse
    ///     >>> patterns = [b'ab', b'a', b'abcd']
    ///     >>> pma = daachorse.DoubleArrayAhoCorasick(patterns, daachorse.MATCH_KIND_LEFTMOST_FIRST)
    ///     >>> pma.find(b'abcd')
    ///     [(0, 2, 0)]
    ///
    /// :param haystack: Bytes to search for.
    /// :type haystack: bytes
    /// :return: A list of matches. Each match is a tuple consisting of the start position, end
    ///          position, and pattern ID.
    /// :rtype: list[tuple[int, int, int]]
    #[pyo3(signature = (haystack, /))]
    fn find(&self, py: Python, haystack: &[u8]) -> PyResult<Vec<(usize, usize, u32)>> {
        let match_kind = self.pma.match_kind();
        Ok(py.detach(|| match match_kind {
            ::daachorse::MatchKind::Standard => self
                .pma
                .find_iter(haystack)
                .map(|m| (m.start(), m.end(), m.value()))
                .collect(),
            ::daachorse::MatchKind::LeftmostLongest | ::daachorse::MatchKind::LeftmostFirst => self
                .pma
                .leftmost_find_iter(haystack)
                .map(|m| (m.start(), m.end(), m.value()))
                .collect(),
        }))
    }

    /// Returns a list of overlapping matches in the given haystack.
    ///
    /// Examples:
    ///     >>> import daachorse
    ///     >>> patterns = [b'bcd', b'ab', b'a']
    ///     >>> pma = daachorse.DoubleArrayAhoCorasick(patterns)
    ///     >>> pma.find_overlapping(b'abcd')
    ///     [(0, 1, 2), (0, 2, 1), (1, 4, 0)]
    ///
    /// :param haystack: Bytes to search for.
    /// :type haystack: bytes
    /// :return: A list of matches. Each match is a tuple consisting of the start position, end
    ///          position, and pattern ID.
    /// :rtype: list[tuple[int, int, int]]
    /// :raises ValueError: if the automaton is built with a LEFTMOST option.
    #[pyo3(signature = (haystack, /))]
    fn find_overlapping(&self, py: Python, haystack: &[u8]) -> PyResult<Vec<(usize, usize, u32)>> {
        if self.pma.match_kind() != ::daachorse::MatchKind::Standard {
            return Err(PyValueError::new_err("match_kind must be STANDARD"));
        }
        Ok(py.detach(|| {
            self.pma
                .find_overlapping_iter(haystack)
                .map(|m| (m.start(), m.end(), m.value()))
                .collect()
        }))
    }

    /// Returns a list of overlapping matches without suffixes in the given haystack iterator.
    ///
    /// The Aho-Corasick algorithm reads through the haystack from left to right and reports
    /// matches when it reaches the end of each pattern. In the overlapping match, more than one
    /// pattern can be returned per report.
    ///
    /// This function returns the first match on each report.
    ///
    /// Examples:
    ///     >>> import daachorse
    ///     >>> patterns = [b'bcd', b'cd', b'abc']
    ///     >>> pma = daachorse.DoubleArrayAhoCorasick(patterns)
    ///     >>> pma.find_overlapping_no_suffix(b'abcd')
    ///     [(0, 3, 2), (1, 4, 0)]
    ///
    /// :param haystack: Bytes to search for.
    /// :type haystack: bytes
    /// :return: A list of matches. Each match is a tuple consisting of the start position, end
    ///          position, and pattern ID.
    /// :rtype: list[tuple[int, int, int]]
    /// :raises ValueError: if the automaton is built with a LEFTMOST option.
    #[pyo3(signature = (haystack, /))]
    fn find_overlapping_no_suffix(
        &self,
        py: Python,
        haystack: &[u8],
    ) -> PyResult<Vec<(usize, usize, u32)>> {
        if self.pma.match_kind() != ::daachorse::MatchKind::Standard {
            return Err(PyValueError::new_err("match_kind must be STANDARD"));
        }
        Ok(py.detach(|| {
            self.pma
                .find_overlapping_no_suffix_iter(haystack)
                .map(|m| (m.start(), m.end(), m.value()))
                .collect()
        }))
    }

    /// Serializes the automaton into a bytes.
    ///
    /// Examples:
    ///     >>> import daachorse
    ///     >>> patterns = [b'bcd', b'cd', b'abc']
    ///     >>> pma = daachorse.DoubleArrayAhoCorasick(patterns)
    ///     >>> b = pma.serialize()
    ///
    /// :return: Serialized automaton.
    /// :rtype: bytes
    #[pyo3(signature = (/))]
    fn serialize(&self, py: Python) -> Vec<u8> {
        py.detach(|| self.pma.serialize())
    }

    /// Deserializes the automaton from the given slice.
    ///
    /// Examples:
    ///     >>> import daachorse
    ///     >>> patterns = [b'bcd', b'cd', b'abc']
    ///     >>> pma = daachorse.DoubleArrayAhoCorasick(patterns)
    ///     >>> b = pma.serialize()
    ///     >>> pma = daachorse.DoubleArrayAhoCorasick.deserialize(b)
    ///
    /// :param data: Bytes to deserialize.
    /// :type data: bytes
    /// :return: A deserialized automaton.
    /// :rtype: daachorse.DoubleArrayAhoCorasick
    /// :raises ValueError: if the automaton is invalid.
    #[staticmethod]
    #[pyo3(signature = (data, /))]
    fn deserialize(py: Python, data: &[u8]) -> PyResult<Self> {
        Ok(Self {
            pma: py
                .detach(|| ::daachorse::DoubleArrayAhoCorasick::deserialize(data))
                .map_err(|e| PyValueError::new_err(e.to_string()))?
                .0,
        })
    }

    pub fn __getstate__(&self, py: Python) -> Vec<u8> {
        py.detach(|| self.pma.serialize())
    }

    pub fn __setstate__(&mut self, py: Python, data: &[u8]) -> PyResult<()> {
        self.pma = py
            .detach(|| ::daachorse::DoubleArrayAhoCorasick::deserialize(data))
            .map_err(|e| PyValueError::new_err(e.to_string()))?
            .0;
        Ok(())
    }

    pub fn __getnewargs__(&self) -> PyResult<([[u8; 1]; 1],)> {
        Ok(([[0]],))
    }
}

/// A character-wise Aho-Corasick automaton using the compact double-array data structure.
///
/// Examples:
///     >>> import daachorse
///     >>> patterns = ['bcd', 'ab', 'a']
///     >>> pma = daachorse.CharwiseDoubleArrayAhoCorasick(patterns)
///     >>> pma.find('abcd')
///     [(0, 1, 2), (1, 4, 0)]
///
/// :param patterns: List of string patterns.
/// :param match_kind: A search option of the Aho-Corasick automaton.
/// :type patterns: list[str]
/// :type match_kind: int
/// :rtype: daachorse.CharwiseDoubleArrayAhoCorasick
#[pyclass(module = "daachorse")]
struct CharwiseDoubleArrayAhoCorasick {
    pma: ::daachorse::CharwiseDoubleArrayAhoCorasick<u32>,
}

#[pymethods]
impl CharwiseDoubleArrayAhoCorasick {
    #[new]
    #[pyo3(signature = (patterns, /, match_kind = 0))]
    fn new(py: Python, patterns: Vec<Py<PyString>>, match_kind: u8) -> PyResult<Self> {
        let raw_patterns: PyResult<Vec<String>> =
            patterns.iter().map(|pat| pat.extract(py)).collect();
        let raw_patterns = raw_patterns?;
        let match_kind = ::daachorse::MatchKind::from(match_kind);
        Ok(Self {
            pma: py
                .detach(|| {
                    ::daachorse::CharwiseDoubleArrayAhoCorasickBuilder::new()
                        .match_kind(match_kind)
                        .build(raw_patterns)
                })
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        })
    }

    /// Returns a list of non-overlapping matches in the given haystack.
    ///
    /// Example 1: Standard semantics
    ///     >>> import daachorse
    ///     >>> patterns = ['bcd', 'ab', 'a']
    ///     >>> pma = daachorse.CharwiseDoubleArrayAhoCorasick(patterns)
    ///     >>> pma.find('abcd')
    ///     [(0, 1, 2), (1, 4, 0)]
    ///
    /// Example 2: Leftmost longest semantics
    ///     >>> import daachorse
    ///     >>> patterns = ['ab', 'a', 'abcd']
    ///     >>> pma = daachorse.CharwiseDoubleArrayAhoCorasick(patterns, daachorse.MATCH_KIND_LEFTMOST_LONGEST)
    ///     >>> pma.find('abcd')
    ///     [(0, 4, 2)]
    ///
    /// Example 3: Leftmost first semantics
    ///     >>> import daachorse
    ///     >>> patterns = ['ab', 'a', 'abcd']
    ///     >>> pma = daachorse.CharwiseDoubleArrayAhoCorasick(patterns, daachorse.MATCH_KIND_LEFTMOST_FIRST)
    ///     >>> pma.find('abcd')
    ///     [(0, 2, 0)]
    ///
    /// :param haystack: String to search for.
    /// :type haystack: str
    /// :return: A list of matches. Each match is a tuple consisting of the start position, end
    ///          position, and pattern ID.
    /// :rtype: list[tuple[int, int, int]]
    #[pyo3(signature = (haystack, /))]
    fn find(&self, py: Python, haystack: &str) -> PyResult<Vec<(usize, usize, u32)>> {
        let mut pos_map = vec![0; haystack.len() + 1];
        let mut len_in_chars = 0;
        let match_kind = self.pma.match_kind();
        Ok(py.detach(|| {
            unsafe {
                for (i, (j, _)) in haystack.char_indices().enumerate() {
                    debug_assert!(j < pos_map.len());
                    *pos_map.get_unchecked_mut(j) = i;
                    len_in_chars = i;
                }
                *pos_map.last_mut().unwrap_unchecked() = len_in_chars + 1;
            }
            match match_kind {
                ::daachorse::MatchKind::Standard => self
                    .pma
                    .find_iter(haystack)
                    .map(|m| {
                        (
                            pos_map.get(m.start()).copied().unwrap_or_default(),
                            pos_map.get(m.end()).copied().unwrap_or_default(),
                            m.value(),
                        )
                    })
                    .collect(),
                ::daachorse::MatchKind::LeftmostLongest | ::daachorse::MatchKind::LeftmostFirst => {
                    self.pma
                        .leftmost_find_iter(haystack)
                        .map(|m| {
                            (
                                pos_map.get(m.start()).copied().unwrap_or_default(),
                                pos_map.get(m.end()).copied().unwrap_or_default(),
                                m.value(),
                            )
                        })
                        .collect()
                }
            }
        }))
    }

    /// Returns a list of overlapping matches in the given haystack.
    ///
    /// Examples:
    ///     >>> import daachorse
    ///     >>> patterns = ['bcd', 'ab', 'a']
    ///     >>> pma = daachorse.CharwiseDoubleArrayAhoCorasick(patterns)
    ///     >>> pma.find_overlapping('abcd')
    ///     [(0, 1, 2), (0, 2, 1), (1, 4, 0)]
    ///
    /// :param haystack: String to search for.
    /// :type haystack: str
    /// :return: A list of matches. Each match is a tuple consisting of the start position, end
    ///          position, and pattern ID.
    /// :rtype: list[tuple[int, int, int]]
    /// :raises ValueError: if the automaton is built with a LEFTMOST option.
    #[pyo3(signature = (haystack, /))]
    fn find_overlapping(&self, py: Python, haystack: &str) -> PyResult<Vec<(usize, usize, u32)>> {
        if self.pma.match_kind() != ::daachorse::MatchKind::Standard {
            return Err(PyValueError::new_err("match_kind must be STANDARD"));
        }
        Ok(py.detach(|| {
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
            self.pma
                .find_overlapping_iter(haystack)
                .map(|m| {
                    (
                        pos_map.get(m.start()).copied().unwrap_or_default(),
                        pos_map.get(m.end()).copied().unwrap_or_default(),
                        m.value(),
                    )
                })
                .collect()
        }))
    }

    /// Returns a list of overlapping matches without suffixes in the given haystack iterator.
    ///
    /// The Aho-Corasick algorithm reads through the haystack from left to right and reports
    /// matches when it reaches the end of each pattern. In the overlapping match, more than one
    /// pattern can be returned per report.
    ///
    /// This function returns the first match on each report.
    ///
    /// Examples:
    ///     >>> import daachorse
    ///     >>> patterns = ['bcd', 'cd', 'abc']
    ///     >>> pma = daachorse.CharwiseDoubleArrayAhoCorasick(patterns)
    ///     >>> pma.find_overlapping_no_suffix('abcd')
    ///     [(0, 3, 2), (1, 4, 0)]
    ///
    /// :param haystack: String to search for.
    /// :type haystack: str
    /// :return: A list of matches. Each match is a tuple consisting of the start position, end
    ///          position, and pattern ID.
    /// :rtype: list[tuple[int, int, int]]
    /// :raises ValueError: if the automaton is built with a LEFTMOST option.
    #[pyo3(signature = (haystack, /))]
    fn find_overlapping_no_suffix(
        &self,
        py: Python,
        haystack: &str,
    ) -> PyResult<Vec<(usize, usize, u32)>> {
        if self.pma.match_kind() != ::daachorse::MatchKind::Standard {
            return Err(PyValueError::new_err("match_kind must be STANDARD"));
        }
        Ok(py.detach(|| {
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
            self.pma
                .find_overlapping_no_suffix_iter(haystack)
                .map(|m| {
                    (
                        pos_map.get(m.start()).copied().unwrap_or_default(),
                        pos_map.get(m.end()).copied().unwrap_or_default(),
                        m.value(),
                    )
                })
                .collect()
        }))
    }

    /// Serializes the automaton into a bytes.
    ///
    /// Examples:
    ///     >>> import daachorse
    ///     >>> patterns = ['bcd', 'cd', 'abc']
    ///     >>> pma = daachorse.CharwiseDoubleArrayAhoCorasick(patterns)
    ///     >>> b = pma.serialize()
    ///
    /// :return: Serialized automaton.
    /// :rtype: bytes
    #[pyo3(signature = (/))]
    fn serialize(&self, py: Python) -> Vec<u8> {
        py.detach(|| self.pma.serialize())
    }

    /// Deserializes the automaton from the given slice.
    ///
    /// Examples:
    ///     >>> import daachorse
    ///     >>> patterns = ['bcd', 'cd', 'abc']
    ///     >>> pma = daachorse.CharwiseDoubleArrayAhoCorasick(patterns)
    ///     >>> b = pma.serialize()
    ///     >>> pma = daachorse.CharwiseDoubleArrayAhoCorasick.deserialize(b)
    ///
    /// :param data: Bytes to deserialize.
    /// :type data: bytes
    /// :return: A deserialized automaton.
    /// :rtype: daachorse.CharwiseDoubleArrayAhoCorasick
    /// :raises ValueError: if the automaton is invalid.
    #[staticmethod]
    #[pyo3(signature = (data, /))]
    fn deserialize(py: Python, data: &[u8]) -> PyResult<Self> {
        Ok(Self {
            pma: py
                .detach(|| ::daachorse::CharwiseDoubleArrayAhoCorasick::deserialize(data))
                .map_err(|e| PyValueError::new_err(e.to_string()))?
                .0,
        })
    }

    pub fn __getstate__(&self, py: Python) -> Vec<u8> {
        py.detach(|| self.pma.serialize())
    }

    pub fn __setstate__(&mut self, py: Python, data: &[u8]) -> PyResult<()> {
        self.pma = py
            .detach(|| ::daachorse::CharwiseDoubleArrayAhoCorasick::deserialize(data))
            .map_err(|e| PyValueError::new_err(e.to_string()))?
            .0;
        Ok(())
    }

    pub fn __getnewargs__(&self) -> PyResult<([&'static str; 1],)> {
        Ok(([" "],))
    }
}

#[pymodule]
fn daachorse(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DoubleArrayAhoCorasick>()?;
    m.add_class::<CharwiseDoubleArrayAhoCorasick>()?;
    m.add(
        "MATCH_KIND_STANDARD",
        ::daachorse::MatchKind::Standard as u8,
    )?;
    m.add(
        "MATCH_KIND_LEFTMOST_LONGEST",
        ::daachorse::MatchKind::LeftmostLongest as u8,
    )?;
    m.add(
        "MATCH_KIND_LEFTMOST_FIRST",
        ::daachorse::MatchKind::LeftmostFirst as u8,
    )?;
    Ok(())
}
