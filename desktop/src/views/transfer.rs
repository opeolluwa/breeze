use leptos::prelude::{CollectView, ElementChild};
use leptos::{prelude::ClassAttribute, view, IntoView};
use leptos_qr::QrCode;
use thaw::Text;

#[leptos::component]
pub fn TransferUI() -> impl IntoView {
    // let sender_dialog_is_open = true;
    // let receiver_dialog_is_open = false;

    // if sender_dialog_is_open == true {
    //     view! {{ SenderUi()} }
    // } else {
    //     view! {{ ReceiveUI()} }
    // }

    //  view!{   <DefaultUI/>}
    view! { <SenderUI /> }
}

#[leptos::component]
pub fn DefaultUI() -> impl leptos::IntoView {
    view! {
        <div class=" text-center flex flex-col align-center justify-center items-center h-[500px]">
            <div>
                <h1 class="font-medium leading-2 text-2xl text-gray-700 dark:text-gray-400 ">
                    What would you like to do?
                </h1>
                <p class="text-base">Do you want to send or receive files?</p>
                <div class="flex justify-center gap-x-5 items-center mt-8">
                    <button class="flex flex-col items-center ">
                        // on:click=move |_| sender_dialog_is_open.set(true)
                        <div class="dark:bg-gray-700 dark:hover:bg-gray-900/50 bg-gray-200 text-gray-400 hover:bg-app-50 hover:text-app transition-all duration-200  p-4 rounded-xl shadow hover:shadow-none cursor-pointer">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                class="size-6"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M8.25 6.75 12 3m0 0 3.75 3.75M12 3v18"
                                />
                            </svg>

                        </div>
                        <p class="mt-2">Send File</p>
                    </button>
                    <div class="flex flex-col items-center ">
                        <button class=" dark:bg-gray-700 dark:hover:bg-gray-900/50 bg-gray-200 text-gray-400 hover:bg-app-50 hover:text-app transition-all duration-200 p-4 rounded-xl shadow hover:shadow-none cursor-pointer">
                            // on:click=move |_| receiver_dialog_is_open.set(false)
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                class="size-6"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M15.75 17.25 12 21m0 0-3.75-3.75M12 21V3"
                                />
                            </svg>
                        </button>
                        <p class="mt-2">Receive File</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[leptos::component]
pub fn SenderUI() -> impl leptos::IntoView {
    let send_steps = [
        "Create Wi-fi hostpot on yor phone",
        "Connect your Laptop the phone hotspot",
        "Open Filesync mobile",
        "Initialize a receive action",
        "Scan Qr code below",
    ];

    view! {
          <div class="pl-3">
              <Text class="font-medium leading-2 text-xl text-gray-700 dark:text-gray-400">
                  Connect mobile
              </Text>
            <div >
    <ol class="list-decimal list-inside pl-2 ml-4 mt-4 ">
                  {send_steps.map(|step| view! { <li>{step}</li> }).collect_view()}
              </ol>
            </div>
              <div class="w-48 h-48 hidden ">
                  <QrCode
                      data="Hello, World!"
                      ecl=leptos_qr::ECL::Q
                      shape=leptos_qr::Shape::RoundedSquare
                      fg_color="#111111"
                      bg_color="#dddddd"
                  />
              </div>
          </div>
      }
}

#[leptos::component]
pub fn ReceiveUI() -> impl leptos::IntoView {
    view! {
        <div class="text-center flex flex-col align-center justify-center items-center h-[500px]">
            // <p>heheh</p>
            "hh"
        </div>
    }
}
