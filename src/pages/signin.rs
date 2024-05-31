use std::{fmt, string};

use crate::state::global::global::GlobalState;
use crate::state::global::session::{self, Session};
use leptos::html::Input;
use leptos::svg::set;
use leptos::*;
use leptos_bulma::components::{
    BModal, BModalContent, BNavbar, BNavbarBrand, BNavbarBurger, BNavbarDivider, BNavbarEnd,
    BNavbarItem, BNavbarItemDropdown, BNavbarMenu, BNavbarStart,
};
use leptos_bulma::elements::{BBox, BButton};
use leptos_bulma::form::{BControl, BField, BInput};
use leptos_router::*;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_wasm_bindgen::{from_value, to_value};
use supabase_js_rs::Credentials;
use tracing::event;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::console::{log, log_1};
use web_sys::js_sys::global;
use web_sys::wasm_bindgen::JsValue;
use web_sys::{MouseEvent, SubmitEvent};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    #[serde(rename = "__isAuthError")]
    pub is_auth_error: bool,
    pub name: String,
    pub status: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberSignUpValues {
    pub name: String,
}

#[derive(Clone, Debug)]
enum EventAuth {
    SIGNED_IN,
}

#[component]
pub fn SigninPage() -> impl IntoView {
    let modal_error = create_rw_signal(false);
    let state = use_context::<GlobalState>().unwrap();
    let session = create_rw_signal(state.clone().session);
    let email_value_signal = create_rw_signal(String::default());
    let pwd_value_signal = create_rw_signal(String::default());
    let email_noderef = create_node_ref::<Input>();
    let pwd_noderef = create_node_ref::<Input>();

    let login_action: Action<(String, String), ()> =
        create_action(move |(email, password): &(String, String)| {
            let router = use_navigate();
            let auth_callback = Closure::new(move |event: JsValue, session: JsValue| {
                let session_from_js_value: Session = from_value(session).unwrap();
                let event_from_js_value: String = from_value(event).unwrap();
                // dbg!(&event);
                match event_from_js_value.as_str() {
                    "SIGNED_IN" => {
                        state.is_auth.set(true);
                        state.session.unwrap().set(session_from_js_value);
                        router("/app", Default::default());
                    }
                    _ => {}
                }
            });

            state.supaclient.auth().on_auth_state_change(&auth_callback);

            auth_callback.forget();
            let username = email.to_string();
            let password = password.to_string();
            let state = use_context::<GlobalState>().unwrap();
            let login_router = use_navigate();

            async move {
                let credentials = Credentials {
                    email: username,
                    password: password,
                    options: Default::default(),
                };

                let res = &state
                    .clone()
                    .supaclient
                    .auth()
                    .sign_in_with_password(credentials)
                    .await;

                if let Ok(session_response) = res {
                    let session: Session = from_value(session_response.clone()).unwrap();
                    state.is_auth.set(true);
                    state.session.unwrap().set(session);
                    login_router("/app", Default::default());
                } else {
                    modal_error.set(true);
                }
            }
        });

    let dispatch_action = move || {
        login_action.dispatch((
            email_noderef.get().unwrap().value(),
            pwd_noderef.get().unwrap().value(),
        ))
    };

    //    let pwd_noderef: NodeRef<Input> = create_node_ref::<Input>();
    //    let pwd_noderef_value= pwd_noderef.get().expect("pwd node not loaded").value();

    // our resource
    // let async_data = create_resource(move || (),   |_| async move {

    //         let state = use_context::<GlobalState>().unwrap();
    //            let credentials = Credentials {
    //                email: email_noderef.get().expect("Cannot get email from login form").value(),
    //                password: pwd_noderef.get().expect("Cannot get email from login form").value()
    //            };
    //            let res = state.clone().supaclient.auth()
    //                .sign_in_with_password(credentials)
    //                .await;

    // });

    //    let auth_callback = Closure::new(move |event: JsValue, session: JsValue| {
    //        log_1(&event);
    //    });

    //    state.supaclient.auth().on_auth_state_change(&auth_callback);

    //    auth_callback.forget();
    //    let once = create_resource(|| (), |_| async move {  let credentials = Credentials {
    //     email: email_noderef_value.clone(),
    //     password: pwd_noderef_value.clone()
    // };
    // let res = state.clone().supaclient.auth()
    //     .sign_in_with_password(credentials)
    //     .await;
    // log_1(&res.unwrap()); });

    //    let sign_in = move |MouseEvent| {

    //        spawn_local(async move {
    // let email = email_noderef.get().unwrap().clone().value();
    //     let password = pwd_noderef.get().unwrap().clone().value();
    //     let state = use_context::<GlobalState>().unwrap();
    //        let credentials = Credentials {
    //            email: email.clone(),
    //            password: password.clone()
    //        };
    //        let res = state.clone().supaclient.auth()
    //            .sign_in_with_password(credentials)
    //            .await;
    //        log_1(&res.unwrap());
    // }
    //    )
    //    };

    let on_submit = move |ev: MouseEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        dispatch_action();

        //    action2.dispatch((email_node.value(), pwd_node.value()));
    };

    view! {
        <section class="hero is-prrimary is-fullheight">
            <div class="hero-body">
                <div class="container has-text-centered">
                    <div class="column is-4 is-offset-4">
                        <h3 class="title has-text-black">{"Login"}</h3>
                        <p class="subtitle has-text-black">{"Please login to proceed."}</p>
                        <BBox>
                            <BModal is_active=modal_error>
                                <BModalContent>
                                    <BBox class="has-text-centered">"Hello, World!"</BBox>
                                </BModalContent>
                            </BModal>
                            // <figure class="avatar">
                            // <img src="https://via.placeholder.com/150"/>
                            // </figure>
                            <form on:submit=|ev| {
                                ev.prevent_default()
                            }>
                                . <BField>
                                    <BControl>
                                        <BInput
                                            placeholder="Your Email"
                                            class="input is-large"
                                            node_ref=email_noderef
                                            input_type="email"
                                        />
                                    </BControl>

                                </BField>
                                <BField>
                                    <BControl>
                                        <BInput
                                            placeholder="Your Password"
                                            class="input is-large"
                                            node_ref=pwd_noderef
                                            input_type="password"
                                        />
                                    </BControl>

                                </BField>
                                <BButton
                                    on:click=move |_| dispatch_action()

                                    class="button is-block is-info is-large is-fullwidth"
                                >
                                    {"Login"}
                                    <i class="fa fa-sign-in" aria-hidden="true"></i>
                                </BButton>
                            </form>
                        </BBox>
                        <p class="has-text-grey">
                            <a href="../">{"Sign Up | "}</a>
                            <a href="../">{"Forgot Password | "}</a>
                            <A href="/app">{"Need Help? | "}</A>
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}
