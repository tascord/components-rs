use std::{task::{Context, Poll}, pin::Pin};

use futures_signals::signal::{MutableSignalCloned, Mutable, Signal};

pub struct Mutable2<A, B>(
    (MutableSignalCloned<A>, Mutable<A>),
    (MutableSignalCloned<B>, Mutable<B>),
)
where
    A: Clone,
    B: Clone;
impl<A, B> Mutable2<A, B>
where
    A: Clone,
    B: Clone,
{
    pub fn new(a: Mutable<A>, b: Mutable<B>) -> Self {
        Mutable2((a.signal_cloned(), a), (b.signal_cloned(), b))
    }
}
impl<A, B> Signal for Mutable2<A, B>
where
    A: Clone,
    B: Clone,
{
    type Item = (A, B);

    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let a = Pin::new(&mut self.0 .0).poll_change(cx);
        let b = Pin::new(&mut self.1 .0).poll_change(cx);
        let mut changed = false;

        let left_done = match a {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };

        let right_done = match b {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };

        if changed {
            Poll::Ready(Some((self.0 .1.get_cloned(), self.1 .1.get_cloned())))
        } else if left_done && right_done {
            Poll::Ready(None)
        } else {
            Poll::Pending
        }
    }
}