use super::*;

#[derive(Clone)]
#[derive(PartialEq)]
pub enum Detail {
    Single,
    Double
}

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct CounterProps {
    from: f64,
    to: f64,
    duration: time::Duration,
    detail: Option<Detail>
}

#[component]
pub fn Counter(props: CounterProps) -> Element {
    let mut count: Signal<_> = use_signal(|| {
        props.from
    });

    use_future(move || async move {
        let ms: u128 = props.duration.as_millis();
        let ms: f64 = ms as f64;
        let fps: f64 = 60.0;
        let step: f64 = ms / fps;
        let steps: u32 = (ms / step).max(1.0) as u32;
        for k in 0..=steps {
            let t: f64 = k as f64 / steps as f64;
            let v: f64 = props.from + (props.to - props.from) * t;
            *count.write() = v;
            let step: u64 = step as u64;
            let step: time::Duration = time::Duration::from_millis(step);
            gloo_future::sleep(step).await;
        }
    });

    match props.detail {
        Some(Detail::Single) => rsx! { { format!("{:.1}", *count.read()) } },
        Some(Detail::Double) => rsx! { { format!("{:.2}", *count.read()) } },
        None => rsx! { { format!("{:.0}", *count.read()) } }
    }
}

