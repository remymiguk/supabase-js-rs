use dotenv::dotenv;
use leptos::{logging::log, *};
use leptos_router::*;
use state::global::global::GlobalState;
use std::env;
use web_sys::js_sys::global;
mod components;
mod pages;
mod state;
use crate::components::{BasicTitle, LoginPage};
use crate::pages::{signin::SigninPage, signup::SignupPage};
use components::BasicNavbar;
use leptos_bulma::elements::BButton;
fn main() {
    
    dotenv().ok();

    provide_context(GlobalState::new());
    mount_to_body(|| view! { <Appstarter/> })
}

#[component]
fn Appstarter() -> impl IntoView {
    let state = use_context::<GlobalState>().unwrap();
    view! {
        <Router>
            <A href="/app">
                <BButton>"app"</BButton>
            </A>
            <A href="/auth/login">
                <BButton>"Login"</BButton>
            </A>

            <A href="/auth/signup">
                <BButton>"signup"</BButton>
            </A>

            // <A href="about" class="button">"About"</A>

            <Routes>
                <Route path="absout" view=About/>

                <ProtectedRoute
                    path="/app"
                    redirect_path="/auth/login"
                    condition=move || state.is_auth.get()
                    view=About
                />
                <Route path="/auth/signup" view=SignupPage/>

                <Route path="/auth/login" view=SigninPage/>
            </Routes>
        </Router>
    }
}

#[component]
fn About() -> impl IntoView {
    let d = use_context::<GlobalState>()
        .unwrap()
        .session
        .unwrap()
        .get()
        .user
        .id;

    view! { {d} }
}
