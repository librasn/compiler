use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

use crate::intermediate::ModuleHeader;

#[derive(Debug, Clone, PartialEq)]
pub struct MacroDefinition<'a> {
    pub name: Cow<'a, str>,
    pub module_header: Option<Rc<RefCell<ModuleHeader<'a>>>>,
}

impl<'a> From<crate::lexer::macros::MacroDefinition<'a>> for MacroDefinition<'a> {
    fn from(macro_def: crate::lexer::macros::MacroDefinition<'a>) -> Self {
        MacroDefinition {
            name: Cow::Borrowed(macro_def.name),
            module_header: None,
        }
    }
}
