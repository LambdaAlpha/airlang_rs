use std::borrow::Borrow;
use std::ops::Deref;

use derive_more::Deref;
use derive_more::DerefMut;
use internment::ArcIntern;

use crate::semantics::val::FuncVal;
use crate::semantics::val::Val;
use crate::type_::Map;

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Facts {
    map: Map<usize /*addr of func*/, Io>,
}

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Io {
    set: Map<(ValId, ValId), ()>,
    call: Map<ValId, Vec<ValId>>,
    solve: Map<ValId, Vec<ValId>>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct ValId(ArcIntern<SendSyncVal>);

#[derive(Clone, PartialEq, Eq, Hash, Deref, DerefMut)]
struct SendSyncVal(Val);

impl Facts {
    pub(super) fn put(&mut self, func: FuncVal, input: ValId, output: ValId) {
        let io = self.map.entry(func.id()).or_default();
        io.set.insert((input.clone(), output.clone()), ());
        let outputs = io.call.entry(input.clone()).or_default();
        outputs.push(output.clone());
        let inputs = io.solve.entry(output).or_default();
        inputs.push(input);
    }

    pub(super) fn call(&self, func: FuncVal, input: ValId) -> Option<&[ValId]> {
        let io = self.map.get(&func.id())?;
        io.call.get(&input).map(|values| &values[..])
    }

    pub(super) fn solve(&self, func: FuncVal, output: ValId) -> Option<&[ValId]> {
        let io = self.map.get(&func.id())?;
        io.solve.get(&output).map(|values| &values[..])
    }

    pub(super) fn exist(&self, func: FuncVal, input: ValId, output: ValId) -> bool {
        let Some(io) = self.map.get(&func.id()) else {
            return false;
        };
        io.set.contains_key(&(input, output))
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&usize, &Io)> {
        self.map.iter()
    }
}

impl Io {
    pub(crate) fn iter(&self) -> impl Iterator<Item = (&Val, &Val)> {
        self.set.iter().map(|((i, o), ())| (&**i, &**o))
    }
}

impl From<Val> for ValId {
    fn from(v: Val) -> Self {
        Self(ArcIntern::new(SendSyncVal(v)))
    }
}

impl Deref for ValId {
    type Target = Val;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// safety: read only
unsafe impl Send for SendSyncVal {}
// safety: read only
unsafe impl Sync for SendSyncVal {}

impl Borrow<Val> for SendSyncVal {
    fn borrow(&self) -> &Val {
        &self.0
    }
}

impl From<&Val> for SendSyncVal {
    fn from(v: &Val) -> Self {
        Self(v.clone())
    }
}
