#![feature(precise_capturing)]
//~^ WARN the feature `precise_capturing` is incomplete

fn hello(_: impl Sized + use<>) {}
//~^ ERROR `use<...>` precise capturing syntax not allowed in argument-position `impl Trait`

fn main() {}
