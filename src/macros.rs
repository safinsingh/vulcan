#[macro_export]
macro_rules! linker_var {
   ($var:ident) => {
      extern C {
         static $var: ();
      }
   };
   ($($var:ident),*) => {
      $($crate::linker_var!($var);)*
   };
}
