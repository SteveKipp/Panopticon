use crate::tcp;
use iced::{futures, Subscription};

pub fn listen(addr: &'static str) -> iced::Subscription<tcp::Connection> {
    Subscription::from_recipe(Listener{
        addr: addr
    })
}

pub struct Listener{
   pub addr: &'static str,
}

impl<H, I> iced_native::subscription::Recipe<H, I> for Listener
where
    H: std::hash::Hasher,
{
    type Output = tcp::Connection;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
        self.addr.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        use iced::futures::stream::StreamExt;

        tcp::listen(self.addr).boxed()
    }
}
