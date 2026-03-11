use std::cell::Cell;

use crate::{
    codegen::{FnCodegen, loops::Looper},
    ty::{ContiguousUniformTy, M, U32, Void, cuda::Shared},
    val::{S, Val},
};

pub struct CpAsyncToken(());

// Intended for export
// Intended to be returned for future sync.
#[derive(Clone, Copy)]
pub struct CpAsyncTicket<'a, T> {
    index_in_cp_async: u32,
    cx: &'a FnCodegen,
    inner: T,
}

pub struct CpAsyncEngine<'a>(pub(crate) &'a FnCodegen);

#[derive(Clone, Copy, Default)]
struct CpAsyncTicketing {
    initialized: u32,
    completed: u32,
}
thread_local! {
    static CP_ASYNC_TICKETING: Cell<CpAsyncTicketing> = Cell::default();
}

impl<'a> CpAsyncEngine<'a> {
    fn async_transaction<T>(&self, f: impl FnOnce(CpAsyncToken) -> T) -> CpAsyncTicket<'a, T> {
        let index_in_cp_async = CP_ASYNC_TICKETING.with(|c| {
            let mut t = c.get();
            t.initialized += 1;
            c.set(t);
            t.initialized
        });
        let ret = f(CpAsyncToken(()));
        let cp_async_commit_group = "llvm.nvvm.cp.async.commit.group";
        let cp_async_commit_group = self
            .0
            .get_intrinsic::<Void, ()>(cp_async_commit_group, true);
        self.0.call_void_fn(cp_async_commit_group, ());
        CpAsyncTicket {
            index_in_cp_async,
            cx: self.0,
            inner: ret,
        }
    }
}

impl<T> CpAsyncTicket<'_, T> {
    pub fn wait_group(&self, n: u32) {
        let cp_async_wait_group = "llvm.nvvm.cp.async.wait.group";
        let cp_async_wait_group = self
            .cx
            .get_intrinsic::<Void, (U32,)>(cp_async_wait_group, true);
        self.cx
            .call_void_fn(cp_async_wait_group, (self.cx.constant_from(n),));
    }
    fn wait(self) -> T {
        let index_to_validate = self.index_in_cp_async;
        let mut ticketing = CP_ASYNC_TICKETING.with(|c| c.get());
        if index_to_validate <= ticketing.completed {
            // then this has already completed and there's no reason to continue
            return self.inner;
        }
        assert!(index_to_validate <= ticketing.initialized);
        const MAX_IN_FLIGHT: u32 = 8;
        // Guaranteed > 0
        let number_behind_head = (ticketing.initialized - index_to_validate).min(MAX_IN_FLIGHT);
        self.wait_group(number_behind_head);

        ticketing.completed = ticketing.initialized - number_behind_head;
        CP_ASYNC_TICKETING.with(|c| c.set(ticketing));
        self.inner
    }

    unsafe fn get_unchecked(self) -> T {
        self.inner
    }
}

pub struct CpAsyncPipeline<'a, 'b, Resource> {
    depth: u32,
    res_index: Val<'a, S<U32>>,
    resource: Val<'a, M<&'b mut Resource, Shared>>,
}

impl<'a, 'b, Resource, ElemT> CpAsyncPipeline<'a, 'b, Resource>
where
    Resource: ContiguousUniformTy<ElemT = ElemT>,
{
    pub fn resources_at_mut<'c>(
        &'c mut self,
        index: Val<'a, U32>,
    ) -> Val<'a, M<&'c mut ElemT, Shared>>
    where
        'a: 'c,
    {
        Resource::runtime_element_mut(self.resource.reborrow_mut(), index)
    }
    pub fn new(depth: u32, smem_resource: Val<'a, M<&'b mut Resource, Shared>>) -> Self {
        Self {
            depth,
            res_index: smem_resource.cx().constant_from(0u32).with_storage(),
            resource: smem_resource,
        }
    }
    pub fn prime_with<'looper, 'loop_borrow, 'short_borrow, L, F>(
        &mut self,
        looper: &'looper mut L,
        mut f: F,
    ) where
        F: FnMut(CpAsyncToken, L::ItemT<'looper>, Val<'a, M<&mut ElemT, Shared>>),
        L: Looper,
        ElemT: 'short_borrow,
    {
        let cp_async_commit_group = "llvm.nvvm.cp.async.commit.group";
        looper.on_first_n(self.depth as usize, |item| {
            let index: u32 = 0.try_into().expect("usize -> u32 overflow");
            let cx = self.res_index.cx();
            let index = cx.constant_from(index);
            let res = self.resources_at_mut(index);
            f(CpAsyncToken(()), item, res);
            let cp_async_commit_group = cx.get_intrinsic::<Void, ()>(cp_async_commit_group, true);
            cx.call_void_fn(cp_async_commit_group, ());
        });
    }
}

pub struct PrimedCpAsyncPipeline<'a, 'b, Resource> {
    inner: CpAsyncPipeline<'a, 'b, Resource>,
}

impl<'a, 'b, Resource, ElemT> PrimedCpAsyncPipeline<'a, 'b, Resource>
where
    Resource: ContiguousUniformTy<ElemT = ElemT>,
{
    fn async_for_each() {}
}
