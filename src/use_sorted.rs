use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
use std::cmp::Ordering;
use std::ops::DerefMut;

/// Reactive sort of iterable
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_sorted)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::use_sorted;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let source = vec![10, 3, 5, 7, 2, 1, 8, 6, 9, 4];
/// let sorted: Signal<Vec<i32>> = use_sorted(source); // [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
/// #
/// # view! { }
/// # }
/// ```
///
/// You can also sort by key or with a compare function.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::{use_sorted_by, use_sorted_by_key};
/// #
/// #[derive(Clone, PartialEq)]
/// pub struct Person {
///     pub name: String,
///     pub age: u16,
/// }
///
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let source = vec![
///     Person {
///         name: "John".to_string(),
///         age: 40,
///     },
///     Person {
///         name: "Jane".to_string(),
///         age: 20,
///     },
///     Person {
///         name: "Joe".to_string(),
///         age: 30,
///     },
///     Person {
///         name: "Jenny".to_string(),
///         age: 22,
///     },
/// ];
///
/// // sort by key
/// let sorted: Signal<Vec<Person>> = use_sorted_by_key(
///     source.clone(),
///     |person: &Person| person.age,
/// );
///
/// // sort with compare function
/// let sorted: Signal<Vec<Person>> = use_sorted_by(
///     source,
///     |p1: &Person, p2: &Person| p1.age.cmp(&p2.age),
/// );
/// #
/// # view! { }
/// # }
/// ```
///
/// Please note that these two ways of sorting are equivalent.
pub fn use_sorted<S, I, T>(iterable: S) -> Signal<I>
where
    S: Into<Signal<I>>,
    T: Ord,
    I: DerefMut<Target = [T]> + Clone + PartialEq + Send + Sync + 'static,
{
    let iterable = iterable.into();

    Signal::derive(move || {
        let mut iterable = iterable.get();
        iterable.sort();
        iterable
    })
}

/// Version of [`use_sorted`] with a compare function.
pub fn use_sorted_by<S, I, T, F>(iterable: S, cmp_fn: F) -> Signal<I>
where
    S: Into<Signal<I>>,
    I: DerefMut<Target = [T]> + Clone + PartialEq + Send + Sync + 'static,
    F: FnMut(&T, &T) -> Ordering + Clone + Send + Sync + 'static,
{
    let iterable = iterable.into();

    Signal::derive(move || {
        let mut iterable = iterable.get();
        iterable.sort_by(cmp_fn.clone());
        iterable
    })
}

/// Version of [`use_sorted`] by key.
pub fn use_sorted_by_key<S, I, T, K, F>(iterable: S, key_fn: F) -> Signal<I>
where
    S: Into<Signal<I>>,
    I: DerefMut<Target = [T]> + Clone + PartialEq + Send + Sync + 'static,
    K: Ord,
    F: FnMut(&T) -> K + Clone + Send + Sync + 'static,
{
    let iterable = iterable.into();

    Signal::derive(move || {
        let mut iterable = iterable.get();
        iterable.sort_by_key(key_fn.clone());
        iterable
    })
}
