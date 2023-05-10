
use yew::prelude::*;

pub struct Home {}


impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
             <div class="container">
                <div class="child1">
                    <div class="header"  > 
                        <h1>{"Hello."}</h1>
                    </div>
                    <div class="header" > 
                        <h3>{"I'm Colin Finkbeiner."}</h3>
                    </div>
                    <div class="content">
                        <p>{"I am an incoming Ph.D. student in Computer Science at the "}
                        <em>{"University of Connecticut"}</em> 
                        {" advised by Prof. Ghada Almashaqbeh. My research interest lies at the intersection of cryptography, privacy systems, and mechanism design specifically in the lens of blockchains and other distributed systems."}</p>
                    </div>
                    <div class="content">
                        <p>{"Previously, I was President of "}
                        <span class="highlight-bottom"><a href="https://www.michiganblockchain.org/#contactus">{"Blockchain at Michigan"}</a></span>
                        {" and a developer for "} 
                        <span class="highlight-bottom"><a href="https://twitter.com/dualityxyz?lang=en">{"Duality.xyz"}</a></span>
                        {" where I worked on market design.  Prior studies were at the "}
                        <em>{"University of Michigan"}</em>
                        {" where I studied Computer Science and minored in Mathematics."}</p>
                    </div>
                    <hr/>
                    <div class="icons">
                        <a href="https://ipfs.io/ipfs/QmQDZLanGNF3SvGet2fj4GNL7otmsTijn57XfD7T1aCgaA"><i class="icon ai ai-cv"></i> </a>
                        <a href="https://github.com/finkbeca"> <i class="icon fab fa-github"></i></a>
                        <a href="mailto:finkbeca@umich.edu"><i class="icon fa-solid fa-envelope"></i> </a>
                        <a href="https://twitter.com/finkbeca">  <i class="icon fab fa-twitter"></i></a>
                       
                        
                    </div>
                </div>
                <div class="child2">
                    
                </div>
             </div>
             
            </>
            }
             
    }
}
