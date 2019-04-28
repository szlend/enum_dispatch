use enum_dispatch::enum_dispatch;

pub struct SuperFoo<T: Bar> {
    _bar: T,
}

impl<T: Bar> Foo<T> for SuperFoo<T> {
    fn do_something(&mut self, _val: T) {
        println!("SuperFoo");
    }
}

pub struct UltraFoo<T: Bar> {
    _bar: T,
}

impl<T: Bar> Foo<T> for UltraFoo<T> {
    fn do_something(&mut self, _val: T) {
        println!("UltraFoo");
    }
}

pub trait Bar {}

#[enum_dispatch]
pub trait Foo<T: Bar> {
    fn do_something(&mut self, val: T);
}

#[enum_dispatch(Foo)]
pub enum AnyFoo<T: Bar> {
    SuperFoo(SuperFoo<T>),
    UltraFoo(UltraFoo<T>),
}
