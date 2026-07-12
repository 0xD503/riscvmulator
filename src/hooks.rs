// type Hook = fn(Box<dyn FnMut(&HookArgs) -> Option<i32> + Send>;
pub(crate) type PeriodicHook = Box<dyn FnMut(&HookArgs) -> Option<i32> + Send>;

pub(crate) struct HookArgs {}
