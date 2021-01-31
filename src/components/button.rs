use log::*;

use std::rc::Rc;
use yew::prelude::*;

pub trait NeqAssign {
    fn neq_assign(&mut self, new: Self) -> ShouldRender;
}

impl<T: PartialEq> NeqAssign for T {
    fn neq_assign(&mut self, new: T) -> ShouldRender {
        if self != &new {
            *self = new;
            true
        } else {
            false
        }
    }
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct Props {
    pub children: Children,
    pub onclick: Callback<MouseEvent>,
}

pub enum Msg {}

pub struct Button {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        info!("{:?}", props);
        Button { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            _ => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
          <button onclick=&self.props.onclick>{self.props.children.clone()}</button>
        }
    }
}
