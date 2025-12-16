use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct CounterProps {
    from: f64,
    to: f64,
    ms: f64
}

#[component]
pub fn Counter(props: CounterProps) -> Element {
    let mut count: Signal<_> = use_signal(|| {
        props.from
    });

    use_future(move || async move {
        let fps: f64 = 60.0;
        let step: f64 = props.ms / fps;
        let steps: u32 = (props.ms / step).max(1.0) as u32;
        for k in 0..=steps {
            let t: f64 = k as f64 / steps as f64;
            let v: f64 = props.from + (props.to - props.from) * t;
            *count.write() = v;
            gloo_future::sleep(time::Duration::from_millis(step as u64)).await;
        }
    });

    rsx!(
        { format!("{:.0}", *count.read()) }
    )
}