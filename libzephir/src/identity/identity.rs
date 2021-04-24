use crate::identity::subject::Subject;
use crate::policy::policy::{CompletePolicy, ToJson};
use crate::identity::role::Role;
use serde_json::{Value, Map};
use crate::policy::policy_set::{PolicySet, PolicySetTrait, PolicySetHelper};

pub struct Identity {
    pub(crate) id: String,
    pub(crate) inline_policy: Option<CompletePolicy>,
    pub(crate) linked_policies: PolicySet<CompletePolicy>,
}

impl Identity {
    pub fn new(id: String, policy: Option<CompletePolicy>) -> Self {
        Identity {
            id,
            inline_policy: policy,
            linked_policies: PolicySet::new(),
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn clear_inline_policy(mut self) -> Self {
        self.inline_policy = Option::None;
        self
    }

    pub fn set_inline_policy(mut self, policy: CompletePolicy) -> Self {
        self.inline_policy = Option::Some(policy);
        self
    }
}

pub trait ToIdentityId {
    fn to_identity_id(&self) -> &String;
}

impl ToIdentityId for Identity {
    fn to_identity_id(&self) -> &String {
        &self.id
    }
}

impl ToIdentityId for String {
    fn to_identity_id(&self) -> &String {
        self
    }
}

impl Subject for Identity {
    fn get_inline_policy(&self) -> Option<&CompletePolicy> {
        self.inline_policy.as_ref()
    }
}

impl ToJson for Identity {
    fn to_json(&self) -> Map<String, Value> {
        let linked_policies = &self.linked_policies;
        let mut map = Map::new();
        map.insert(String::from("id"), Value::from(self.id.clone()));
        map.insert(String::from("inline_policy"),
            if self.inline_policy.is_none() {
                Value::Null
            } else {
                Value::from(self.inline_policy.as_ref().unwrap().to_json())
            }
        );
        map.insert(
            String::from("linked_policies"),
            Value::from(linked_policies.into_iter().map(|ref p| p.id.clone()).collect::<Vec<String>>())
        );

        map
    }
}

impl Into<Value> for Identity {
    fn into(self) -> Value {
        Value::Object(self.to_json())
    }
}

impl PolicySetTrait<CompletePolicy> for Identity {
    fn add_policy(mut self, policy: CompletePolicy) -> Self {
        self.linked_policies = PolicySetHelper::link_policy(self.linked_policies, policy);
        self
    }

    fn remove_policy<S: ToString>(mut self, id: S) -> Self {
        self.linked_policies = PolicySetHelper::unlink_policy(self.linked_policies, id);
        self
    }
}

impl Role for Identity {
    fn linked_policies(&self) -> &PolicySet<CompletePolicy> {
        &self.linked_policies
    }
}
