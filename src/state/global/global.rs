use leptos::*;
use serde_wasm_bindgen::from_value;
use supabase_js_rs::{Auth, SupabaseClient};
use wasm_bindgen::JsValue;

#[derive(Clone, Debug)]
pub struct GlobalState {
    pub is_auth: RwSignal<bool>,
    pub name: RwSignal<String>,
    pub supaclient: SupabaseClient,
    pub session: Option<RwSignal<super::session::Session>>,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            is_auth: create_rw_signal(false),
            name: create_rw_signal("Bob".to_string()),
            supaclient: supabase_js_rs::create_client("https://fysbtmmglbetbagvawtv.supabase.co", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImZ5c2J0bW1nbGJldGJhZ3Zhd3R2Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3MTU1MTYwOTgsImV4cCI6MjAzMTA5MjA5OH0.rJlx5yd4CgSXHCFt4qKzOfqRatbdYrd_HYtgBl-lsXc"),
            session: Some(create_rw_signal(super::session::Session::new())),
        }
    }
}
