use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use magnus::prelude::*;
use magnus::{Error, RString, Ruby, function, method};

fn ruby_hash<T: Hash>(value: &T) -> i64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish() as i64
}

fn ordering_value(ordering: Ordering) -> i32 {
    match ordering {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

#[magnus::wrap(class = "RPMVersion::Evr", free_immediately)]
#[derive(Clone)]
struct RubyEvr {
    inner: crate::Evr<'static>,
}

impl RubyEvr {
    fn new(epoch: Option<String>, version: String, release: String) -> Self {
        RubyEvr {
            inner: crate::Evr::new(epoch.unwrap_or_default(), version, release),
        }
    }

    fn parse(evr: String) -> Self {
        let (epoch, version, release) = crate::Evr::parse_values(&evr);
        RubyEvr::new(
            Some(epoch.to_owned()),
            version.to_owned(),
            release.to_owned(),
        )
    }

    fn epoch(&self) -> &str {
        self.inner.epoch()
    }

    fn version(&self) -> &str {
        self.inner.version()
    }

    fn release(&self) -> &str {
        self.inner.release()
    }

    fn evr(&self) -> String {
        self.inner.evr()
    }

    fn evr_short(&self) -> String {
        self.inner.to_string()
    }

    fn sortkey(ruby: &Ruby, this: &Self) -> RString {
        ruby.str_from_slice(this.inner.sortkey().as_bytes())
    }

    fn to_s(&self) -> String {
        self.inner.to_string()
    }

    fn inspect(&self) -> String {
        format!(
            "#<RPMVersion::Evr epoch={:?} version={:?} release={:?}>",
            self.epoch(),
            self.version(),
            self.release()
        )
    }

    fn ruby_cmp(&self, other: &Self) -> i32 {
        ordering_value(self.inner.cmp(&other.inner))
    }

    fn ruby_eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn ruby_lt(&self, other: &Self) -> bool {
        self.inner < other.inner
    }

    fn ruby_le(&self, other: &Self) -> bool {
        self.inner <= other.inner
    }

    fn ruby_gt(&self, other: &Self) -> bool {
        self.inner > other.inner
    }

    fn ruby_ge(&self, other: &Self) -> bool {
        self.inner >= other.inner
    }

    fn hash(&self) -> i64 {
        ruby_hash(&self.inner)
    }
}

#[magnus::wrap(class = "RPMVersion::Nevra", free_immediately)]
#[derive(Clone)]
struct RubyNevra {
    inner: crate::Nevra<'static>,
}

impl RubyNevra {
    fn new(
        name: String,
        epoch: Option<String>,
        version: String,
        release: String,
        arch: String,
    ) -> Self {
        RubyNevra {
            inner: crate::Nevra::new(name, epoch.unwrap_or_default(), version, release, arch),
        }
    }

    fn parse(nevra: String) -> Self {
        let (name, epoch, version, release, arch) = crate::Nevra::parse_values(&nevra);
        RubyNevra::new(
            name.to_owned(),
            Some(epoch.to_owned()),
            version.to_owned(),
            release.to_owned(),
            arch.to_owned(),
        )
    }

    fn name(&self) -> &str {
        self.inner.name()
    }

    fn epoch(&self) -> &str {
        self.inner.epoch()
    }

    fn version(&self) -> &str {
        self.inner.version()
    }

    fn release(&self) -> &str {
        self.inner.release()
    }

    fn arch(&self) -> &str {
        self.inner.arch()
    }

    fn evr(&self) -> RubyEvr {
        RubyEvr::new(
            Some(self.epoch().to_owned()),
            self.version().to_owned(),
            self.release().to_owned(),
        )
    }

    fn nevra(&self) -> String {
        self.inner.nevra()
    }

    fn nevra_short(&self) -> String {
        self.inner.nevra_short()
    }

    fn nvra(&self) -> String {
        self.inner.nvra()
    }

    fn to_s(&self) -> String {
        self.inner.to_string()
    }

    fn inspect(&self) -> String {
        format!(
            "#<RPMVersion::Nevra name={:?} epoch={:?} version={:?} release={:?} arch={:?}>",
            self.name(),
            self.epoch(),
            self.version(),
            self.release(),
            self.arch()
        )
    }

    fn ruby_cmp(&self, other: &Self) -> i32 {
        ordering_value(self.inner.cmp(&other.inner))
    }

    fn ruby_eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn ruby_lt(&self, other: &Self) -> bool {
        self.inner < other.inner
    }

    fn ruby_le(&self, other: &Self) -> bool {
        self.inner <= other.inner
    }

    fn ruby_gt(&self, other: &Self) -> bool {
        self.inner > other.inner
    }

    fn ruby_ge(&self, other: &Self) -> bool {
        self.inner >= other.inner
    }

    fn hash(&self) -> i64 {
        ruby_hash(&self.inner)
    }
}

#[magnus::wrap(class = "RPMVersion::Requirement", free_immediately)]
#[derive(Clone)]
struct RubyRequirement {
    inner: crate::Requirement<'static>,
}

impl RubyRequirement {
    fn new(
        ruby: &Ruby,
        name: String,
        op: Option<String>,
        evr: Option<&RubyEvr>,
    ) -> Result<Self, Error> {
        match (op, evr) {
            (None, None) => Ok(RubyRequirement {
                inner: crate::Requirement::new(name),
            }),
            (Some(op), Some(evr)) => {
                let op = match op.as_str() {
                    "<" | "LT" => crate::ReqOperator::LT,
                    "<=" | "LE" => crate::ReqOperator::LE,
                    "=" | "==" | "EQ" => crate::ReqOperator::EQ,
                    ">=" | "GE" => crate::ReqOperator::GE,
                    ">" | "GT" => crate::ReqOperator::GT,
                    _ => {
                        return Err(Error::new(
                            ruby.exception_arg_error(),
                            format!("invalid operator: {op:?} (expected <, <=, =, >=, or >)"),
                        ));
                    }
                };
                let (epoch, version, release) = evr.inner.values();
                let evr = crate::Evr::new(epoch.to_owned(), version.to_owned(), release.to_owned());
                Ok(RubyRequirement {
                    inner: crate::Requirement::with_constraint(name, op, evr),
                })
            }
            _ => Err(Error::new(
                ruby.exception_arg_error(),
                "op and evr must both be provided, or both omitted",
            )),
        }
    }

    fn name(&self) -> &str {
        self.inner.name()
    }

    fn constraint(&self) -> Option<(String, RubyEvr)> {
        self.inner.constraint().map(|(op, evr)| {
            let op = op.to_string();
            let (epoch, version, release) = evr.values();
            (
                op,
                RubyEvr::new(
                    Some(epoch.to_owned()),
                    version.to_owned(),
                    release.to_owned(),
                ),
            )
        })
    }

    fn satisfies(&self, name: String, evr: &RubyEvr) -> bool {
        self.inner.satisfies(&name, &evr.inner)
    }

    fn to_s(&self) -> String {
        self.inner.to_string()
    }

    fn inspect(&self) -> String {
        format!("#<RPMVersion::Requirement {}>", self.inner)
    }

    fn ruby_eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn hash(&self) -> i64 {
        ruby_hash(&self.inner)
    }
}

fn evr_compare(evr1: String, evr2: String) -> i32 {
    ordering_value(crate::rpm_evr_compare(&evr1, &evr2))
}

fn evr_sort(evrs: Vec<String>) -> Vec<String> {
    let mut parsed: Vec<(crate::Evr<'_>, usize)> = evrs
        .iter()
        .enumerate()
        .map(|(index, evr)| (crate::Evr::parse(evr), index))
        .collect();
    parsed.sort_unstable();
    parsed
        .into_iter()
        .map(|(_, index)| evrs[index].clone())
        .collect()
}

fn nevra_sort(nevras: Vec<String>) -> Vec<String> {
    let mut parsed: Vec<(crate::Nevra<'_>, usize)> = nevras
        .iter()
        .enumerate()
        .map(|(index, nevra)| (crate::Nevra::parse(nevra), index))
        .collect();
    parsed.sort_unstable();
    parsed
        .into_iter()
        .map(|(_, index)| nevras[index].clone())
        .collect()
}

fn evr_sortkey_from_values(
    ruby: &Ruby,
    epoch: Option<String>,
    version: String,
    release: String,
) -> RString {
    let key = crate::EvrSortKey::from_values(epoch.as_deref().unwrap_or(""), &version, &release);
    ruby.str_from_slice(key.as_bytes())
}

fn evr_sortkey_parse(ruby: &Ruby, evr: String) -> RString {
    let key = crate::EvrSortKey::parse(&evr);
    ruby.str_from_slice(key.as_bytes())
}

fn version_sortkey(ruby: &Ruby, version: String) -> RString {
    ruby.str_from_slice(&crate::version_sortkey(&version))
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("RPMVersion")?;

    let evr = module.define_class("Evr", ruby.class_object())?;
    evr.define_singleton_method("new", function!(RubyEvr::new, 3))?;
    evr.define_singleton_method("parse", function!(RubyEvr::parse, 1))?;
    evr.define_method("epoch", method!(RubyEvr::epoch, 0))?;
    evr.define_method("version", method!(RubyEvr::version, 0))?;
    evr.define_method("release", method!(RubyEvr::release, 0))?;
    evr.define_method("evr", method!(RubyEvr::evr, 0))?;
    evr.define_method("evr_short", method!(RubyEvr::evr_short, 0))?;
    evr.define_method("sortkey", method!(RubyEvr::sortkey, 0))?;
    evr.define_method("to_s", method!(RubyEvr::to_s, 0))?;
    evr.define_method("inspect", method!(RubyEvr::inspect, 0))?;
    evr.define_method("<=>", method!(RubyEvr::ruby_cmp, 1))?;
    evr.define_method("==", method!(RubyEvr::ruby_eq, 1))?;
    evr.define_method("eql?", method!(RubyEvr::ruby_eq, 1))?;
    evr.define_method("<", method!(RubyEvr::ruby_lt, 1))?;
    evr.define_method("<=", method!(RubyEvr::ruby_le, 1))?;
    evr.define_method(">", method!(RubyEvr::ruby_gt, 1))?;
    evr.define_method(">=", method!(RubyEvr::ruby_ge, 1))?;
    evr.define_method("hash", method!(RubyEvr::hash, 0))?;

    let nevra = module.define_class("Nevra", ruby.class_object())?;
    nevra.define_singleton_method("new", function!(RubyNevra::new, 5))?;
    nevra.define_singleton_method("parse", function!(RubyNevra::parse, 1))?;
    nevra.define_method("name", method!(RubyNevra::name, 0))?;
    nevra.define_method("epoch", method!(RubyNevra::epoch, 0))?;
    nevra.define_method("version", method!(RubyNevra::version, 0))?;
    nevra.define_method("release", method!(RubyNevra::release, 0))?;
    nevra.define_method("arch", method!(RubyNevra::arch, 0))?;
    nevra.define_method("evr", method!(RubyNevra::evr, 0))?;
    nevra.define_method("nevra", method!(RubyNevra::nevra, 0))?;
    nevra.define_method("nevra_short", method!(RubyNevra::nevra_short, 0))?;
    nevra.define_method("nvra", method!(RubyNevra::nvra, 0))?;
    nevra.define_method("to_s", method!(RubyNevra::to_s, 0))?;
    nevra.define_method("inspect", method!(RubyNevra::inspect, 0))?;
    nevra.define_method("<=>", method!(RubyNevra::ruby_cmp, 1))?;
    nevra.define_method("==", method!(RubyNevra::ruby_eq, 1))?;
    nevra.define_method("eql?", method!(RubyNevra::ruby_eq, 1))?;
    nevra.define_method("<", method!(RubyNevra::ruby_lt, 1))?;
    nevra.define_method("<=", method!(RubyNevra::ruby_le, 1))?;
    nevra.define_method(">", method!(RubyNevra::ruby_gt, 1))?;
    nevra.define_method(">=", method!(RubyNevra::ruby_ge, 1))?;
    nevra.define_method("hash", method!(RubyNevra::hash, 0))?;

    let requirement = module.define_class("Requirement", ruby.class_object())?;
    requirement.define_singleton_method("new", function!(RubyRequirement::new, 3))?;
    requirement.define_method("name", method!(RubyRequirement::name, 0))?;
    requirement.define_method("constraint", method!(RubyRequirement::constraint, 0))?;
    requirement.define_method("satisfies?", method!(RubyRequirement::satisfies, 2))?;
    requirement.define_method("to_s", method!(RubyRequirement::to_s, 0))?;
    requirement.define_method("inspect", method!(RubyRequirement::inspect, 0))?;
    requirement.define_method("==", method!(RubyRequirement::ruby_eq, 1))?;
    requirement.define_method("eql?", method!(RubyRequirement::ruby_eq, 1))?;
    requirement.define_method("hash", method!(RubyRequirement::hash, 0))?;

    let req_operator = module.define_module("ReqOperator")?;
    req_operator.const_set("LT", "<")?;
    req_operator.const_set("LE", "<=")?;
    req_operator.const_set("EQ", "=")?;
    req_operator.const_set("GE", ">=")?;
    req_operator.const_set("GT", ">")?;

    let sortkey = module.define_module("EvrSortKey")?;
    sortkey.define_module_function("from_values", function!(evr_sortkey_from_values, 3))?;
    sortkey.define_module_function("parse", function!(evr_sortkey_parse, 1))?;

    module.define_module_function("evr_compare", function!(evr_compare, 2))?;
    module.define_module_function("evr_sort", function!(evr_sort, 1))?;
    module.define_module_function("nevra_sort", function!(nevra_sort, 1))?;
    module.define_module_function("version_sortkey", function!(version_sortkey, 1))?;

    Ok(())
}
