use yew::prelude::*;

enum Msg {
    AddOne,
}

// struct Model {
//     // `ComponentLink` is like a reference to a component.
//     // It can be used to send messages to the component
//     link: ComponentLink<Self>,
//     value: i64,
// }

// impl Component for Model {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
//         Self { link, value: 0 }
//     }

//     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//         match msg {
//             Msg::AddOne => {
//                 self.value += 1;
//                 // the value has changed so we need to
//                 // re-render for it to appear on the page
//                 true
//             }
//         }
//     }

//     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
//         // Should only return "true" if new properties are different to
//         // previously received properties.
//         // This component has no properties so we will always return "false".
//         false
//     }

//     fn view(&self) -> Html {
//         html! {
//             <div>
//                 <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
//                 <p>{ self.value }</p>
//             </div>
//         }
//     }
// }

struct App {
    link: ComponentLink<Self>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="main">
                <h1>{ "Help Me" }</h1>
                <button > { "My Clickable" } </button>
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

fn main() {
    yew::start_app::<App>();
}
