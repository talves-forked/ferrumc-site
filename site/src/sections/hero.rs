use leptos::*;
use leptos_tw_ui::components::typography::{Typography, TypographyVariant};

use crate::components::buttons::{DownloadButton, GitHubButton};
use crate::theme::TypographyClass;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center max-lg:px-6 justify-start min-h-[calc(100vh-80px)] pt-[80px]">
          <h1 class="text-white text-center font-roboto text-[30px] sm:text-[40px] xl:text-[72px] font-extrabold leading-[110%] max-w-[994px]">
            <Typography variant=TypographyVariant::SpanInline class={TypographyClass::H1.get()}>Multithreaded</Typography>
            server you never knew you needed
          </h1>
          <h2 class="text-white py-[25px] font-roboto text-[16px] lg:text-[20px] tracking-[160%] opacity-65 max-w-[798px] text-center">
            A high-performance Minecraft server implementation, crafted in Rust for unparalleled speed and efficiency.
          </h2>

          <div class="flex max-lg:flex-col items-center gap-4">
            <DownloadButton />

            <GitHubButton />
          </div>

          <img src="/assets/ferrumc-trans.png" class="w-full max-w-[450px]" />
        </div>
    }
}
