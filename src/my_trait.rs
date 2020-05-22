use std::fmt::Debug;

pub trait Fly {
    fn fly(&self) -> bool;
}

#[derive(Debug)]
pub struct Duck;

#[derive(Debug)]
pub struct Pig;

impl Fly for Duck
{
    fn fly(&self) -> bool {
        true
    }
}

impl Fly for Pig
{
    fn fly(&self) -> bool
    {
        false
    }
}

pub fn can_fly_static(s: impl Fly) -> bool
{
    s.fly()
}

#[allow(dead_code)]
pub fn can_fly_generic_static<T>(s: T) -> bool where T: Fly
{
    s.fly()
}

pub fn can_fly(s: impl Fly + Debug) -> impl Fly
{
    if s.fly() {
        println!("{:?} can fly!", s)
    } else {
        println!("{:?} cannot fly!", s);
    }

    s
}

pub fn dny_can_fly(s: impl Fly + Debug + 'static) -> Box<dyn Fly>
{
    if s.fly() {
        println!("{:?} can fly!", s)
    } else {
        println!("{:?} cannot fly!", s)
    }

    Box::new(s)
}
