pub fn set_panic_hook() {
  #[cfg(feature = "debug")]
  {
    pub use console_error_panic_hook::set_once;
    set_once();
  }
}