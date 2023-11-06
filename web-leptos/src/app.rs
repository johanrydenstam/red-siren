mod intro;
mod core;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use shared::Event;
use crate::error_template::{AppError, ErrorTemplate};

#[component]
pub fn RootComponent() -> impl IntoView {
    // let core = core::new();
    // let (view, render) = create_signal(core.view());
    // let (event, set_event) = create_signal(Event::Reset);

    // create_effect(move |_| {
    //     core::update(&core, event.get(), render);
    // });
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/red-siren.css"/>
        <Title text="Red Siren"/>

        
        <Router fallback=|| {
          let mut outside_errors = Errors::default();
          outside_errors.insert_with_default_key(AppError::NotFound);
          view! {
              <ErrorTemplate outside_errors/>
          }
          .into_view()
      }>
          <main>
              <Routes>
                  <Route path="" view=intro::IntroComponent/>
              </Routes>
          </main>
      </Router>
    }
}