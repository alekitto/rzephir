use crate::identity::identity::{Identity, ToIdentityId};
use crate::policy::policy::{CompletePolicy, ToJson};
use crate::policy::policy_set::{PolicySet, PolicySetTrait, PolicySetHelper};
use std::cmp::Ordering;
use crate::identity::subject::Subject;
use crate::identity::role::Role;
use serde_json::{Value, Map};
use std::slice::Iter;

pub struct IdentitySet {
    identities: Vec<Identity>,
}

impl<'a> IntoIterator for &'a IdentitySet {
    type Item = &'a Identity;
    type IntoIter = Iter<'a, Identity>;

    fn into_iter(self) -> Self::IntoIter {
        self.identities.iter()
    }
}

impl IdentitySet {
    fn new() -> Self {
        IdentitySet {
            identities: vec![]
        }
    }

    fn insert_if_missing(identities: &mut Vec<Identity>, identity: Identity) {
        match identities.iter_mut().find(|ref i| i.id == identity.id) {
            Some(_) => {
                return;
            }
            None => {
                identities.push(identity)
            }
        }
    }

    pub fn add_identity(mut self, identity: Identity) -> Self {
        Self::insert_if_missing(self.identities.as_mut(), identity);

        self
    }

    pub fn remove_identity<T: ToIdentityId>(mut self, identity: T) -> Self {
        let identity_id = identity.to_identity_id();
        self.identities = self.identities
            .into_iter()
            .filter(|i| i.id.cmp(identity_id) != Ordering::Equal)
            .collect()
        ;

        self
    }
}

pub struct Group {
    pub(crate) name: String,
    pub(crate) identities: IdentitySet,

    pub(crate) inline_policy: Option<CompletePolicy>,
    pub(crate) linked_policies: PolicySet<CompletePolicy>,
}

impl Group {
    pub fn new(name: String, policy: Option<CompletePolicy>) -> Self {
        Group {
            name,
            identities: IdentitySet::new(),
            inline_policy: policy,
            linked_policies: PolicySet::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn clear_inline_policy(mut self) -> Self {
        self.inline_policy = Option::None;
        self
    }

    pub fn set_inline_policy(mut self, policy: CompletePolicy) -> Self {
        self.inline_policy = Option::Some(policy);
        self
    }

    pub fn add_identity(mut self, identity: Identity) -> Self {
        self.identities = self.identities.add_identity(identity);
        self
    }

    pub fn remove_identity<T: ToIdentityId>(mut self, identity: T) -> Self {
        self.identities = self.identities.remove_identity(identity);
        self
    }
}

impl PolicySetTrait<CompletePolicy> for Group {
    fn add_policy(mut self, policy: CompletePolicy) -> Self {
        self.linked_policies = PolicySetHelper::link_policy(self.linked_policies, policy);
        self
    }

    fn remove_policy<S: ToString>(mut self, id: S) -> Self {
        self.linked_policies = PolicySetHelper::unlink_policy(self.linked_policies, id);
        self
    }
}

impl Into<Value> for Group {
    fn into(self) -> Value {
        Value::Object(self.to_json())
    }
}

impl ToJson for Group {
    fn to_json(&self) -> Map<String, Value> {
        let linked_policies = &self.linked_policies;
        let identities = &self.identities;
        let mut map = Map::new();
        map.insert(String::from("id"), Value::from(self.name.clone()));
        map.insert(
            String::from("inline_policy"),
            if self.inline_policy.is_none() {
                Value::Null
            } else {
                Value::from(self.inline_policy.as_ref().unwrap().to_json())
            }
        );
        map.insert(
            String::from("identities"),
            Value::from(identities.into_iter().map(|ref i| i.id.clone()).collect::<Vec<String>>())
        );
        map.insert(
            String::from("linked_policies"),
            Value::from(linked_policies.into_iter().map(|ref p| p.id.clone()).collect::<Vec<String>>())
        );

        map
    }
}

impl Subject for Group {
    fn get_inline_policy(&self) -> Option<&CompletePolicy> {
        self.inline_policy.as_ref()
    }
}

impl Role for Group {
    fn linked_policies(&self) -> &PolicySet<CompletePolicy> {
        &self.linked_policies
    }
}
