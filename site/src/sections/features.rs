use leptos::*;
use leptos_tw_ui::components::icons::IconCheckMark;

use crate::components::buttons::{DownloadButton, GitHubButton};

#[component]
pub fn Features() -> impl IntoView {
    let features = [
        "Fully multithreaded and thread-safe architecture",
        "High performance and memory efficiency",
        "Customizable server list",
        "World importing from vanilla Minecraft",
        "Powerful Entity Component System",
        "Lightning-fast world loading",
        "Compatible with Vanilla Minecraft clients (1.20.1)",
    ];

    view! {
      <div class="mb-[100px]">
        <div class="pt-[80px] flex flex-col items-center justify-center container mx-auto max-lg:px-6">
          <p class="text-white z-10 font-roboto text-[24px] md:text-[32px] lg:text-[48px] font-extrabold max-w-[900px] text-center leading-[110%]">
            Fully multi-threaded and lightning fast world loading speed
          </p>
          <img src="/assets/chunk_loading.webp" class="mt-[49px] mb-[80px] rounded-xl" />
        </div>

        <div class="max-lg:py-[20px] py-[80px] flex gap-[80px] max-xl:items-center max-xl:justify-center lg:container lg:mx-auto max-lg:px-6">
          <img src="/assets/chunk_loading.webp" class="w-full max-w-[674px] rounded-xl max-xl:hidden" />

          <div class="flex flex-col items-start justify-center">
            <p class="font-roboto text-[34px] lg:text-[48px] font-extrabold tracking-[110%] text-white">
              Powerful server.
            </p>
            <p class="text-primary font-roboto text-[34px] lg:text-[48px] font-extrabold tracking-[110%] -mt-4">
              Fully Multithreaded.
            </p>

            <p class="text-white font-roboto text-[18px] leading-[160%] opacity-65">
              Here are the key features we have for this server:
            </p>

            <div class="mt-4">
            {features.into_iter()
              .map(|feature| view! { <div class="flex items-center justify-start text-white gap-2">
                    <IconCheckMark class="w-4 fill-primary" />
                    <p>{feature}</p>
                  </div> })
              .collect::<Vec<_>>()}
            </div>

            <div class="max-sm:w-full mt-[20px]">
              <DownloadButton />
            </div>
          </div>
        </div>

        <div class="flex flex-col items-center justify-center py-[80px] lg:container max-lg:px-6 lg:mx-auto">
          <p class="text-white font-roboto max-lg:text-center font-extrabold max-lg:text-[30px] lg:text-[46px] leading-[110%]">
            Want to start contributing?
          </p>
          <p class="text-primary font-roboto max-lg:text-center font-extrabold max-lg:text-[30px] lg:text-[46px] leading-[110%]">
            Check us out on GitHub
          </p>
          <div class="max-sm:w-full mt-[20px]">
            <GitHubButton />
          </div>

        </div>
      </div>
    }
}
