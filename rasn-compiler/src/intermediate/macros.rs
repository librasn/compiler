use std::cell::RefCell;
use std::rc::Rc;

use crate::intermediate::ModuleHeader;
use crate::lexer::macros::MacroDefinition;

#[derive(Debug, Clone, PartialEq)]
pub struct ToplevelMacroDefinition {
    pub name: String,
    pub index: Option<(Rc<RefCell<ModuleHeader>>, usize)>,
}

impl From<MacroDefinition<'_>> for ToplevelMacroDefinition {
    fn from(macro_def: MacroDefinition<'_>) -> Self {
        ToplevelMacroDefinition {
            name: macro_def.name.to_string(),
            index: None,
        }
    }
}
