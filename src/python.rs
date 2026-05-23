use std::cmp::Ordering;

use pyo3::prelude::*;
use pyo3::types::PyType;

/// An RPM version specifier: Epoch, Version, Release.
///
/// Supports ordering via RPM's version comparison algorithm. See also the
/// module-level `evr_compare` function for comparing raw EVR strings.
#[pyclass(name = "Evr", frozen, eq, ord, hash, from_py_object)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[pyclass(name = "Nevra", frozen, eq, ord, hash, from_py_object)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
// Requirement
// ---------------------------------------------------------------------------

/// Comparison operator for an RPM dependency requirement.
#[pyclass(name = "ReqOperator", frozen, eq, eq_int, hash, from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyReqOperator {
    LT,
    LE,
    EQ,
    GE,
    GT,
}

impl From<PyReqOperator> for crate::ReqOperator {
    fn from(op: PyReqOperator) -> Self {
        match op {
            PyReqOperator::LT => crate::ReqOperator::LT,
            PyReqOperator::LE => crate::ReqOperator::LE,
            PyReqOperator::EQ => crate::ReqOperator::EQ,
            PyReqOperator::GE => crate::ReqOperator::GE,
            PyReqOperator::GT => crate::ReqOperator::GT,
        }
    }
}

impl From<crate::ReqOperator> for PyReqOperator {
    fn from(op: crate::ReqOperator) -> Self {
        match op {
            crate::ReqOperator::LT => PyReqOperator::LT,
            crate::ReqOperator::LE => PyReqOperator::LE,
            crate::ReqOperator::EQ => PyReqOperator::EQ,
            crate::ReqOperator::GE => PyReqOperator::GE,
            crate::ReqOperator::GT => PyReqOperator::GT,
        }
    }
}

/// Accepts either a string (`"<"`, `"<="`, `"="`, `">="`, `">"`) or a `ReqOperator` enum value.
#[derive(FromPyObject)]
enum OpArg {
    Enum(PyReqOperator),
    Str(String),
}

impl OpArg {
    fn into_req_operator(self) -> PyResult<crate::ReqOperator> {
        match self {
            OpArg::Enum(e) => Ok(e.into()),
            OpArg::Str(s) => match s.as_str() {
                "<" | "LT" => Ok(crate::ReqOperator::LT),
                "<=" | "LE" => Ok(crate::ReqOperator::LE),
                "=" | "==" | "EQ" => Ok(crate::ReqOperator::EQ),
                ">=" | "GE" => Ok(crate::ReqOperator::GE),
                ">" | "GT" => Ok(crate::ReqOperator::GT),
                _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "invalid operator: {s:?} (expected <, <=, =, >=, or >)"
                ))),
            },
        }
    }
}

/// An RPM dependency requirement: a package name with an optional version constraint.
///
/// A requirement like ``Requirement("foo", ">=", Evr.parse("2.0-1"))`` is satisfied
/// by any package named ``foo`` whose EVR is at least ``2.0-1``.
/// A requirement with no constraint (just a name) is satisfied by any version.
#[pyclass(name = "Requirement", frozen, eq, hash, from_py_object)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PyRequirement(crate::Requirement<'static>);

#[pymethods]
impl PyRequirement {
    /// Construct a requirement.
    ///
    /// With just a name, any version satisfies.
    /// With an operator and EVR, only matching versions satisfy.
    #[new]
    #[pyo3(signature = (name, op=None, evr=None))]
    fn new(name: &str, op: Option<OpArg>, evr: Option<PyEvr>) -> PyResult<Self> {
        match (op, evr) {
            (None, None) => Ok(PyRequirement(crate::Requirement::new(name.to_owned()))),
            (Some(op_arg), Some(py_evr)) => {
                let op = op_arg.into_req_operator()?;
                let (epoch, version, release) = py_evr.0.values();
                let evr = crate::Evr::new(epoch.to_owned(), version.to_owned(), release.to_owned());
                Ok(PyRequirement(crate::Requirement::with_constraint(
                    name.to_owned(),
                    op,
                    evr,
                )))
            }
            _ => Err(pyo3::exceptions::PyValueError::new_err(
                "op and evr must both be provided, or both omitted",
            )),
        }
    }

    /// The required package name.
    #[getter]
    fn name(&self) -> &str {
        self.0.name()
    }

    /// The version constraint as (ReqOperator, Evr), or None.
    #[getter]
    fn constraint(&self) -> Option<(PyReqOperator, PyEvr)> {
        self.0
            .constraint()
            .map(|(op, evr)| (PyReqOperator::from(op), PyEvr::from(evr.clone())))
    }

    /// Check whether a given package name and EVR satisfy this requirement.
    fn satisfies(&self, name: &str, evr: &PyEvr) -> bool {
        self.0.satisfies(name, &evr.0)
    }

    fn __repr__(&self) -> String {
        format!("Requirement({})", self.0)
    }

    fn __str__(&self) -> String {
        self.0.to_string()
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
    m.add_class::<PyReqOperator>()?;
    m.add_class::<PyRequirement>()?;
    m.add_function(wrap_pyfunction!(evr_compare, m)?)?;

    Ok(())
}
