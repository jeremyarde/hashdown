use dioxus::prelude::*;
use fermi::use_atom_state;

use crate::mainapp::APP;

pub fn Login(cx: Scope) -> Element {
    let app_state = use_atom_state(cx, APP);

    let onsubmit = move |evt: FormEvent| {
        cx.spawn(async move {
            let resp = reqwest::Client::new() 
                .post("http://localhost:8080/login")
                .form(&[
                    ("username", &evt.values["username"]),
                    ("password", &evt.values["password"]),
                ])
                .send()
                .await;

            match resp {
                // Parse data from here, such as storing a response token
                Ok(_data) => println!("Login successful!"),

                //Handle any errors from the fetch here
                Err(_err) => {
                    println!("Login failed - you need a login server running on localhost:8080.")
                }
            }
        });
    };
    cx.render(rsx! {
        if app_state.show_login {
            rsx!{
                h1 { "Login" }
            form {
                onsubmit: onsubmit,
                prevent_default: "onsubmit", // Prevent the default behavior of <form> to post
                input { r#type: "text", id: "username", name: "username" }
                label { "Username" }
                br {}
                input { r#type: "password", id: "password", name: "password" }
                label { "Password" }
                br {}
                button { "Login" }
            }
            }
        }
    })
}
