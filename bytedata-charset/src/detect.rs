
/// Confidence level of the charset detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[expect(clippy::exhaustive_enums)]
pub enum Confidence {
    /// The charset is detected with a certain confidence.
    Certain,
    /// The charset is detected with a tentative confidence.
    /// Nothing indicates that this is the correct charset, but nothing indicates that it is not.
    Tentative,
    /// The charset is irrelevant to the data.
    Irrelevant,
}

/// A detector that checks if a charset is a possible match for the given byte sequence.
pub trait CharsetDetector {
    /// Detects the presence of a BOM or reasonable bytes in the given byte sequence.
    fn detect(&self, bytes: &[u8]) -> DetectionResult;
}

/// The result of a charset detection test.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[expect(clippy::exhaustive_enums)]
pub enum DetectionResult {
    /// The charset is detected with a certain confidence.
    Certain,
    /// The charset is detected with a tentative confidence.
    /// Nothing guarantees that this is the correct charset, but nothing indicates that it is not.
    Tentative,
    /// The charset is irrelevant to the data.
    Irrelevant,
    /// The data input was too short to determine the charset.
    Incomplete,
}

impl DetectionResult {
    /// Returns true if the charset is detected with a certain confidence.
    #[inline]
    #[must_use]
    pub const fn is_certain(self) -> bool {
        matches!(self, Self::Certain)
    }

    /// Returns true if the charset is detected with a tentative confidence.
    #[inline]
    #[must_use]
    pub const fn is_tentative(self) -> bool {
        matches!(self, Self::Tentative)
    }

    /// Returns true if the charset is irrelevant to the data.
    #[inline]
    #[must_use]
    pub const fn is_irrelevant(self) -> bool {
        matches!(self, Self::Irrelevant)
    }

    /// Returns true if the data input was too short to determine the charset.
    #[inline]
    #[must_use]
    pub const fn is_incomplete(self) -> bool {
        matches!(self, Self::Incomplete)
    }

    /// Returns a result based on the confidence level.
    #[inline]
    #[must_use]
    pub const fn from_confidence(confidence: crate::detect::Confidence) -> Self {
        match confidence {
            crate::detect::Confidence::Certain => Self::Certain,
            crate::detect::Confidence::Tentative => Self::Tentative,
            crate::detect::Confidence::Irrelevant => Self::Irrelevant,
        }
    }

    /// Returns an optional confidence level.
    #[inline]
    #[must_use]
    pub const fn to_confidence(self) -> Option<crate::detect::Confidence> {
        match self {
            Self::Certain => Some(crate::detect::Confidence::Certain),
            Self::Tentative => Some(crate::detect::Confidence::Tentative),
            Self::Irrelevant => Some(crate::detect::Confidence::Irrelevant),
            Self::Incomplete => None,
        }
    }

    /// Returns the most certain result.
    #[inline]
    #[must_use]
    pub const fn min(self, other: Self) -> Self {
        match (self, other) {
            (Self::Certain, _) | (_, Self::Certain) => Self::Certain,
            (Self::Tentative, _) | (_, Self::Tentative) => Self::Tentative,
            (Self::Irrelevant, _) | (_, Self::Irrelevant) => Self::Irrelevant,
            (Self::Incomplete, Self::Incomplete) => Self::Incomplete,
        }
    }

    /// Returns `true` if `self` is more certain than `other`.
    #[inline]
    #[must_use]
    pub const fn le(self, other: Self) -> bool {
        match (self, other) {
            (Self::Certain, _) => true,
            (_, Self::Certain) => false,
            (Self::Tentative, _) => true,
            (_, Self::Tentative) => false,
            (Self::Irrelevant, _) => true,
            (_, Self::Irrelevant) => false,
            (Self::Incomplete, _) => true,
        }
    }
}

impl From<crate::detect::Confidence> for DetectionResult {
    #[inline]
    #[must_use]
    fn from(confidence: crate::detect::Confidence) -> Self {
        Self::from_confidence(confidence)
    }
}

/// A charset detector that can select the most likely charset from the given byte sequence.
pub trait CharsetSelector: CharsetDetector {
    /// The charset type that is selected
    type Charset: crate::CharsetRef;

    /// Selects the most likely charset from the given byte sequence.
    #[must_use]
    fn select(&self, bytes: &[u8]) -> (Self::Charset, DetectionResult);
}

impl<T: crate::CharsetRef + CharsetDetector + Copy> CharsetSelector for T {
    type Charset = T;

    #[inline]
    #[must_use]
    fn select(&self, bytes: &[u8]) -> (Self::Charset, DetectionResult) {
        (*self, self.detect(bytes))
    }
}

/// A charset detector that can select the most likely UTF charset from the given byte sequence.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UtfDetector;

impl UtfDetector {
    /// Detect the if the bytes are UTF encoded.
    #[must_use]
    #[allow(clippy::missing_inline_in_public_items)]
    pub const fn detect_const(bytes: &[u8]) -> DetectionResult {
        let x = [
            crate::Utf8Encoding::detect_const(bytes),
            crate::Utf16Encoding::UTF16_BE.detect_const(bytes),
            crate::Utf16Encoding::UTF16_LE.detect_const(bytes),
            #[cfg(feature = "utf-32")]
            crate::Utf32Encoding::UTF32_BE.detect_const(bytes),
            #[cfg(feature = "utf-32")]
            crate::Utf32Encoding::UTF32_BE.detect_const(bytes),
        ];
        let mut pos = 0;
        let mut idx = 1;
        while idx < x.len() {
            if !x[idx].le(x[pos]) {
                pos = idx;
            }
            idx += 1;
        }
        x[pos]
    }

    /// Select the most likely UTF charset from the given byte sequence.
    #[must_use]
    #[allow(clippy::missing_inline_in_public_items)]
    pub const fn select_const(bytes: &[u8]) -> (&'static (dyn crate::CharsetRef), DetectionResult) {
        let x = [
            (&crate::Utf8Encoding as &'static dyn crate::CharsetRef, crate::Utf8Encoding::detect_const(bytes)),
            (&crate::Utf16Encoding::UTF16_BE as &'static dyn crate::CharsetRef, crate::Utf16Encoding::UTF16_BE.detect_const(bytes)),
            (&crate::Utf16Encoding::UTF16_LE as &'static dyn crate::CharsetRef, crate::Utf16Encoding::UTF16_LE.detect_const(bytes)),
            #[cfg(feature = "utf-32")]
            (&crate::Utf32Encoding::UTF32_BE as &'static dyn crate::CharsetRef, crate::Utf32Encoding::UTF32_BE.detect_const(bytes)),
            #[cfg(feature = "utf-32")]
            (&crate::Utf32Encoding::UTF32_LE as &'static dyn crate::CharsetRef, crate::Utf32Encoding::UTF32_LE.detect_const(bytes)),
        ];
        
        let mut pos = 0;
        let mut idx = 1;
        while idx < x.len() {
            if !x[idx].1.le(x[pos].1) {
                pos = idx;
            }
            idx += 1;
        }
        x[pos]
    }
}

impl CharsetDetector for UtfDetector {
    #[must_use]
    #[inline]
    fn detect(&self, bytes: &[u8]) -> DetectionResult {
        Self::detect_const(bytes)
    }
}

impl CharsetSelector for UtfDetector {
    type Charset = &'static (dyn crate::CharsetRef);

    #[must_use]
    #[inline]
    fn select(&self, bytes: &[u8]) -> (Self::Charset, DetectionResult) {
        Self::select_const(bytes)
    }
}
