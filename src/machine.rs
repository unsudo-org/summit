use super::*;

pub fn use_machine<T>() -> (
    impl Fn() -> T,
    impl Fn() -> T,
    impl FnMut(T, u32) + 'static + Copy
)
where
    T: Default,
    T: Copy,
    T: 'static {
    let now: Signal<_> = use_signal(T::default);
    let mut incoming: Signal<_> = use_signal(T::default);

    let goto = move |state: T, ms: u32| {
        *incoming.write() = state;

        cb::Timeout::new(ms, {
            let mut now: Signal<T> = now.clone();
            move || {
                *now.write() = state.clone();
            }
        })
        .forget();
    };

    let now = move || *now.read();
    let incoming = move || *incoming.read();

    (now, incoming, goto)
}