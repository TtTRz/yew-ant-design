use log::*;

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
    pub title: String,
}

pub struct HeaderComp {
    link: ComponentLink<Self>,
    props: Props,
}
pub enum Msg {}

impl Component for HeaderComp {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        info!("{:?}", props);
        HeaderComp { props, link }
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
          <div>{&self.props.title}</div>
        }
    }
}
