use wasm_bindgen_futures::JsFuture;
use web_sys::*;
use yew::prelude::*;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};


#[function_component(Producer)]
fn producer() -> Html {
    let buttonsNames = vec!["Explorar","Modo trabajo","Ray"];
    let names = vec!["Sam","Bob","Ray"];
    html!{
        <div class="master">
            <div class="header">
                <div>
                    <h3>{"Ivan"}</h3>
                </div>
                <div class="header-buttons-container">
                    {
                        buttonsNames.into_iter().map(|name|{
                            html!{<button key={name} class="header-button">{ name }</button>}
                        }).collect::<Html>()
                    }
                </div>
            </div>
            <div>
                <button>{"Matches"}</button>
                <button>{"Mensajes"}</button>
            </div>
            <div>
                <div>
                    
                    {
                        names.into_iter().map(|name|{
                            html!{<div key={name}>{ name }</div>}
                        }).collect::<Html>()
                    }
                    
                </div>
            </div>
        </div>
    }
}

#[function_component(Consumer)]
fn consumer() -> Html {
    html!{
        <div class="detail">
            <h3>{"You are also here"}</h3>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html!{
        <div class="container">
            <Producer/>
            <Consumer/>
        </div>
    }
}

fn main() {
    println!("Hello, world!");

    yew::start_app::<App>();
}
