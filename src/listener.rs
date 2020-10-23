use crate::tcp;
use iced::{futures, Subscription};


pub struct Listener(String);

impl<H, I> iced_native::subscription::Recipe<H, I> for Listener
where
    H: std::hash::Hasher,
{
    type Output = tcp::Connection;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
        self.0.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        use iced::futures::stream::StreamExt;

        tcp::listen(self.0).boxed()
    }
}
