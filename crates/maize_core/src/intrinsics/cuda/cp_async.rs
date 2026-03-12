use std::cell::Cell;

use crate::{
    codegen::{FnCodegen, loops::Looper},
    ty::{ContiguousUniformTy, M, R, U32, ValTy, Void, cuda::Shared},
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

#[expect(unused, reason = "I'm deciding whether this should be used at all")]
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

fn wait_group(cx: &FnCodegen, n: u32) {
    let cp_async_wait_group = "llvm.nvvm.cp.async.wait.group";

    let cp_async_wait_group = cx.get_intrinsic::<Void, (U32,)>(cp_async_wait_group, true);
    cx.call_void_fn(cp_async_wait_group, (cx.constant_from(n),));
}

fn commit_group(cx: &FnCodegen) {
    let cp_async_commit_group = "llvm.nvvm.cp.async.commit.group";
    let cp_async_commit_group = cx.get_intrinsic::<Void, ()>(cp_async_commit_group, true);
    cx.call_void_fn(cp_async_commit_group, ());
}

#[expect(unused, reason = "I'm deciding whether this should be used at all")]
impl<T> CpAsyncTicket<'_, T> {
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
        wait_group(&self.cx, number_behind_head);

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

impl<'a, 'b, Resource> CpAsyncPipeline<'a, 'b, Resource> {
    pub fn new(depth: u32, smem_resource: Val<'a, M<&'b mut Resource, Shared>>) -> Self {
        Self {
            depth,
            res_index: smem_resource.cx().constant_from(0u32).with_storage(),
            resource: smem_resource,
        }
    }
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

    pub fn prime_with<L, F>(
        mut self,
        mut looper: L,
        mut f: F,
    ) -> PrimedCpAsyncPipeline<'a, 'b, Resource, L, F>
    where
        F: for<'l> FnMut(CpAsyncToken, L::ItemT, Val<'a, M<&mut ElemT, Shared>>),
        L: Looper<'a>,
    {
        let cx = self.res_index.cx();
        looper.on_first_n(self.depth as usize, |item| {
            let cp_index = self.res_index.as_mut().load();
            let new_index = (cp_index + 1) % self.depth;
            self.res_index.as_mut().store(new_index);
            let res = self.resources_at_mut(cp_index);
            f(CpAsyncToken(()), item, res);
            commit_group(&cx);
        });
        self.res_index.as_mut().store(cx.constant_from(0u32));
        PrimedCpAsyncPipeline {
            inner: self,
            looper,
            copy_func: f,
        }
    }
}

pub struct PrimedCpAsyncPipeline<'a, 'b, Resource, L, F> {
    inner: CpAsyncPipeline<'a, 'b, Resource>,
    looper: L,
    copy_func: F,
}

pub struct DepletedCpAsyncPipeline<'a, 'b, Resource> {
    inner: CpAsyncPipeline<'a, 'b, Resource>,
}

impl<'a, 'b, Resource, ElemT, L, CopyFunc> PrimedCpAsyncPipeline<'a, 'b, Resource, L, CopyFunc>
where
    Resource: ContiguousUniformTy<ElemT = ElemT>,
    L: Looper<'a>,
    CopyFunc: FnMut(CpAsyncToken, L::ItemT, Val<'a, M<&mut ElemT, Shared>>),
    ElemT: ValTy,
{
    pub fn at_steady_state<F>(mut self, f: &mut F) -> DepletedCpAsyncPipeline<'a, 'b, Resource>
    where
        F: FnMut(Val<'a, R<&ElemT, Shared>>),
    {
        let max_outstanding = self.inner.depth - 1;
        let cx = self.inner.res_index.cx();
        self.looper.for_every_value(|item| {
            let index = self.inner.res_index.as_mut().load();
            wait_group(&cx, max_outstanding);
            let resource = self.inner.resources_at_mut(index);
            f(resource.reborrow());
            (self.copy_func)(CpAsyncToken(()), item, resource);
            commit_group(&cx);
            self.inner
                .res_index
                .as_mut()
                .store((index + 1) % self.inner.depth);
        });

        DepletedCpAsyncPipeline { inner: self.inner }
    }
}

impl<'a, 'b, Resource, ElemT> DepletedCpAsyncPipeline<'a, 'b, Resource>
where
    Resource: ContiguousUniformTy<ElemT = ElemT>,
    ElemT: ValTy,
{
    pub fn finalize<F>(mut self, f: &mut F) -> CpAsyncPipeline<'a, 'b, Resource>
    where
        F: FnMut(Val<'a, R<&ElemT, Shared>>),
    {
        let cx = self.inner.res_index.cx();
        for num_remaining in (0..self.inner.depth).rev() {
            let index = self.inner.res_index.as_mut().load();
            wait_group(cx, num_remaining);
            let resource = self.inner.resources_at_mut(index);
            f(resource.reborrow());
            self.inner
                .res_index
                .as_mut()
                .store((index + 1) % self.inner.depth);
        }
        self.inner
    }

    pub fn continue_with<L, CopyFunc, ValueFunc, RolloverFunc>(
        &mut self,
        mut looper: L,
        mut new_cpy_func: CopyFunc,
        mut per_value: ValueFunc,
        mut on_rollover: RolloverFunc,
    ) where
        CopyFunc: FnMut(CpAsyncToken, L::ItemT, Val<'a, M<&mut ElemT, Shared>>),
        ValueFunc: FnMut(Val<'a, R<&ElemT, Shared>>),
        RolloverFunc: FnMut(),
        L: Looper<'a>,
    {
        let n = self.inner.res_index.as_mut().load();
        let cx = self.inner.res_index.cx();
        let max_outstanding = self.inner.depth - 1;
        looper.on_first_n_runtime(n, |item| {
            let index = self.inner.res_index.as_mut().load();
            wait_group(cx, max_outstanding);
            let resource = self.inner.resources_at_mut(index);
            per_value(resource.reborrow());
            new_cpy_func(CpAsyncToken(()), item, resource);
            commit_group(&cx);
            self.inner
                .res_index
                .as_mut()
                .store((index + 1) % self.inner.depth);
        });
        on_rollover();
        looper.for_every_value(|item| {
            let index = self.inner.res_index.as_mut().load();
            wait_group(&cx, max_outstanding);
            let resource = self.inner.resources_at_mut(index);
            per_value(resource.reborrow());
            new_cpy_func(CpAsyncToken(()), item, resource);
            commit_group(&cx);
            self.inner
                .res_index
                .as_mut()
                .store((index + 1) % self.inner.depth);
        });
    }
}
