use crate::state::State;

pub type System = &'static dyn Fn(&mut State) -> ();