use godot::classes::IControl;
use godot::classes::Control;
use godot::{prelude::*};
use opencompose_rs::view_builder;

use crate::gdrust_trinkets::gdui::ast_parser::ASTParser;

#[derive(GodotClass, Debug)]
#[class(base=Control)]
pub struct TestControl {
    pub base: Base<Control>
}

impl TestControl {}

#[godot_api]
impl IControl for TestControl {
    fn init(base: Base<Control>) -> Self {
        Self {
            base
        }
    }

    fn enter_tree(&mut self) {
        let sample_tree = view_builder!(
            Text(text: "Text")
        );
        let parser = ASTParser {
            ast: sample_tree
        };
        let control = parser.parse();
        self.base_mut().add_child(&control);
    }
}
