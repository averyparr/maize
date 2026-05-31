use crate::{
    intrinsics::{Intrinsic, impl_intrinsics},
    tipe::{A, Ty},
    val::Val,
};

impl_intrinsics!(
    MapAddress: "llvm.nvvm.mapa.shared.cluster"(A<*const u8, 3>, u32) -> A<*const u8, 7>,
);

impl<T: Ty> Val<A<*mut T, 3>> {
    pub fn map_address_to_cluster(self, cluster: Val<u32>) -> Val<A<*mut T, 7>> {
        MapAddress.call((self.bitcast(), cluster)).bitcast()
    }
    pub fn mapa(self, cluster: Val<u32>) -> Val<A<*mut T, 7>> {
        self.map_address_to_cluster(cluster)
    }
}

impl<T: Ty> Val<A<*const T, 3>> {
    pub fn map_address_to_cluster(self, cluster: Val<u32>) -> Val<A<*const T, 7>> {
        MapAddress.call((self.bitcast(), cluster)).bitcast()
    }
    pub fn mapa(self, cluster: Val<u32>) -> Val<A<*const T, 7>> {
        self.map_address_to_cluster(cluster)
    }
}
