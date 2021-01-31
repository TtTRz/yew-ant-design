use log::*;
use yew::prelude::*;

use super::*;

pub struct State {
    title: String,
    number: isize,
}

impl Default for State {
    fn default() -> Self {
        State {
            title: String::from("init"),
            number: 0,
        }
    }
}

pub struct App {
    link: ComponentLink<Self>,
    state: State,
}

pub enum Msg {
    Click,
    HandleChangeTitle,
    AddNum,
}

impl Component for App {
    type Properties = ();
    type Message = Msg;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            state: State::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                info!("click");
                false
            }
            Msg::HandleChangeTitle => {
                info!("handleChangeTitle");
                self.state.title = String::from("new title");
                true
            }
            Msg::AddNum => {
                info!("add number");
                self.state.number += 1;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");
        let onclick = self.link.callback(|_| Msg::Click);
        let handle_change_title = self.link.callback(|_| Msg::HandleChangeTitle);
        let handle_add = self.link.callback(|_| Msg::AddNum);
        html! {
          <>
            <Header>
              <Menu>
                <MenuItem>
                  {"hello"}
                </MenuItem>
              </Menu>
            </Header>
            {self.state.number}
            <button onclick=onclick>{"click me"}</button>
            <Button onclick=handle_change_title>{"hello"}</Button>
            <Button onclick=handle_add>{"+1"}</Button>
          </>
        }
        // html! {
        //     <div class="todomvc-wrapper">
        //         <section class="todoapp">
        //             <header class="header">
        //                 <h1>{ "todos" }</h1>
        //                 { self.view_input() }
        //             </header>
        //             <section class="main">
        //                 <input class="toggle-all" type="checkbox" checked=self.state.is_all_completed() onclick=self.link.callback(|_| Msg::ToggleAll) />
        //                 <ul class="todo-list">
        //                     { for self.state.entries.iter().filter(|e| self.state.filter.fit(e))
        //                         .enumerate()
        //                         .map(|val| self.view_entry(val)) }
        //                 </ul>
        //             </section>
        //             <footer class="footer">
        //                 <span class="todo-count">
        //                     <strong>{ self.state.total() }</strong>
        //                     { " item(s) left" }
        //                 </span>
        //                 <ul class="filters">
        //                     { for Filter::iter().map(|flt| self.view_filter(flt)) }
        //                 </ul>
        //                 <button class="clear-completed" onclick=self.link.callback(|_| Msg::ClearCompleted)>
        //                     { format!("Clear completed ({})", self.state.total_completed()) }
        //                 </button>
        //             </footer>
        //         </section>
        //         <footer class="info">
        //             <p>{ "Double-click to edit a todo" }</p>
        //             <p>{ "Written by " }<a href="https://github.com/DenisKolodin/" target="_blank">{ "Denis Kolodin" }</a></p>
        //             <p>{ "Part of " }<a href="http://todomvc.com/" target="_blank">{ "TodoMVC" }</a></p>
        //         </footer>
        //     </div>
        // }
    }
}
