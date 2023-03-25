use yew::prelude::*;
use yew::ServerRenderer;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        println!("doing something");
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    println!("server>?");

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

pub async fn runapp() -> String {
    yew::ServerRenderer::<App>::new().render().await
}

// pub async fn runapp() -> String {
//     yew::ServerRenderer::<App>::new().render().await
// }

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
