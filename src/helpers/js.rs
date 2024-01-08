#[macro_export]
macro_rules! console_log {
   ( $( $arg:expr ),* ) => {
    {
        $(
            web_sys::console::log_1(&format!( "{:?}", $arg ).into());
        )*
    }
   }
}

#[macro_export]
macro_rules! console_warn {
    ( $( $arg:expr ),* ) => {
     {
          $(
                web_sys::console::warn_1(&format!( "{:?}", $arg ).into());
          )*
     }
    }
}
