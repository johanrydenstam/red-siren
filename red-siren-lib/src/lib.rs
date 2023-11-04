#![allow(non_snake_case)]

use dioxus::prelude::*;

mod clock;
mod play;
mod splash;

use clock::Clock;
use play::Play;
use splash::Splash;

pub fn app(cx: Scope) -> Element {
    use_shared_state_provider(&cx, || Clock::default());

    let clock = use_shared_state::<Clock>(&cx).unwrap();
    let create_eval = use_eval(&cx);
    let eval = use_memo(&cx, (), |_| {
        create_eval(
            r#"
        let prevTime, currentTime;
        function tick() {
            currentTime = new Date();

            const elapsed = currentTime.valueOf() - (prevTime ? prevTime : currentTime).valueOf();

            dioxus.send(elapsed);
            window.requestAnimationFrame(tick);
            prevTime = currentTime;
        }

        window.requestAnimationFrame(tick);
    "#,
        )
        .unwrap()
    });
    let ticker: &Coroutine<()> = use_coroutine(&cx, |rx| {
        to_owned![clock, eval];
        async move {
            while let Some(elapsed) = eval.recv().await.ok() {
                let mut m_clock = clock.write();
                *m_clock = m_clock.tick(elapsed.as_u64().unwrap());
            }
        }
    });

    cx.render(rsx!(Splash {}))
}
