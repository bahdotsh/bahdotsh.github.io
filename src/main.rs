use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
use self::pages::*;

#[derive(yew_router::Switch, Clone)]
pub enum MainRoute {
    #[to = "/"]
    Home,
}

pub struct Main;

impl Component for Main {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
		<>
                <div class="main">
                    <Router<MainRoute, ()>
                        render = Router::render(|switch: MainRoute| {
                            match switch {
                                MainRoute::Home => html!{ <Home/> },
                            }
                        })
                    />
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Main>();
}
