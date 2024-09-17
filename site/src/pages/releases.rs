use leptos::*;
use leptos_meta::*;

#[component]
pub fn Releases() -> impl IntoView {
    view! {
        <Title text="FerrumC | Releases"/>
        <main>
            <div class="flex flex-col min-h-screen">
                <div class="flex flex-col items-center max-lg:px-6 justify-start min-h-[calc(100vh-80px)] pt-[80px]">
                    <h1 class="text-white text-center font-roboto text-[30px] sm:text-[40px] xl:text-[72px] font-extrabold leading-[110%] max-w-[994px]">
                        Releases
                    </h1>
                    <img src="/assets/ferrumc-trans.png" class="w-full max-w-[450px]" />
                </div>
            </div>
        </main>
    }
}
