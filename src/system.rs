use crate::world::World;

pub enum System {
    Start (fn(&mut World) -> ()),
    Uptade (fn(&mut World) -> ()),
    Exit (fn(&mut World) -> ()),
}