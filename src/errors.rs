use std::fmt::Display;

error_chain! {
    errors {
        UnlinkedContextReference(reference: String) {
            description("Can only call resolve on linked references")
            display("Can only call resolve on linked references: {:?}", reference)
        }
        InvalidMatchPatternIndex(index: usize) {
            description("No match pattern at index")
            display("No match pattern at index {}", index)
        }
        InvalidAtomIndex(index: usize) {
            description("No atom at index")
            display("No atom at index {}", index)
        }
        RestoreEmptyScopeStack {
            description("The scope stack could not be restored as the cleared scope stack is empty")
            display("The scope stack could not be restored as the cleared scope stack is empty")
        }
    }
}
