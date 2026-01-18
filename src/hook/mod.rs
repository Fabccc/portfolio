use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;

pub fn use_filter_find<S, I, T, P>(iterable: S, predicate: P) -> Signal<Option<T>>
where
    S: Into<Signal<I>>,
    I: AsRef<[T]> + Clone + PartialEq + Send + Sync + 'static,
    P: Fn(&T) -> bool + Clone + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
{
    let iterable = iterable.into();

    Signal::derive(move || {
        let iterable = iterable.get();
        iterable
            .as_ref()
            .iter()
            .find(|&item| predicate(item))
            .cloned()
    })
}
