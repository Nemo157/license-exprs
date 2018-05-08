use std::borrow::Cow;

use spdx;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id<'a>(Cow<'a, str>);

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Simple<'a> {
    LicenseId {
        id: Id<'a>,
        or_later: bool,
    },

    LicenseRef {
        id: Id<'a>,
        document: Option<Id<'a>>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Compound<'a> {
    Simple {
        license: Simple<'a>,
    },

    WithException {
        license: Simple<'a>,
        exception: Id<'a>,
    },

    And {
        left: Box<Compound<'a>>,
        right: Box<Compound<'a>>,
    },

    Or {
        left: Box<Compound<'a>>,
        right: Box<Compound<'a>>,
    },
}

impl<'a> Id<'a> {
    pub fn into_static(self) -> Id<'static> {
        Id(self.0.into_owned().into())
    }
}

impl<'a> AsRef<str> for Id<'a> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> Simple<'a> {
    pub fn is_valid(&self) -> bool {
        match *self {
            Simple::LicenseId { ref id, .. } => spdx::LICENSES.contains(&id.as_ref()),
            Simple::LicenseRef { .. } => true,
        }
    }

    pub fn into_static(self) -> Simple<'static> {
        match self {
            Simple::LicenseId { id, or_later } =>
                Simple::LicenseId { id: id.into_static(), or_later },
            Simple::LicenseRef { id, document } =>
                Simple::LicenseRef { id: id.into_static(), document: document.map(|d| d.into_static()) },
        }
    }
}

impl<'a> Compound<'a> {
    pub fn is_valid(&self) -> bool {
        match *self {
            Compound::Simple { ref license }
            | Compound::WithException { ref license, .. } =>
                license.is_valid(),
            Compound::And { ref left, ref right }
            | Compound::Or { ref left, ref right } =>
                left.is_valid() && right.is_valid(),
        }
    }

    pub fn into_static(self) -> Compound<'static> {
        match self {
            Compound::Simple { license } =>
                Compound::Simple { license: license.into_static() },
            Compound::WithException { license, exception } =>
                Compound::WithException { license: license.into_static(), exception: exception.into_static() },
            Compound::And { left, right } =>
                Compound::And { left: Box::new(left.into_static()), right: Box::new(right.into_static()) },
            Compound::Or { left, right } =>
                Compound::Or { left: Box::new(left.into_static()), right: Box::new(right.into_static()) },
        }
    }
}
