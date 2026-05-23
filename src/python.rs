use std::cmp::Ordering;

use pyo3::prelude::*;
use pyo3::types::PyType;

/// An RPM version specifier: Epoch, Version, Release.
///
/// Supports ordering via RPM's version comparison algorithm. See also the
/// module-level `evr_compare` function for comparing raw EVR strings.
#[pyclass(name = "Evr", eq, ord, from_py_object)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PyEvr(crate::Evr<'static>);

impl<'a> From<crate::Evr<'a>> for PyEvr {
    fn from(e: crate::Evr<'a>) -> Self {
        let (epoch, version, release) = e.values();
        PyEvr(crate::Evr::new(
            epoch.to_owned(),
            version.to_owned(),
            release.to_owned(),
        ))
    }
}

#[pymethods]
impl PyEvr {
    /// Construct an Evr from its three components.
    #[new]
    fn new(epoch: &str, version: &str, release: &str) -> Self {
        PyEvr(crate::Evr::new(
            epoch.to_owned(),
            version.to_owned(),
            release.to_owned(),
        ))
    }

    /// Parse an EVR string such as `"2.3.4-5"` or `"1:2.3.4-5"`.
    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, evr: &str) -> Self {
        // parse_values returns slices that borrow from `evr`; convert to owned immediately.
        let (epoch, version, release) = crate::Evr::parse_values(evr);
        PyEvr(crate::Evr::new(
            epoch.to_owned(),
            version.to_owned(),
            release.to_owned(),
        ))
    }

    fn __repr__(&self) -> String {
        format!(
            "Evr(epoch={:?}, version={:?}, release={:?})",
            self.0.epoch(),
            self.0.version(),
            self.0.release(),
        )
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    /// The epoch string. Empty string means no epoch (equivalent to epoch 0).
    #[getter]
    fn epoch(&self) -> &str {
        self.0.epoch()
    }

    /// The version string.
    #[getter]
    fn version(&self) -> &str {
        self.0.version()
    }

    /// The release string.
    #[getter]
    fn release(&self) -> &str {
        self.0.release()
    }

    /// Write the EVR in normalized form, always including the epoch (e.g. `"0:1.2.3-4"`).
    fn as_normalized_form(&self) -> String {
        self.0.as_normalized_form()
    }
}

// ---------------------------------------------------------------------------
// Nevra
// ---------------------------------------------------------------------------

/// A full RPM NEVRA: Name, Epoch, Version, Release, Architecture.
///
/// Supports ordering via RPM's version comparison algorithm.
#[pyclass(name = "Nevra", eq, ord, from_py_object)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PyNevra(crate::Nevra<'static>);

impl<'a> From<crate::Nevra<'a>> for PyNevra {
    fn from(n: crate::Nevra<'a>) -> Self {
        let (name, epoch, version, release, arch) = n.values();
        PyNevra(crate::Nevra::new(
            name.to_owned(),
            epoch.to_owned(),
            version.to_owned(),
            release.to_owned(),
            arch.to_owned(),
        ))
    }
}

#[pymethods]
impl PyNevra {
    /// Construct a Nevra from its five components.
    #[new]
    fn new(name: &str, epoch: &str, version: &str, release: &str, arch: &str) -> Self {
        PyNevra(crate::Nevra::new(
            name.to_owned(),
            epoch.to_owned(),
            version.to_owned(),
            release.to_owned(),
            arch.to_owned(),
        ))
    }

    /// Parse a NEVRA string such as `"foo-1.2.3-4.x86_64"` or `"foo-1:1.2.3-4.x86_64"`.
    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, nevra: &str) -> Self {
        // parse_values returns slices that borrow from `nevra`; convert to owned immediately.
        let (name, epoch, version, release, arch) = crate::Nevra::parse_values(nevra);
        PyNevra(crate::Nevra::new(
            name.to_owned(),
            epoch.to_owned(),
            version.to_owned(),
            release.to_owned(),
            arch.to_owned(),
        ))
    }

    fn __repr__(&self) -> String {
        format!(
            "Nevra(name={:?}, epoch={:?}, version={:?}, release={:?}, arch={:?})",
            self.0.name(),
            self.0.epoch(),
            self.0.version(),
            self.0.release(),
            self.0.arch(),
        )
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    /// The package name.
    #[getter]
    fn name(&self) -> &str {
        self.0.name()
    }

    /// The epoch string. Empty string means no epoch (equivalent to epoch 0).
    #[getter]
    fn epoch(&self) -> &str {
        self.0.epoch()
    }

    /// The version string.
    #[getter]
    fn version(&self) -> &str {
        self.0.version()
    }

    /// The release string.
    #[getter]
    fn release(&self) -> &str {
        self.0.release()
    }

    /// The architecture string (e.g. `"x86_64"`, `"noarch"`).
    #[getter]
    fn arch(&self) -> &str {
        self.0.arch()
    }

    /// The EVR (Epoch, Version, Release) portion of this NEVRA as an `Evr` object.
    fn evr(&self) -> PyEvr {
        // evr() has &'a self receiver so we can't use it here; extract fields via the
        // plain &self accessors (epoch/version/release) and build a fresh Evr<'static>.
        PyEvr(crate::Evr::new(
            self.0.epoch().to_owned(),
            self.0.version().to_owned(),
            self.0.release().to_owned(),
        ))
    }

    /// Write the NEVRA in normalized form, always including the epoch
    /// (e.g. `"foo-0:1.2.3-4.x86_64"`).
    fn as_normalized_form(&self) -> String {
        self.0.as_normalized_form()
    }

    /// Write an NVRA string (no epoch), typically used for RPM filenames
    /// (e.g. `"foo-1.2.3-4.x86_64"`).
    fn nvra(&self) -> String {
        self.0.nvra()
    }
}

// ---------------------------------------------------------------------------
// Module-level functions
// ---------------------------------------------------------------------------

/// Compare two EVR strings using RPM's version comparison algorithm.
///
/// Returns -1, 0, or 1 if `evr1` is less than, equal to, or greater than `evr2`.
///
/// # Example
/// ```python
/// assert evr_compare("1.2.3-4", "1.2.3-5") == -1
/// assert evr_compare("2:1.0-1", "1:9.9-1") == 1
/// ```
#[pyfunction]
fn evr_compare(evr1: &str, evr2: &str) -> i32 {
    match crate::rpm_evr_compare(evr1, evr2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

// ---------------------------------------------------------------------------
// Module registration
// ---------------------------------------------------------------------------

/// Python module exporting rpm_version functionality.
///
/// Register all public types with the Python interpreter.
#[pymodule]
pub fn rpm_version(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyEvr>()?;
    m.add_class::<PyNevra>()?;
    m.add_function(wrap_pyfunction!(evr_compare, m)?)?;

    Ok(())
}
