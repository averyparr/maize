use std::mem::MaybeUninit;

use maize_core::{
    collective_mbar_init,
    control_flow::{If, While},
    intrinsics::cuda::{
        cuda,
        mbar::{
            Mbar, MbarArrive, MbarArriveOrdering, MbarScope, MbarToken, MbarWaitOrdering,
            TimeLimitNs, mbar_arrive, mbar_test_wait_parity, mbar_try_wait_parity,
        },
    },
    reflect_on_struct,
    tipe::{A, Ty},
    val::{S, Val},
};

reflect_on_struct!(
    #[derive(Clone, Copy)]
    struct PipeState {
        index: u32,
        stage: u32,
        phase: u32,
    }
);

fn destructure(val: Val<PipeState>) -> (Val<u32>, Val<u32>, Val<u32>) {
    (
        PipeState::index().field(val.copy()),
        PipeState::stage().field(val.copy()),
        PipeState::phase().field(val.copy()),
    )
}

enum Role {
    Producer = 0,
    Consumer = 1,
}

struct TMAPipe<T: 'static> {
    mbars: Val<A<*mut [[Mbar; 2]; 0], 3>>,
    data: Val<A<*mut [T; 0], 3>>,
    stages: usize,
    state: Val<S<PipeState>>,
}

pub struct TMAProducer<T: 'static> {
    pipe: TMAPipe<T>,
}

pub struct TMAConsumer<T: 'static> {
    pipe: TMAPipe<T>,
}

pub fn new_producer_consumer<T: 'static + Ty, const N: usize>(
    mbars: Val<A<&'static mut MaybeUninit<[[Mbar; 2]; N]>, 3>>,
    mut data: Val<A<&'static mut MaybeUninit<[T; N]>, 3>>,
    producer_arrive_count: u32,
    consumer_arrive_count: u32,
) -> (TMAProducer<T>, TMAConsumer<T>) {
    let mut mbars = collective_mbar_init(
        mbars,
        data.constant([producer_arrive_count, consumer_arrive_count]),
    );
    let mut pipe_state = |index, stage, phase| {
        let state = PipeState {
            index,
            stage,
            phase,
        };
        TMAPipe {
            mbars: mbars.reborrow().as_mut_ptr().ptr_cast(),
            data: data.reborrow().as_mut_ptr().ptr_cast(),
            stages: N as _,
            state: mbars.constant(state).with_storage(),
        }
    };
    let producer = TMAProducer {
        pipe: pipe_state(0, 0, 1),
    };
    let consumer = TMAConsumer {
        pipe: pipe_state(0, 0, 0),
    };
    (producer, consumer)
}

impl<T: Ty + 'static> TMAPipe<T> {
    fn current_stage(&self) -> Val<u32> {
        PipeState::stage().field_ref(self.state.get_ref()).load()
    }
    fn current_phase(&self) -> Val<u32> {
        PipeState::phase().field_ref(self.state.get_ref()).load()
    }
    fn past_prefill(&self) -> Val<bool> {
        PipeState::index()
            .field_ref(self.state.get_ref())
            .load()
            .ge(self.state.constant(self.stages as _))
    }

    fn mbar_at_current_stage(&self, role: Role) -> Val<A<*mut Mbar, 3>> {
        self.mbars
            .copy()
            .index(self.current_stage().cvt())
            .index_static(role as usize)
    }

    fn mbar_try_wait(
        &self,
        role: Role,
        sem: MbarWaitOrdering,
        scope: MbarScope,
        time_hint: Option<TimeLimitNs>,
    ) -> Val<bool> {
        let parity = self.current_phase();
        let mbar = self.mbar_at_current_stage(role);
        let mbar = unsafe { mbar.assume_mut() };
        mbar_try_wait_parity(mbar, parity, sem, scope, time_hint)
    }

    fn mbar_test_wait(&self, role: Role, sem: MbarWaitOrdering, scope: MbarScope) -> Val<bool> {
        let parity = self.current_phase();
        // So we wait on the producer to arrive
        let mbar = self.mbar_at_current_stage(role);
        let mbar = unsafe { mbar.assume_mut() };
        mbar_test_wait_parity(mbar, parity, sem, scope)
    }

    fn consumer_release(&self, sem: MbarArriveOrdering, scope: MbarScope) -> Val<MbarToken> {
        let arrive = MbarArrive::Thread;
        let mbar = unsafe { self.mbar_at_current_stage(Role::Consumer).assume_mut() };
        mbar_arrive(mbar, arrive, sem, scope, false)
    }

    fn producer_release(&self, sem: MbarArriveOrdering, scope: MbarScope) -> Val<MbarToken> {
        let tx_bytes = T::size().try_into().expect("usize -> u32 overflow");
        let tx_bytes = self.state.constant(tx_bytes);
        let arrive = MbarArrive::ExpectTx(tx_bytes);
        let mbar = unsafe { self.mbar_at_current_stage(Role::Producer).assume_mut() };
        mbar_arrive(mbar, arrive, sem, scope, false)
    }

    fn data_ptr(&self) -> Val<A<*mut T, 3>> {
        let (_, stage, _) = destructure(self.state.get_ref().load());
        self.data.copy().index(stage.cvt())
    }

    fn advance(&mut self) {
        let idx = PipeState::index().field_ref(self.state.get_ref());
        let stage = PipeState::stage().field_ref(self.state.get_ref());
        let phase = PipeState::phase().field_ref(self.state.get_ref());
        let next_idx = idx.constant(1) + idx.load();
        let curr_stage = stage.load();
        let curr_phase = phase.load();
        let one = idx.constant(1);
        let zero = idx.constant(0);
        let at_transition = curr_stage.copy().eq(stage.constant((self.stages - 1) as _));
        let next_stage = If(at_transition.copy())
            .then(|| zero)
            .or_else(|| one.copy() + curr_stage);
        let next_phase = If(at_transition)
            .then(|| curr_phase.copy() ^ one)
            .or_else(|| curr_phase);
        PipeState::index()
            .field_mut(self.state.get_mut())
            .store(next_idx);
        PipeState::stage()
            .field_mut(self.state.get_mut())
            .store(next_stage);
        PipeState::phase()
            .field_mut(self.state.get_mut())
            .store(next_phase);
    }
}

impl<T: Ty> TMAConsumer<T> {
    const WAIT_ON: Role = Role::Producer;
    fn consumer_try_wait(&self, scope: MbarScope, time_hint: Option<TimeLimitNs>) -> Val<bool> {
        self.pipe
            .mbar_try_wait(Self::WAIT_ON, MbarWaitOrdering::Acquire, scope, time_hint)
    }

    pub fn step_scoped_with_hint(
        &mut self,
        scope: MbarScope,
        time_hint: Option<TimeLimitNs>,
        mut f: impl FnMut(Val<A<&T, 3>>),
    ) {
        While(|| !self.consumer_try_wait(scope, time_hint)).run(|| ());
        If(self.pipe.past_prefill()).then(|| {
            self.pipe
                .state
                .fn_ref()
                .intrinsic(cuda)
                .cp_async_bulk_wait_group_reads_only((self.pipe.stages - 1) as _)
        });
        let data = unsafe { self.pipe.data_ptr().assume_ref() };
        f(data);
        self.pipe
            .consumer_release(MbarArriveOrdering::Release, scope);
        self.pipe.advance();
    }
    pub fn step(&mut self, f: impl FnMut(Val<A<&T, 3>>)) {
        self.step_scoped_with_hint(MbarScope::Cta, None, f);
    }
    pub fn cluster_step(&mut self, f: impl FnMut(Val<A<&T, 3>>)) {
        self.step_scoped_with_hint(MbarScope::Cluster, None, f);
    }

    pub fn test_wait(&self, scope: MbarScope) -> Val<bool> {
        self.pipe
            .mbar_test_wait(Self::WAIT_ON, MbarWaitOrdering::Acquire, scope)
    }
}

impl<T: Ty> TMAProducer<T> {
    const WAIT_ON: Role = Role::Consumer;

    fn producer_try_wait(&self, scope: MbarScope, time_hint: Option<TimeLimitNs>) -> Val<bool> {
        self.pipe
            .mbar_try_wait(Self::WAIT_ON, MbarWaitOrdering::Acquire, scope, time_hint)
    }
    pub fn step_scoped_with_hint(
        &mut self,
        scope: MbarScope,
        time_hint: Option<TimeLimitNs>,
        mut f: impl FnMut(Val<A<&mut Mbar, 3>>, Val<A<&mut T, 3>>),
    ) {
        If(self.pipe.past_prefill())
            .then(|| While(|| !self.producer_try_wait(scope, time_hint)).run(|| ()));

        let data = unsafe { self.pipe.data_ptr().assume_mut() };
        let mbar = unsafe { self.pipe.mbar_at_current_stage(Role::Producer).assume_mut() };
        f(mbar, data);
        self.pipe
            .producer_release(MbarArriveOrdering::Release, scope);
        self.pipe.advance();
    }
    pub fn step(&mut self, f: impl FnMut(Val<A<&mut Mbar, 3>>, Val<A<&mut T, 3>>)) {
        self.step_scoped_with_hint(MbarScope::Cta, None, f);
    }
    pub fn cluster_step(&mut self, f: impl FnMut(Val<A<&mut Mbar, 3>>, Val<A<&mut T, 3>>)) {
        self.step_scoped_with_hint(MbarScope::Cluster, None, f);
    }
    pub fn test_wait(&self, scope: MbarScope) -> Val<bool> {
        self.pipe
            .mbar_test_wait(Self::WAIT_ON, MbarWaitOrdering::Acquire, scope)
    }
}
