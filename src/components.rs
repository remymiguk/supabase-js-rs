use std::env;

use leptos::html::{Input, P};
use leptos::logging::log;
use leptos::*;
use leptos_router::*;

use leptos_bulma::components::{
    BNavbar, BNavbarBrand, BNavbarBurger, BNavbarDivider, BNavbarEnd, BNavbarItem,
    BNavbarItemDropdown, BNavbarMenu, BNavbarStart,
};
use leptos_bulma::form::{BControl, BField, BInput};

use leptos_bulma::elements::{BButton, BSubtitle, BTitle};
use supabase_js_rs::Credentials;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::console::log_1;
use web_sys::wasm_bindgen::JsValue;
use web_sys::{MouseEvent, SubmitEvent};

use crate::state::global::global::GlobalState;
use crate::state::types::SupabaseClient_response;
use crate::state::{self, global};

#[component]
pub fn BasicTitle() -> impl IntoView {
    view! {
        <BTitle is=1>"This is the title"</BTitle>
        <BSubtitle is=2>"This is the subtitle"</BSubtitle>
    }
}

#[component]
pub fn BasicNavbar() -> impl IntoView {
    let burger_is_active = create_rw_signal(false);

    let show_alert = move |_| {
        let _ = window().alert_with_message("Item clicked");
    };

    view! {
        <BNavbar class="is-primary has-shadow block">
            <BNavbarBrand>
                <BNavbarItem class="is-size-4" on:click=show_alert>
                    "Leptos Bulma"
                </BNavbarItem>

                <BNavbarBurger is_active=burger_is_active/>
            </BNavbarBrand>

            <BNavbarMenu is_active=burger_is_active>
                <BNavbarStart>
                    <BNavbarItem on:click=show_alert>"Item"</BNavbarItem>
                </BNavbarStart>

                <BNavbarEnd>
                    <BNavbarItem on:click=show_alert>"Item"</BNavbarItem>

                    <BNavbarItemDropdown
                        dropdown_class="is-right"
                        trigger=|| {
                            view! { <span class="has-text-weight-bold">"Dropdown item"</span> }
                        }
                    >

                        <BNavbarItem on:click=show_alert>"Item"</BNavbarItem>
                        <BNavbarDivider/>
                        <BNavbarItem on:click=show_alert>"Item"</BNavbarItem>
                    </BNavbarItemDropdown>
                </BNavbarEnd>
            </BNavbarMenu>
        </BNavbar>
    }
}

async fn login(email: String, password: String) {
    let y = use_navigate();

    let state = use_context::<GlobalState>().unwrap();

    let auth_event_callback: Closure<dyn FnMut(JsValue, JsValue)> =
        Closure::new(move |event: JsValue, session: JsValue| {
            state.is_auth.set(true);
            y("/app", Default::default());
        });

    state
        .supaclient
        .auth()
        .on_auth_state_change(&auth_event_callback);

    let user_login = &state
        .supaclient
        .auth()
        .sign_in_with_password(Credentials {
            password,
            email,
            options: Default::default(),
        })
        .await;

    auth_event_callback.forget();
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let state = use_context::<GlobalState>().unwrap();

    let email_noderef = create_node_ref::<Input>();
    let pwd_noderef = create_node_ref::<Input>();

    let action2 =
        create_action(|input: &(String, String)| login(input.0.to_string(), input.1.to_string()));

    let on_submit = move |ev: MouseEvent| {
        // stop the page from reloading!
        ev.prevent_default();
        // call the login function

        let email_node = email_noderef.get().expect("email node not loaded");
        let pwd_node = pwd_noderef.get().expect("pwd node not loaded");
        action2.dispatch((email_node.value(), pwd_node.value()));
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
                            <form>

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
                                            placeholder="Your Password"
                                            class="input is-large"
                                            node_ref=pwd_noderef
                                            input_type="password"
                                        />
                                    </BControl>

                                </BField>

                                <BButton
                                    on:click=on_submit
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

#[component]
fn TemplateApp() -> impl IntoView {
    let d = use_context::<GlobalState>().unwrap().name;

    view! {
        <BasicTitle/>
        <p>{d.get()}</p>
    }
}
