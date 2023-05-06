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
        hash::{
            Hash,
            Hasher,
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

#[derive(Debug)]
pub(crate) struct Keeper<D: ?Sized> {
    raw: RawRef<D>,
}

#[allow(unused)]
impl<D: ?Sized> Keeper<D> {
    pub(crate) fn new(d: D) -> Self
    where
        D: Sized,
    {
        Keeper {
            raw: RawRef::new(d, RefType::Keeper),
        }
    }

    pub(crate) fn keeper(k: &Keeper<D>) -> Result<Self, RefState> {
        Self::try_from(&k.raw)
    }

    pub(crate) fn reader(k: &Keeper<D>) -> Result<Reader<D>, RefState> {
        Reader::try_from(&k.raw)
    }

    pub(crate) fn owner(k: &Keeper<D>) -> Result<Owner<D>, RefState> {
        Owner::try_from(&k.raw)
    }

    pub(crate) fn state(k: &Keeper<D>) -> RefState {
        k.raw.shared().state()
    }
}

impl<D: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Keeper<U>> for Keeper<D> {}

impl<D: ?Sized> TryClone for Keeper<D> {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        Self::keeper(self).ok()
    }
}

impl<D: ?Sized> TryFrom<&RawRef<D>> for Keeper<D> {
    type Error = RefState;
    fn try_from(value: &RawRef<D>) -> Result<Self, Self::Error> {
        Ok(Keeper {
            raw: RawRef::clone_to(value, RefType::Keeper)?,
        })
    }
}

impl<D: ?Sized> Drop for Keeper<D> {
    fn drop(&mut self) {
        self.raw.drop_from(RefType::Keeper)
    }
}

impl<D: Default> Default for Keeper<D> {
    fn default() -> Self {
        Keeper::new(D::default())
    }
}

impl<D: ?Sized> PartialEq for Keeper<D> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<D: ?Sized> Eq for Keeper<D> {}

impl<D: ?Sized> Hash for Keeper<D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state)
    }
}

#[derive(Debug)]
pub(crate) struct Reader<D: ?Sized> {
    raw: RawRef<D>,
}

#[allow(unused)]
impl<D: ?Sized> Reader<D> {
    pub(crate) fn new(d: D) -> Self
    where
        D: Sized,
    {
        Reader {
            raw: RawRef::new(d, RefType::Reader),
        }
    }

    pub(crate) fn from_keeper(keeper: &Keeper<D>) -> Result<Self, RefState> {
        Self::try_from(&keeper.raw)
    }

    pub(crate) fn reader(r: &Reader<D>) -> Result<Self, RefState> {
        Self::try_from(&r.raw)
    }

    pub(crate) fn keeper(r: &Reader<D>) -> Result<Keeper<D>, RefState> {
        Keeper::try_from(&r.raw)
    }

    pub(crate) fn state(r: &Reader<D>) -> RefState {
        r.raw.shared().state()
    }
}

impl<D: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Reader<U>> for Reader<D> {}

impl<D: ?Sized> TryClone for Reader<D> {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        Self::reader(self).ok()
    }
}

impl<D: ?Sized> Deref for Reader<D> {
    type Target = D;
    fn deref(&self) -> &Self::Target {
        // SAFETY: when self is alive there is no owner and data hasn't been dropped
        unsafe { self.raw.shared().deref() }
    }
}

impl<D: ?Sized> TryFrom<&RawRef<D>> for Reader<D> {
    type Error = RefState;
    fn try_from(value: &RawRef<D>) -> Result<Self, Self::Error> {
        Ok(Reader {
            raw: RawRef::clone_to(value, RefType::Reader)?,
        })
    }
}

impl<D: ?Sized> Drop for Reader<D> {
    fn drop(&mut self) {
        self.raw.drop_from(RefType::Reader)
    }
}

impl<D: Default> Default for Reader<D> {
    fn default() -> Self {
        Reader::new(D::default())
    }
}

impl<D: ?Sized> PartialEq for Reader<D> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<D: ?Sized> Eq for Reader<D> {}

impl<D: ?Sized> Hash for Reader<D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state)
    }
}

#[derive(Debug)]
pub(crate) struct Owner<D: ?Sized> {
    raw: RawRef<D>,
}

#[allow(unused)]
impl<D: ?Sized> Owner<D> {
    pub(crate) fn new(d: D) -> Self
    where
        D: Sized,
    {
        Owner {
            raw: RawRef::new(d, RefType::Owner),
        }
    }

    pub(crate) fn from_keeper(keeper: &Keeper<D>) -> Result<Self, RefState> {
        Self::try_from(&keeper.raw)
    }

    pub(crate) fn keeper(o: &Owner<D>) -> Result<Keeper<D>, RefState> {
        Keeper::try_from(&o.raw)
    }

    pub(crate) fn state(o: &Owner<D>) -> RefState {
        o.raw.shared().state()
    }

    pub(crate) fn borrow_mut(o: &Owner<D>) -> &mut D {
        // SAFETY: we have exclusive ref and data hasn't been dropped
        unsafe { o.raw.shared().deref_mut() }
    }

    pub(crate) fn move_data(o: Owner<D>) -> D
    where
        D: Sized,
    {
        // SAFETY:
        // we have exclusive ref
        // we consume the Owner when taking
        // we change the state to dropped
        // so we won't access the data anymore
        unsafe { o.raw.shared().move_data() }
    }

    pub(crate) fn drop_data(o: Owner<D>) {
        // SAFETY:
        // we have exclusive ref
        // we consume the Owner when deleting
        // we change the state to dropped
        // so we won't access the data anymore
        unsafe { o.raw.shared().drop_data() }
    }
}

impl<D: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Owner<U>> for Owner<D> {}

impl<D: ?Sized> TryClone for Owner<D> {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        None
    }
}

impl<D: ?Sized> Deref for Owner<D> {
    type Target = D;
    fn deref(&self) -> &Self::Target {
        // SAFETY: we have exclusive ref and data hasn't been dropped
        unsafe { self.raw.shared().deref() }
    }
}

impl<D: ?Sized> DerefMut for Owner<D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: we have exclusive ref and data hasn't been dropped
        unsafe { self.raw.shared().deref_mut() }
    }
}

impl<D: ?Sized> TryFrom<&RawRef<D>> for Owner<D> {
    type Error = RefState;
    fn try_from(value: &RawRef<D>) -> Result<Self, Self::Error> {
        Ok(Owner {
            raw: RawRef::clone_to(value, RefType::Owner)?,
        })
    }
}

impl<D: ?Sized> Drop for Owner<D> {
    fn drop(&mut self) {
        self.raw.drop_from(RefType::Owner)
    }
}

impl<D: Default> Default for Owner<D> {
    fn default() -> Self {
        Owner::new(D::default())
    }
}

impl<D: ?Sized> PartialEq for Owner<D> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<D: ?Sized> Eq for Owner<D> {}

impl<D: ?Sized> Hash for Owner<D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state)
    }
}

#[derive(Debug)]
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

    fn clone_to(&self, ref_type: RefType) -> Result<RawRef<D>, RefState> {
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

impl<D: ?Sized> PartialEq for RawRef<D> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl<D: ?Sized> Eq for RawRef<D> {}

impl<D: ?Sized> Hash for RawRef<D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr.hash(state)
    }
}

struct SharedCell<D: ?Sized> {
    state: Cell<RefState>,
    data: UnsafeCell<D>,
}

impl<D: ?Sized> SharedCell<D> {
    fn new(d: D, ref_type: RefType) -> Self
    where
        D: Sized,
    {
        SharedCell {
            state: Cell::new(RefState::new(ref_type)),
            data: UnsafeCell::new(d),
        }
    }

    fn state(&self) -> RefState {
        self.state.get()
    }

    fn clone_to(&self, ref_type: RefType) -> Result<(), RefState> {
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

    // SAFETY: make sure data not dropped and there is no owner
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
    Keeper,
    Reader,
    Owner,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub(crate) struct RefState {
    // the sign bit indicates whether data has been dropped (negative)
    // other bits indicates keeper cnt
    keeper_cnt: i32,
    // the sign bit indicates whether data has been owned (negative)
    // other bits indicates reader cnt
    reader_cnt: i32,
}

#[allow(unused)]
impl RefState {
    pub(crate) fn is_dropped(&self) -> bool {
        self.keeper_cnt < 0
    }
    pub(crate) fn is_owned(&self) -> bool {
        self.reader_cnt < 0
    }
    pub(crate) fn keeper_cnt(&self) -> u32 {
        (self.keeper_cnt & i32::MAX) as u32
    }
    pub(crate) fn is_keeper_full(&self) -> bool {
        self.keeper_cnt & i32::MAX == i32::MAX
    }
    pub(crate) fn reader_cnt(&self) -> u32 {
        (self.reader_cnt & i32::MAX) as u32
    }
    pub(crate) fn is_reader_full(&self) -> bool {
        self.reader_cnt == i32::MAX
    }
    pub(crate) fn is_reading(&self) -> bool {
        self.reader_cnt != 0
    }
}

impl Debug for RefState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RefState")
            .field("drop", &self.is_dropped())
            .field("keep", &self.keeper_cnt())
            .field("read", &self.reader_cnt())
            .field("own", &self.is_owned())
            .finish()
    }
}

impl RefState {
    fn new(ref_type: RefType) -> Self {
        let (keep_cnt, read_cnt) = match ref_type {
            RefType::Keeper => (1, 0),
            RefType::Reader => (0, 1),
            RefType::Owner => (0, i32::MIN),
        };
        RefState {
            keeper_cnt: keep_cnt,
            reader_cnt: read_cnt,
        }
    }

    fn clone_to(mut self, ref_type: RefType) -> Result<RefState, RefState> {
        match ref_type {
            RefType::Keeper => {
                if self.is_keeper_full() {
                    Err(self)
                } else {
                    self.keeper_cnt += 1;
                    Ok(self)
                }
            }
            RefType::Reader => {
                if self.is_dropped() || self.is_owned() || self.is_reader_full() {
                    Err(self)
                } else {
                    self.reader_cnt += 1;
                    Ok(self)
                }
            }
            RefType::Owner => {
                if self.is_dropped() || self.is_reading() {
                    Err(self)
                } else {
                    self.reader_cnt = i32::MIN;
                    Ok(self)
                }
            }
        }
    }

    fn drop_from(mut self, ref_type: RefType) -> RefState {
        match ref_type {
            RefType::Keeper => self.keeper_cnt -= 1,
            RefType::Reader => self.reader_cnt -= 1,
            RefType::Owner => self.reader_cnt = 0,
        }
        self
    }

    fn drop(mut self) -> RefState {
        self.keeper_cnt |= i32::MIN;
        self
    }

    // if already dropped, return false
    fn should_drop(&self) -> bool {
        self.keeper_cnt == 0 && self.reader_cnt == 0
    }

    fn should_dealloc(&self) -> bool {
        self.keeper_cnt == i32::MIN && self.reader_cnt == 0
    }
}

#[cfg(test)]
mod test;
