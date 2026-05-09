pub mod file_picker;
pub mod file_saver;
pub mod task;

#[cfg(not(target_arch = "wasm32"))]
pub trait NativeOnlySend: Send {}
#[cfg(not(target_arch = "wasm32"))]
impl<T: Send> NativeOnlySend for T {}

#[cfg(target_arch = "wasm32")]
pub trait NativeOnlySend {}
#[cfg(target_arch = "wasm32")]
impl<T> NativeOnlySend for T {}

pub fn spawn(fut: impl Future<Output = ()> + 'static + NativeOnlySend) {
    #[cfg(not(target_arch = "wasm32"))]
    std::thread::spawn(|| pollster::block_on(fut));

    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(fut);
}
