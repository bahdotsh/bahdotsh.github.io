use yew::prelude::*;

pub struct Home;

impl Component for Home {
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
 		<div>
                        <h1>{ "Namaskaram!" } </h1>
                        <p style="font-weight: 500; font-size: 2.2em"> { "My name is Gokul Santhosh." } </p>
                        <p> { " I'm a storyteller by heart and I love solving complex real-world problems,
                       	 derive user-friendly solutions and share them as beautiful stories. I have worked on everything I
                         have found interesting, from machine learning and AI to cryptography, blockchain and distributed systems.
                         And from all those experiences what I believe is, all that matters is how each of the solutions I arrives at affects the life of the people." } </p>
                        <p> { "  I am currently based out of Kerala, India. Most of my work is open source and is publicly available on "}
                        <a href={ "https://github.com/bahdotsh"} target={"_blank"}>{"GitHub"}</a> {". "}
                          { "I am currently building (hopefully) cool stuff with rust and C++,
                          infact this website is built on  " } <a href={ "https://yew.rs/"} target={"_blank"}>{"yew"}</a> {"!"} </p>
                        <p> { " Sometimes I have my rants on the world and life, " }
                        <a href={ "https://medium.com/@gokulsanthoshofficial"} target={"_blank"}>{"here"}</a> {","} { " and
                          on tech, " } <a href={ "https://blog.gokuls.in"} target={"_blank"}>{"here"}</a> {"."}</p>
                        <p> { " Connect me on " } <a href={ "https://twitter.com/bahdotshx"} target={"_blank"}>{"twitter"}</a> { " or send me a hi on "}
                        <a href={ "mailto:gokulsanthoshofficial@gmail.com"} target={"_blank"}>{"mail"}</a> {"."}</p>
               </div>
        }
    }
}
