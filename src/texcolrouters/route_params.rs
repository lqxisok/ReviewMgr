use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
pub struct ReviewProjectParams {
    pub proj_id: Option<usize>,
}