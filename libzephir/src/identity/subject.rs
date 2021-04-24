use crate::identity::role::Role;
use crate::policy::policy::{CompletePolicy, ToJson};

pub trait Subject: Role + ToJson {
    /// Returns the inline policy associate with the subject.
    fn get_inline_policy(&self) -> Option<&CompletePolicy>;
}