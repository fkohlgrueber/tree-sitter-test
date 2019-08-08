
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::ConsoleService;

use tree_sitter::{Parser, Language, Node};

extern "C" { fn tree_sitter_rust() -> Language; }

pub struct Model {
    console: ConsoleService,
    input: String,
    text: String,
    parser: Parser,
}

pub enum Msg {
    UpdateText(String)
}


impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {

        let mut parser = Parser::new();

        let language = unsafe { tree_sitter_rust() };
        parser.set_language(language).unwrap();

        let mut model = Model {
            console: ConsoleService::new(),
            input: "fn test(){\n    let x = 1 + 2;\n}".to_string(),
            text: "".to_string(),
            parser: parser,
        };

        model.update(Msg::UpdateText(model.input.clone()));

        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateText(s) => {
                self.console.log("Updating...");
                self.input = s.clone();
                self.text = match self.parser.parse(s, None) {
                    Some(tree) => {
                        print_node(&tree.root_node(), 0)
                    },
                    None => {
                        "Error".to_string()
                    }
                };
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div style="padding: 10px; height: 100%", >
                <textarea oninput=|e| Msg::UpdateText(e.value), > {{ self.input.clone() }} < /textarea>
                <textarea readonly="true", > {{ self.text.clone() }} < /textarea>
            </div>
        }
    }
}

fn print_node(node: &Node, indent: usize) -> String {
    format!("{}{}\"{}\"{}{}{}", 
        " ".repeat(indent), 
        if node.is_named() { "" } else { "(" },
        node.kind(),
        if node.is_named() { "" } else { ")" },
        if node.children().count() > 0 { "\n" } else { "" },
        node.children().map(|x| print_node(&x, indent+4)).collect::<Vec<_>>().join("\n")
    )
}