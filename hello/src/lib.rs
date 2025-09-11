extern crate alloc;

// Funzione exportata "run"
#[no_mangle]
pub extern "C" fn run() {
    // se il modulo è compilato con target wasm32-wasi e istanziato con WASI che eredita stdout,
    // questa stampa comparirà nello stdout del processo host.
    // Tuttavia, il macro println! usa la std; qui ti mostro il modo semplice:
    // (Se usi std normalmente, puoi usare println! direttamente e compilare con wasm32-wasi)
    #[cfg(target_family = "wasm")]
    {
        // se compilato con std: println! funziona
    }
}