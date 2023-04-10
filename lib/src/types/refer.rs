use {
    crate::traits::TryClone,
    std::{
        alloc,
        cell::{
            Cell,
            UnsafeCell,
        },
        fmt::{
            Debug,
            Formatter,
        },
        marker::PhantomData,
        ops::{
            Deref,
            DerefMut,
        },
        ptr::{
            self,
            NonNull,
        },
    },
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct BoxRef<D> {
    raw: RawRef<D>,
}

#[allow(unused)]
impl<D> BoxRef<D> {
    pub(crate) fn new(d: D) -> Self {
        BoxRef {
            raw: RawRef::new(d, RefType::Box),
        }
    }

    pub(crate) fn ref_box(&self) -> Result<Self, CellState> {
        Self::try_from(&self.raw)
    }

    pub(crate) fn ref_im(&self) -> Result<ImRef<D>, CellState> {
        ImRef::try_from(&self.raw)
    }

    pub(crate) fn ref_mut(&self) -> Result<MutRef<D>, CellState> {
        MutRef::try_from(&self.raw)
    }

    pub(crate) fn state(&self) -> CellState {
        self.raw.state()
    }
}

impl<D> TryClone for BoxRef<D> {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        self.ref_box().ok()
    }
}

impl<D> TryFrom<&RawRef<D>> for BoxRef<D> {
    type Error = CellState;
    fn try_from(value: &RawRef<D>) -> Result<Self, Self::Error> {
        Ok(BoxRef {
            raw: RawRef::clone_from(value, RefType::Box)?,
        })
    }
}

impl<D> Drop for BoxRef<D> {
    fn drop(&mut self) {
        self.raw.drop_from(RefType::Box)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ImRef<D> {
    raw: RawRef<D>,
}

#[allow(unused)]
impl<D> ImRef<D> {
    pub(crate) fn new(d: D) -> Self {
        ImRef {
            raw: RawRef::new(d, RefType::Im),
        }
    }

    pub(crate) fn from_box(box_cell: &BoxRef<D>) -> Result<Self, CellState> {
        Self::try_from(&box_cell.raw)
    }

    pub(crate) fn ref_im(&self) -> Result<Self, CellState> {
        Self::try_from(&self.raw)
    }

    pub(crate) fn ref_box(&self) -> Result<BoxRef<D>, CellState> {
        BoxRef::try_from(&self.raw)
    }

    pub(crate) fn state(&self) -> CellState {
        self.raw.state()
    }
}

impl<D> TryClone for ImRef<D> {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        self.ref_im().ok()
    }
}

impl<D> Deref for ImRef<D> {
    type Target = D;
    fn deref(&self) -> &Self::Target {
        // SAFETY: when self is alive there is no mutable ref and data hasn't been dropped
        unsafe { self.raw.deref() }
    }
}

impl<D> TryFrom<&RawRef<D>> for ImRef<D> {
    type Error = CellState;
    fn try_from(value: &RawRef<D>) -> Result<Self, Self::Error> {
        Ok(ImRef {
            raw: RawRef::clone_from(value, RefType::Im)?,
        })
    }
}

impl<D> Drop for ImRef<D> {
    fn drop(&mut self) {
        self.raw.drop_from(RefType::Im)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct MutRef<D> {
    raw: RawRef<D>,
}

#[allow(unused)]
impl<D> MutRef<D> {
    pub(crate) fn new(d: D) -> Self {
        MutRef {
            raw: RawRef::new(d, RefType::Mut),
        }
    }

    pub(crate) fn from_box(box_cell: &BoxRef<D>) -> Result<Self, CellState> {
        Self::try_from(&box_cell.raw)
    }

    pub(crate) fn ref_box(&self) -> Result<BoxRef<D>, CellState> {
        BoxRef::try_from(&self.raw)
    }

    pub(crate) fn state(&self) -> CellState {
        self.raw.state()
    }

    pub(crate) fn delete(self) {
        // SAFETY: we have exclusive ref and we consume self when delete, so we won't delete twice
        unsafe { self.raw.drop_data() }
    }
}

impl<D> TryClone for MutRef<D> {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        None
    }
}

impl<D> Deref for MutRef<D> {
    type Target = D;
    fn deref(&self) -> &Self::Target {
        // SAFETY: we have exclusive ref and data hasn't been dropped
        unsafe { self.raw.deref() }
    }
}

impl<D> DerefMut for MutRef<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: we have exclusive ref and data hasn't been dropped
        unsafe { self.raw.deref_mut() }
    }
}

impl<D> TryFrom<&RawRef<D>> for MutRef<D> {
    type Error = CellState;
    fn try_from(value: &RawRef<D>) -> Result<Self, Self::Error> {
        Ok(MutRef {
            raw: RawRef::clone_from(value, RefType::Mut)?,
        })
    }
}

impl<D> Drop for MutRef<D> {
    fn drop(&mut self) {
        self.raw.drop_from(RefType::Mut)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct RawRef<D> {
    ptr: NonNull<SharedCell<D>>,
    phantom: PhantomData<SharedCell<D>>,
}

impl<D> RawRef<D> {
    fn new(d: D, ref_type: RefType) -> Self {
        RawRef {
            ptr: Box::leak(Box::new(SharedCell::new(d, ref_type))).into(),
            phantom: PhantomData,
        }
    }

    fn clone_from(&self, ref_type: RefType) -> Result<RawRef<D>, CellState> {
        self.shared().clone_from(ref_type)?;
        Ok(RawRef {
            ptr: self.ptr,
            phantom: PhantomData,
        })
    }

    fn drop_from(&self, ref_type: RefType) {
        self.shared().drop_from(ref_type);

        if self.shared().should_dealloc() {
            let layout = alloc::Layout::for_value(self.shared());
            // SAFETY:
            // state promises that we can and should dealloc
            // we are the last RawCell accessible to the ptr of shared cell and we are dropped
            // we carefully don't make any ref to shared cell when calling dealloc
            unsafe {
                alloc::dealloc(self.ptr.as_ptr().cast(), layout);
            }
        }
    }

    fn shared(&self) -> &SharedCell<D> {
        // SAFETY: when self is alive, ptr is always valid and we never call ptr.as_mut()
        unsafe { self.ptr.as_ref() }
    }

    fn state(&self) -> CellState {
        self.shared().state()
    }

    // SAFETY: make sure data not dropped and there is no mut ref
    unsafe fn deref<'a>(&self) -> &'a D {
        self.shared().deref()
    }

    // SAFETY: make sure data not dropped and there is no ref
    unsafe fn deref_mut<'a>(&self) -> &'a mut D {
        self.shared().deref_mut()
    }

    // SAFETY: call only once and there is no ref
    unsafe fn drop_data(&self) {
        self.shared().drop_data()
    }
}

struct SharedCell<D> {
    state: SharedState,
    data: SharedData<D>,
}

impl<D> SharedCell<D> {
    fn new(d: D, ref_type: RefType) -> Self {
        SharedCell {
            state: SharedState::new(ref_type),
            data: SharedData::new(d),
        }
    }

    fn state(&self) -> CellState {
        self.state.state()
    }

    fn clone_from(&self, ref_type: RefType) -> Result<(), CellState> {
        self.state.clone_from(ref_type)
    }

    fn drop_from(&self, ref_type: RefType) {
        self.state.drop_from(ref_type);
        if self.state.should_drop() {
            // SAFETY: state promises that we can and should drop
            unsafe { self.drop_data() }
        }
    }

    // SAFETY: call only once and there is no ref
    unsafe fn drop_data(&self) {
        self.data.drop();
        self.state.drop();
    }

    fn should_dealloc(&self) -> bool {
        self.state.should_dealloc()
    }

    // SAFETY: make sure data not dropped and there is no mut ref
    unsafe fn deref<'a>(&self) -> &'a D {
        self.data.deref()
    }

    // SAFETY: make sure data not dropped and there is no ref
    unsafe fn deref_mut<'a>(&self) -> &'a mut D {
        self.data.deref_mut()
    }
}

pub(crate) struct SharedData<D> {
    value: UnsafeCell<D>,
}

impl<D> SharedData<D> {
    fn new(d: D) -> Self {
        SharedData {
            value: UnsafeCell::new(d),
        }
    }

    // SAFETY: make sure data not dropped and there is no mut ref
    unsafe fn deref<'a>(&self) -> &'a D {
        self.value.get().as_ref().unwrap()
    }

    // SAFETY: make sure data not dropped and there is no ref
    unsafe fn deref_mut<'a>(&self) -> &'a mut D {
        self.value.get().as_mut().unwrap()
    }

    // SAFETY: make sure data not dropped and there is no ref
    unsafe fn drop(&self) {
        ptr::drop_in_place(self.value.get())
    }
}

#[derive(Debug, Copy, Clone)]
enum RefType {
    Box,
    Im,
    Mut,
}

#[derive(PartialEq, Eq)]
pub(crate) struct CellState {
    // the sign bit indicates whether data has been dropped (negative)
    // other bits indicates box ref cnt
    box_cnt: i32,
    // the sign bit indicates whether data has been mutable borrowed (negative)
    // other bits indicates immutable borrow cnt
    ref_cnt: i32,
}

#[allow(unused)]
impl CellState {
    pub(crate) fn is_dropped(&self) -> bool {
        self.box_cnt < 0
    }
    pub(crate) fn is_mut(&self) -> bool {
        self.ref_cnt < 0
    }
    pub(crate) fn box_cnt(&self) -> u32 {
        (self.box_cnt & i32::MAX) as u32
    }
    pub(crate) fn is_box_full(&self) -> bool {
        self.box_cnt & i32::MAX == i32::MAX
    }
    pub(crate) fn im_cnt(&self) -> u32 {
        (self.ref_cnt & i32::MAX) as u32
    }
    pub(crate) fn is_im_full(&self) -> bool {
        self.ref_cnt == i32::MAX
    }
    pub(crate) fn is_referred(&self) -> bool {
        self.ref_cnt != 0
    }

    fn clone_box(box_cnt: i32) -> i32 {
        box_cnt + 1
    }
    fn drop_box(box_cnt: i32) -> i32 {
        box_cnt - 1
    }
    fn clone_im(ref_cnt: i32) -> i32 {
        ref_cnt + 1
    }
    fn drop_im(ref_cnt: i32) -> i32 {
        ref_cnt - 1
    }
    fn clone_mut(_: i32) -> i32 {
        i32::MIN
    }
    fn drop_mut(_: i32) -> i32 {
        0
    }
    fn should_drop(&self) -> bool {
        self.box_cnt == 0 && self.ref_cnt == 0
    }
    fn drop(ref_cnt: i32) -> i32 {
        ref_cnt | i32::MIN
    }
    fn should_dealloc(&self) -> bool {
        self.box_cnt == i32::MIN && self.ref_cnt == 0
    }
}

impl Debug for CellState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CellState")
            .field("drop", &self.is_dropped())
            .field("box", &self.box_cnt())
            .field("im", &self.im_cnt())
            .field("mut", &self.is_mut())
            .finish()
    }
}

struct SharedState {
    // the sign bit indicates whether data has been dropped (negative)
    // other bits indicates box ref cnt
    box_cnt: Cell<i32>,
    // the sign bit indicates whether data has been mutable borrowed (negative)
    // other bits indicates immutable borrow cnt
    ref_cnt: Cell<i32>,
}

impl SharedState {
    fn new(ref_type: RefType) -> Self {
        let (box_cnt, ref_cnt) = match ref_type {
            RefType::Box => (1, 0),
            RefType::Im => (0, 1),
            RefType::Mut => (0, i32::MIN),
        };
        SharedState {
            box_cnt: Cell::new(box_cnt),
            ref_cnt: Cell::new(ref_cnt),
        }
    }

    fn state(&self) -> CellState {
        CellState {
            box_cnt: self.box_cnt.get(),
            ref_cnt: self.ref_cnt.get(),
        }
    }

    fn clone_from(&self, ref_type: RefType) -> Result<(), CellState> {
        let state = self.state();
        match ref_type {
            RefType::Box => {
                if state.is_box_full() {
                    Err(state)
                } else {
                    self.box_cnt.set(CellState::clone_box(state.box_cnt));
                    Ok(())
                }
            }
            RefType::Im => {
                if state.is_dropped() || state.is_mut() || state.is_im_full() {
                    Err(state)
                } else {
                    self.ref_cnt.set(CellState::clone_im(state.ref_cnt));
                    Ok(())
                }
            }
            RefType::Mut => {
                if state.is_dropped() || state.is_referred() {
                    Err(state)
                } else {
                    self.ref_cnt.set(CellState::clone_mut(state.ref_cnt));
                    Ok(())
                }
            }
        }
    }

    fn drop_from(&self, ref_type: RefType) {
        let state = self.state();
        match ref_type {
            RefType::Box => self.box_cnt.set(CellState::drop_box(state.box_cnt)),
            RefType::Im => self.ref_cnt.set(CellState::drop_im(state.ref_cnt)),
            RefType::Mut => self.ref_cnt.set(CellState::drop_mut(state.ref_cnt)),
        }
    }

    fn drop(&self) {
        self.box_cnt.set(CellState::drop(self.box_cnt.get()))
    }

    // if already dropped, return false
    fn should_drop(&self) -> bool {
        self.state().should_drop()
    }

    fn should_dealloc(&self) -> bool {
        self.state().should_dealloc()
    }
}

#[cfg(test)]
mod test;
