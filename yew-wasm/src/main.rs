use yew::prelude::*;
use web_sys::{Event, HtmlInputElement};
use mylib::{Rectangle};

enum Msg {
    AddOne,
    UpdateScript(String),
}

struct Model {
    value: i64,
    detail: String,
    script: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            detail: "".to_owned(),
            script: "".to_owned(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::UpdateScript(s) => {
                self.script = s.clone();
                self.detail = s;
                if self.detail.len() == 0{
                    self.detail = format!("Object is {} ", Rectangle::new(0.0, 2.0, 0.0, 2.0).j().unwrap());
                } else {
                    self.detail = "".to_owned();
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();

        let on_script_change = link.batch_callback(|e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            input.map(|input| Msg::UpdateScript(input.value()))
        }
        );
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
                <input
                type="text"
                name="todo"
                value={self.script.clone()}
                placeholder="Create a new todo"
                onchange={on_script_change}
                />
                <p>{ self.detail.clone() }</p>
                // <button onclick={link.callback(|_| Msg::UpdateDetail)}>{"Detail"}</button>

            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}