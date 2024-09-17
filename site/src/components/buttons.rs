use leptos::*;
use leptos_tw_ui::components::{buttons::button::LinkButton, icons::IconGitHub};

use crate::theme::ButtonVariant;

#[component]
pub fn GitHubButton() -> impl IntoView {
    view! {
        <LinkButton class="flex" href={"https://github.com/ferrumc-rs/ferrumc"} target="_blank" variant={ButtonVariant::Link.get()}>
            <div class="flex items-center justify-center gap-2">
                {"Contribute"}<IconGitHub class="w-5 fill-white" />
            </div>
        </LinkButton>
    }
}

#[component]
pub fn DownloadButton() -> impl IntoView {
    view! {
        <LinkButton href={"/releases"} variant={ButtonVariant::Solid.get()}>{"Download"}</LinkButton>
    }
}
