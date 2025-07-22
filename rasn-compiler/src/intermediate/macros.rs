use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

use crate::intermediate::ModuleHeader;
use crate::lexer::macros::MacroDefinition;

#[derive(Debug, Clone, PartialEq)]
pub struct ToplevelMacroDefinition<'a> {
    pub name: Cow<'a, str>,
    pub module_header: Option<Rc<RefCell<ModuleHeader>>>,
}

impl<'a> From<MacroDefinition<'a>> for ToplevelMacroDefinition<'a> {
    fn from(macro_def: MacroDefinition<'a>) -> Self {
        ToplevelMacroDefinition {
            name: Cow::Borrowed(macro_def.name),
            module_header: None,
        }
    }
}
