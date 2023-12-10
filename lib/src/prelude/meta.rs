use crate::{
    ctx::NameMap,
    int::Int,
    prelude::{
        Named,
        Prelude,
    },
};

#[derive(Clone)]
pub(crate) struct MetaPrelude {
    pub(crate) version_major: Named<Int>,
    pub(crate) version_minor: Named<Int>,
    pub(crate) version_patch: Named<Int>,
}

impl Default for MetaPrelude {
    fn default() -> Self {
        MetaPrelude {
            version_major: Named::new("air.version_major", version_major()),
            version_minor: Named::new("air.version_minor", version_minor()),
            version_patch: Named::new("air.version_patch", version_patch()),
        }
    }
}

impl Prelude for MetaPrelude {
    fn put(&self, m: &mut NameMap) {
        self.version_major.put(m);
        self.version_minor.put(m);
        self.version_patch.put(m);
    }
}

fn version_major() -> Int {
    const MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
    Int::from_sign_string_radix(true, MAJOR, 10)
}

fn version_minor() -> Int {
    const MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
    Int::from_sign_string_radix(true, MINOR, 10)
}

fn version_patch() -> Int {
    const PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");
    Int::from_sign_string_radix(true, PATCH, 10)
}
