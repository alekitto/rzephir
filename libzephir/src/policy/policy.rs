use crate::compiler::compiled_policy::CompiledPolicy;
use crate::compiler::compiler::Compiler;
use crate::err::Error;
use crate::policy::match_result::MatchResult;
use crate::policy::{PolicyEffect, PolicyVersion};
use serde_json::{Map, Value};

pub trait ToJson {
    /// Return a JSON value representing the policy
    fn to_json(&self) -> Map<String, Value>;

    /// Gets the JSON string value
    fn to_json_string(&self) -> String {
        serde_json::to_string(&self.to_json()).unwrap()
    }

    /// Performs the conversion.
    fn to_value(&self) -> Value {
        Value::from(self.to_json())
    }
}

/// Policy trait.
/// This is the main common interface for all the policy implementations
pub trait Policy: ToJson {
    fn id(&self) -> &String {
        unimplemented!()
    }

    /// Return true if the policy is complete.
    ///
    /// The evaluation result will return "ALLOWED" or "DENIED" if
    /// the policy is complete.
    ///
    /// If the policy is not complete, the evaluation result can be "ABSTAIN".
    fn complete(&self) -> bool {
        false
    }

    /// Get the default version of the policy
    fn default() -> Self;
}

/// Represents a policy that can be matched against
/// action and resource identifiers
pub trait MatchablePolicy: Policy {
    /// Gets the policy effect
    fn get_effect(&self) -> PolicyEffect;

    /// Calculate if this policy is matching
    fn matching<T, S>(&self, action: Option<T>, resource: Option<S>) -> MatchResult
    where
        T: ToString,
        S: ToString;

    /// Gets the action of the policy.
    fn get_actions(&self) -> Vec<String>;

    /// Gets the resources of the policy.
    fn get_resources(&self) -> Vec<String>;
}

/// Partial policy struct
/// Actions and Resources can be optional
#[derive(Clone, Debug)]
pub struct PartialPolicy {
    pub version: PolicyVersion,
    pub effect: PolicyEffect,
    pub actions: Option<Vec<String>>,
    pub resources: Option<Vec<String>>,
}

impl PartialPolicy {
    /// Default partial policy.
    pub fn default() -> PartialPolicy {
        PartialPolicy {
            version: PolicyVersion::Version1,
            effect: PolicyEffect::Allow,
            actions: Option::None,
            resources: Option::None,
        }
    }

    /// Resets the partial policy.
    pub fn reset(&mut self) -> () {
        self.version = PolicyVersion::Version1;
        self.actions = Option::None;
        self.resources = Option::None;
    }
}

impl Into<Value> for PartialPolicy {
    fn into(self) -> Value {
        self.to_value()
    }
}

impl ToJson for PartialPolicy {
    fn to_json(&self) -> Map<String, Value> {
        let mut result = Map::new();
        result.insert(String::from("version"), Value::from(&self.version));
        result.insert(String::from("effect"), Value::from(&self.effect));

        if self.actions.is_some() {
            result.insert(
                String::from("actions"),
                Value::from(self.actions.clone().unwrap()),
            );
        }

        if self.resources.is_some() {
            result.insert(
                String::from("resources"),
                Value::from(self.resources.clone().unwrap()),
            );
        }

        result
    }
}

impl Policy for PartialPolicy {
    fn default() -> PartialPolicy {
        PartialPolicy::default()
    }
}

/// Represents a complete policy which can be matched completely
pub struct CompletePolicy {
    pub id: String,
    pub version: PolicyVersion,
    pub effect: PolicyEffect,
    actions: Vec<String>,
    resources: Vec<String>,

    compiled_policy: CompiledPolicy,
}

impl CompletePolicy {
    /// Get a new policy object
    pub fn new<A, R>(
        id: String,
        version: PolicyVersion,
        effect: PolicyEffect,
        actions: Vec<A>,
        resources: Vec<R>,
    ) -> Result<CompletePolicy, Error>
    where
        A: ToString,
        R: ToString,
    {
        if actions.is_empty() {
            return Err(Error::actions_cannot_be_empty());
        }

        let resources = if resources.is_empty() {
            vec!["*".to_string()]
        } else {
            resources.into_iter().map(|s| s.to_string()).collect()
        };

        let actions: Vec<String> = actions.into_iter().map(|s| s.to_string()).collect();
        let compiled_policy = Compiler::compile(&actions, &resources);

        Ok(CompletePolicy {
            id,
            version,
            effect,
            actions,
            resources,
            compiled_policy,
        })
    }
}

impl Policy for CompletePolicy {
    fn id(&self) -> &String {
        &self.id
    }

    fn complete(&self) -> bool {
        true
    }

    fn default() -> CompletePolicy {
        unimplemented!()
    }
}

impl ToJson for CompletePolicy {
    fn to_json(&self) -> Map<String, Value> {
        let mut result = Map::new();
        result.insert(String::from("id"), Value::from(self.id.clone()));
        result.insert(String::from("version"), Value::from(&self.version));
        result.insert(
            String::from("effect"),
            Value::from(match self.effect {
                PolicyEffect::Allow => "ALLOW",
                _ => "DENY",
            }),
        );
        result.insert(String::from("actions"), Value::from(self.actions.clone()));
        result.insert(
            String::from("resources"),
            Value::from(self.resources.clone()),
        );

        result
    }
}

impl MatchablePolicy for CompletePolicy {
    fn get_effect(&self) -> PolicyEffect {
        self.effect.clone()
    }

    fn matching<T, S>(&self, action: Option<T>, resource: Option<S>) -> MatchResult
    where
        T: ToString,
        S: ToString,
    {
        let mut result = MatchResult::new();
        let compiled = &self.compiled_policy;

        if action.is_some() {
            result.update_action(compiled.match_action(&action.unwrap().to_string()));
            result._update(self);
        }

        if compiled.all_resources {
            result.update_resource(true);
            result._update(self);
        } else if resource.is_some() {
            let m = compiled.match_resource(resource);
            if m.is_some() {
                let is_match = m.unwrap();
                result.update_resource(is_match);
                result._update(self);
            }
        }

        // @todo: Conditions

        result
    }

    fn get_actions(&self) -> Vec<String> {
        self.actions.clone()
    }

    fn get_resources(&self) -> Vec<String> {
        self.resources.clone()
    }
}

#[macro_export]
macro_rules! zephir_policy {
    ( $id:expr, $version:expr, $effect:expr, $actions:expr, $resources:expr ) => {{
        let temp_policy = $crate::policy::policy_new($id.into(), $version, $effect, $actions, $resources);
        temp_policy
    }};
    ( $id:expr, $version:expr, $effect:expr, $actions:expr ) => {{
        $crate::zephir_policy!(
            $id,
            $version,
            $effect,
            $actions,
            std::vec![] as std::vec::Vec<std::string::String>
        )
    }};
}

#[cfg(test)]
mod tests {
    use crate::policy::policy::{MatchablePolicy, Policy, ToJson};
    use crate::policy::{PolicyEffect, PolicyVersion};
    use crate::zephir_policy;

    #[test]
    fn complete_policy_could_be_created() {
        let p = zephir_policy!(
            "TestPolicy",
            PolicyVersion::Version1,
            PolicyEffect::Deny,
            vec!["core:GetVersion", "test:GetResource"]
        )
        .unwrap();

        assert_eq!(p.complete(), true);
        assert_eq!(p.resources, vec!["*"]);
        assert_eq!(p.actions, vec!["core:GetVersion", "test:GetResource"]);
        assert_eq!(p.to_json_string(), "{\"actions\":[\"core:GetVersion\",\"test:GetResource\"],\"effect\":\"DENY\",\"id\":\"TestPolicy\",\"resources\":[\"*\"],\"version\":1}");
    }

    #[test]
    fn policy_creation_should_return_err_if_actions_are_empty() {
        let result = zephir_policy!(
            "TestPolicy",
            PolicyVersion::Version1,
            PolicyEffect::Allow,
            vec![] as Vec<String>
        )
        .err()
        .unwrap();

        assert_eq!(result.to_string(), "Actions set cannot be empty");
    }

    #[test]
    fn policy_matching_should_work_if_policy_contains_all_actions() {
        let policy =
            zephir_policy!("TestPolicy", PolicyVersion::Version1, PolicyEffect::Allow, vec!["*"]).unwrap();
        let result = policy.matching(Some("TestAction"), Some("urn::resource:test"));

        assert_eq!(result.is_match(), true);
        assert_eq!(result.is_full(), true);

        let result = policy.matching(Some("FooAction"), Some("urn::resource:test"));

        assert_eq!(result.is_match(), true);
        assert_eq!(result.is_full(), true);
    }

    #[test]
    fn policy_matching_should_work_with_actions_star_glob() {
        let policy = zephir_policy!(
            "TestPolicy",
            PolicyVersion::Version1,
            PolicyEffect::Allow,
            vec!["*Action"]
        )
        .unwrap();
        let result = policy.matching(Some("FooAction"), Some("urn::resource:test"));

        assert_eq!(result.is_match(), true);
        assert_eq!(result.is_full(), true);

        let result = policy.matching(Some("FooBar"), Some("urn::resource:test"));

        assert_eq!(result.is_match(), false);
        assert_eq!(result.is_full(), true);
    }

    #[test]
    fn policy_matching_should_work_with_actions_question_mark_glob() {
        let policy =
            zephir_policy!("TestPolicy", PolicyVersion::Version1, PolicyEffect::Allow, vec!["Foo?ar"]).unwrap();

        let result = policy.matching(Some("FooAction"), Some("urn::resource:test"));
        assert_eq!(result.is_match(), false);
        assert_eq!(result.is_full(), true);
        let result = policy.matching(Some("FooBar"), Some("urn::resource:test"));
        assert_eq!(result.is_match(), true);
        assert_eq!(result.is_full(), true);
        let result = policy.matching(Some("FooDar"), Some("urn::resource:test"));
        assert_eq!(result.is_match(), true);
        assert_eq!(result.is_full(), true);
        let result = policy.matching(Some("FooFar"), Some("urn::resource:test"));
        assert_eq!(result.is_match(), true);
        assert_eq!(result.is_full(), true);
    }

    #[test]
    fn matching_should_return_a_partial_policy() {
        let policy =
            zephir_policy!("TestPolicy", PolicyVersion::Version1, PolicyEffect::Allow, vec!["*"]).unwrap();
        let m = policy.matching(Some("TestAction"), None as Option<String>);
        assert_eq!(m.is_full(), true);

        let policy = zephir_policy!(
            "TestPolicy",
            PolicyVersion::Version1,
            PolicyEffect::Allow,
            vec!["TestAction"],
            vec!["urn:resource:test"]
        )
        .unwrap();
        let m = policy.matching(Some("NoAction"), None as Option<String>);
        assert_eq!(m.is_full(), true);

        let m = policy.matching(Some("TestAction"), None as Option<String>);
        assert_eq!(m.is_full(), false);

        let partial = m.get_partial();
        assert_eq!(partial.complete(), false);
        assert_eq!(partial.effect, PolicyEffect::Allow);
        assert_eq!(partial.version, PolicyVersion::Version1);

        let resources = partial.resources.as_ref();
        assert_eq!(resources.is_some(), true);
        assert_eq!(*resources.unwrap(), vec!["urn:resource:test".to_string()]);
        assert_eq!(
            partial.to_json_string(),
            "{\"effect\":\"ALLOW\",\"resources\":[\"urn:resource:test\"],\"version\":1}"
        );

        let m = policy.matching(None as Option<String>, Some("urn:resource:test"));
        let partial = m.get_partial();
        assert_eq!(
            partial.to_json_string(),
            "{\"actions\":[\"TestAction\"],\"effect\":\"ALLOW\",\"version\":1}"
        );
    }
}
