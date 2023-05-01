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
        marker::{
            PhantomData,
            Unsize,
        },
        ops::{
            CoerceUnsized,
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
pub(crate) struct BoxRef<D: ?Sized> {
    raw: RawRef<D>,
}

#[allow(unused)]
impl<D: ?Sized> BoxRef<D> {
    pub(crate) fn new(d: D) -> Self
    where
        D: Sized,
    {
        BoxRef {
            raw: RawRef::new(d, RefType::Box),
        }
    }

    pub(crate) fn ref_box(b: &BoxRef<D>) -> Result<Self, CellState> {
        Self::try_from(&b.raw)
    }

    pub(crate) fn ref_im(b: &BoxRef<D>) -> Result<ImRef<D>, CellState> {
        ImRef::try_from(&b.raw)
    }

    pub(crate) fn ref_mut(b: &BoxRef<D>) -> Result<MutRef<D>, CellState> {
        MutRef::try_from(&b.raw)
    }

    pub(crate) fn state(b: &BoxRef<D>) -> CellState {
        b.raw.shared().state()
    }
}

impl<D: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<BoxRef<U>> for BoxRef<D> {}

impl<D: ?Sized> TryClone for BoxRef<D> {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        Self::ref_box(self).ok()
    }
}

impl<D: ?Sized> TryFrom<&RawRef<D>> for BoxRef<D> {
    type Error = CellState;
    fn try_from(value: &RawRef<D>) -> Result<Self, Self::Error> {
        Ok(BoxRef {
            raw: RawRef::clone_to(value, RefType::Box)?,
        })
    }
}

impl<D: ?Sized> Drop for BoxRef<D> {
    fn drop(&mut self) {
        self.raw.drop_from(RefType::Box)
    }
}

impl<D: Default> Default for BoxRef<D> {
    fn default() -> Self {
        BoxRef::new(D::default())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ImRef<D: ?Sized> {
    raw: RawRef<D>,
}

#[allow(unused)]
impl<D: ?Sized> ImRef<D> {
    pub(crate) fn new(d: D) -> Self
    where
        D: Sized,
    {
        ImRef {
            raw: RawRef::new(d, RefType::Im),
        }
    }

    pub(crate) fn from_box(box_cell: &BoxRef<D>) -> Result<Self, CellState> {
        Self::try_from(&box_cell.raw)
    }

    pub(crate) fn ref_im(i: &ImRef<D>) -> Result<Self, CellState> {
        Self::try_from(&i.raw)
    }

    pub(crate) fn ref_box(i: &ImRef<D>) -> Result<BoxRef<D>, CellState> {
        BoxRef::try_from(&i.raw)
    }

    pub(crate) fn state(i: &ImRef<D>) -> CellState {
        i.raw.shared().state()
    }
}

impl<D: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<ImRef<U>> for ImRef<D> {}

impl<D: ?Sized> TryClone for ImRef<D> {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        Self::ref_im(self).ok()
    }
}

impl<D: ?Sized> Deref for ImRef<D> {
    type Target = D;
    fn deref(&self) -> &Self::Target {
        // SAFETY: when self is alive there is no mutable ref and data hasn't been dropped
        unsafe { self.raw.shared().deref() }
    }
}

impl<D: ?Sized> TryFrom<&RawRef<D>> for ImRef<D> {
    type Error = CellState;
    fn try_from(value: &RawRef<D>) -> Result<Self, Self::Error> {
        Ok(ImRef {
            raw: RawRef::clone_to(value, RefType::Im)?,
        })
    }
}

impl<D: ?Sized> Drop for ImRef<D> {
    fn drop(&mut self) {
        self.raw.drop_from(RefType::Im)
    }
}

impl<D: Default> Default for ImRef<D> {
    fn default() -> Self {
        ImRef::new(D::default())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct MutRef<D: ?Sized> {
    raw: RawRef<D>,
}

#[allow(unused)]
impl<D: ?Sized> MutRef<D> {
    pub(crate) fn new(d: D) -> Self
    where
        D: Sized,
    {
        MutRef {
            raw: RawRef::new(d, RefType::Mut),
        }
    }

    pub(crate) fn from_box(box_cell: &BoxRef<D>) -> Result<Self, CellState> {
        Self::try_from(&box_cell.raw)
    }

    pub(crate) fn ref_box(m: &MutRef<D>) -> Result<BoxRef<D>, CellState> {
        BoxRef::try_from(&m.raw)
    }

    pub(crate) fn state(m: &MutRef<D>) -> CellState {
        m.raw.shared().state()
    }

    pub(crate) fn borrow_mut(m: &MutRef<D>) -> &mut D {
        // SAFETY: we have exclusive ref and data hasn't been dropped
        unsafe { m.raw.shared().deref_mut() }
    }

    pub(crate) fn move_data(m: MutRef<D>) -> D
    where
        D: Sized,
    {
        // SAFETY:
        // we have exclusive ref
        // we consume the MutRef when taking
        // we change the state to dropped
        // so we won't access the data anymore
        unsafe { m.raw.shared().move_data() }
    }

    pub(crate) fn drop_data(m: MutRef<D>) {
        // SAFETY:
        // we have exclusive ref
        // we consume the MutRef when deleting
        // we change the state to dropped
        // so we won't access the data anymore
        unsafe { m.raw.shared().drop_data() }
    }
}

impl<D: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<MutRef<U>> for MutRef<D> {}

impl<D: ?Sized> TryClone for MutRef<D> {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        None
    }
}

impl<D: ?Sized> Deref for MutRef<D> {
    type Target = D;
    fn deref(&self) -> &Self::Target {
        // SAFETY: we have exclusive ref and data hasn't been dropped
        unsafe { self.raw.shared().deref() }
    }
}

impl<D: ?Sized> DerefMut for MutRef<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: we have exclusive ref and data hasn't been dropped
        unsafe { self.raw.shared().deref_mut() }
    }
}

impl<D: ?Sized> TryFrom<&RawRef<D>> for MutRef<D> {
    type Error = CellState;
    fn try_from(value: &RawRef<D>) -> Result<Self, Self::Error> {
        Ok(MutRef {
            raw: RawRef::clone_to(value, RefType::Mut)?,
        })
    }
}

impl<D: ?Sized> Drop for MutRef<D> {
    fn drop(&mut self) {
        self.raw.drop_from(RefType::Mut)
    }
}

impl<D: Default> Default for MutRef<D> {
    fn default() -> Self {
        MutRef::new(D::default())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct RawRef<D: ?Sized> {
    ptr: NonNull<SharedCell<D>>,
    phantom: PhantomData<SharedCell<D>>,
}

impl<D: ?Sized> RawRef<D> {
    fn new(d: D, ref_type: RefType) -> Self
    where
        D: Sized,
    {
        RawRef {
            ptr: Box::leak(Box::new(SharedCell::new(d, ref_type))).into(),
            phantom: PhantomData,
        }
    }

    fn clone_to(&self, ref_type: RefType) -> Result<RawRef<D>, CellState> {
        self.shared().clone_to(ref_type)?;
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
}

impl<D: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<RawRef<U>> for RawRef<D> {}

struct SharedCell<D: ?Sized> {
    state: Cell<CellState>,
    data: UnsafeCell<D>,
}

impl<D: ?Sized> SharedCell<D> {
    fn new(d: D, ref_type: RefType) -> Self
    where
        D: Sized,
    {
        SharedCell {
            state: Cell::new(CellState::new(ref_type)),
            data: UnsafeCell::new(d),
        }
    }

    fn state(&self) -> CellState {
        self.state.get()
    }

    fn clone_to(&self, ref_type: RefType) -> Result<(), CellState> {
        self.state.set(self.state.get().clone_to(ref_type)?);
        Ok(())
    }

    fn drop_from(&self, ref_type: RefType) {
        self.state.set(self.state.get().drop_from(ref_type));
        if self.state.get().should_drop() {
            // SAFETY: state promises that we can and should drop
            unsafe { self.drop_data() }
        }
    }

    // SAFETY: call only once and there is no ref
    unsafe fn move_data(&self) -> D
    where
        D: Sized,
    {
        self.state.set(self.state.get().drop());
        ptr::read(self.data.get())
    }

    // SAFETY: call only once and there is no ref
    unsafe fn drop_data(&self) {
        self.state.set(self.state.get().drop());
        ptr::drop_in_place(self.data.get())
    }

    fn should_dealloc(&self) -> bool {
        self.state.get().should_dealloc()
    }

    // SAFETY: make sure data not dropped and there is no mut ref
    unsafe fn deref<'a>(&self) -> &'a D {
        self.data.get().as_ref().unwrap()
    }

    // SAFETY: make sure data not dropped and there is no ref
    unsafe fn deref_mut<'a>(&self) -> &'a mut D {
        self.data.get().as_mut().unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
enum RefType {
    Box,
    Im,
    Mut,
}

#[derive(PartialEq, Eq, Copy, Clone)]
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

impl CellState {
    fn new(ref_type: RefType) -> Self {
        let (box_cnt, ref_cnt) = match ref_type {
            RefType::Box => (1, 0),
            RefType::Im => (0, 1),
            RefType::Mut => (0, i32::MIN),
        };
        CellState { box_cnt, ref_cnt }
    }

    fn clone_to(mut self, ref_type: RefType) -> Result<CellState, CellState> {
        match ref_type {
            RefType::Box => {
                if self.is_box_full() {
                    Err(self)
                } else {
                    self.box_cnt += 1;
                    Ok(self)
                }
            }
            RefType::Im => {
                if self.is_dropped() || self.is_mut() || self.is_im_full() {
                    Err(self)
                } else {
                    self.ref_cnt += 1;
                    Ok(self)
                }
            }
            RefType::Mut => {
                if self.is_dropped() || self.is_referred() {
                    Err(self)
                } else {
                    self.ref_cnt = i32::MIN;
                    Ok(self)
                }
            }
        }
    }

    fn drop_from(mut self, ref_type: RefType) -> CellState {
        match ref_type {
            RefType::Box => self.box_cnt -= 1,
            RefType::Im => self.ref_cnt -= 1,
            RefType::Mut => self.ref_cnt = 0,
        }
        self
    }

    fn drop(mut self) -> CellState {
        self.box_cnt |= i32::MIN;
        self
    }

    // if already dropped, return false
    fn should_drop(&self) -> bool {
        self.box_cnt == 0 && self.ref_cnt == 0
    }

    fn should_dealloc(&self) -> bool {
        self.box_cnt == i32::MIN && self.ref_cnt == 0
    }
}

#[cfg(test)]
mod test;
