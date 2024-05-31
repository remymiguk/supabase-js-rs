use crate::state::global::global::GlobalState;
use crate::state::global::session::Session;
use leptos::html::Input;
use leptos::*;
use leptos_bulma::components::{
    BNavbar, BNavbarBrand, BNavbarBurger, BNavbarDivider, BNavbarEnd, BNavbarItem,
    BNavbarItemDropdown, BNavbarMenu, BNavbarStart,
};
use leptos_bulma::elements::BButton;
use leptos_bulma::form::{BControl, BField, BInput};
use leptos_router::*;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_wasm_bindgen::{from_value, to_value};
use supabase_js_rs::Credentials;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::console::log_1;
use web_sys::js_sys::global;
use web_sys::wasm_bindgen::JsValue;
use web_sys::{MouseEvent, SubmitEvent};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberSignUpValues {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberSignUpData {
    pub data: MemberSignUpValues,
}

#[component]
pub fn SignupPage() -> impl IntoView {
    let modal_error = create_rw_signal(false);

    let state = use_context::<GlobalState>().unwrap();

    let email_noderef = create_node_ref::<Input>();
    let pwd_noderef = create_node_ref::<Input>();
    let name_noderef = create_node_ref::<Input>();

    let signup_action: Action<(String, String, String), ()> =
        create_action(move |(email, password, name): &(String, String, String)| {
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



            let user_data_dat = &MemberSignUpData {
                data: MemberSignUpValues{ name: name.to_string()},
            };


            let credentials = Credentials {
                email: username,
                password: password,
                options: to_value(&user_data_dat).unwrap(),
            };
            async move {
                

                let res = &state.clone().supaclient.auth().sign_up(credentials).await;

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
        signup_action.dispatch((
            email_noderef.get().unwrap().value(),
            pwd_noderef.get().unwrap().value(),
            name_noderef.get().unwrap().value(),
        ))
    };

    let on_submit = move |ev: MouseEvent| {
        // stop the page from reloading!
        ev.prevent_default();
        // call the login function

        let email_node = email_noderef.get().expect("email node not loaded");
        let pwd_node: HtmlElement<Input> = pwd_noderef.get().expect("pwd node not loaded");
        let name_node: HtmlElement<Input> = name_noderef.get().expect("pwd node not loaded");
    };

    view! {
        <section class="hero is-success is-fullheight">
            <div class="hero-body">
                <div class="container has-text-centered">
                    <div class="column is-4 is-offset-4">
                        <h3 class="title has-text-black">{"Login"}</h3>
                        <p class="subtitle has-text-black">{"Please login to proceed."}</p>
                        <div class="box">
                            <figure class="avatar">
                                <img src="https://via.placeholder.com/150"/>
                            </figure>
                            <form on:submit=|ev| ev.prevent_default()>

                                <BField>
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
                                        placeholder="Your Name"
                                        class="input is-large"
                                        node_ref=name_noderef
                                        input_type="text"
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
                        </div>
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
