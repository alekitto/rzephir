use crate::policy::allowed_result::{AllowedOutcome, AllowedResult};
use crate::policy::policy::{CompletePolicy, MatchablePolicy};
use crate::policy::policy_set::PolicySet;
use crate::policy::PolicyEffect;
use serde_json::Value;

fn allowed(
    policies: Vec<&CompletePolicy>,
    action: Option<String>,
    resource: Option<String>,
) -> AllowedResult {
    let mut outcome: AllowedOutcome = AllowedOutcome::Abstain;
    let mut partials = vec![];

    for p in policies {
        let result = p.matching(action.as_ref(), resource.as_ref());
        if !result.is_match() {
            continue;
        }

        if result.is_full() {
            if p.effect == PolicyEffect::Deny {
                return AllowedResult::new(AllowedOutcome::Denied, vec![]);
            }

            outcome = AllowedOutcome::Allowed;
            continue;
        }

        partials.push(result.get_partial().clone());
    }

    AllowedResult::new(outcome, partials)
}

pub trait Role: Into<Value> {
    fn linked_policies(&self) -> &PolicySet<CompletePolicy>;

    fn allowed(&self, action: Option<String>, resource: Option<String>) -> AllowedResult {
        let mut policies = vec![];
        let linked_policies = self.linked_policies();
        for policy in linked_policies {
            policies.push(policy);
        }

        allowed(policies, action, resource)
    }

    fn into(self) -> Value {
        Value::Null
    }
}

#[cfg(test)]
mod tests {
    use crate::identity::role::allowed;
    use crate::policy::allowed_result::AllowedOutcome;
    use crate::policy::policy::{PartialPolicy, ToJson};
    use crate::policy::{PolicyEffect, PolicyVersion};
    use crate::zephir_policy;
    use serde_json::{Map, Value};

    #[test]
    fn allowed_should_return_denied_on_no_policy() {
        let res = allowed(vec![], Option::None, Option::None);
        assert_eq!(res.outcome(), AllowedOutcome::Denied);
    }

    #[test]
    fn allowed_should_check_matching_on_all_passed_policies() {
        let res = allowed(
            vec![
                &zephir_policy!(
                    String::from("p1"),
                    PolicyVersion::Version1,
                    PolicyEffect::Allow,
                    vec!["get_first"]
                )
                .unwrap(),
                &zephir_policy!(
                    String::from("p2"),
                    PolicyVersion::Version1,
                    PolicyEffect::Allow,
                    vec!["get_second"]
                )
                .unwrap(),
            ],
            Option::Some(String::from("get_first")),
            Option::None,
        );

        assert_eq!(res.outcome(), AllowedOutcome::Allowed);
    }

    #[test]
    fn allowed_should_check_matching_with_resources() {
        let res = allowed(
            vec![
                &zephir_policy!(
                    String::from("p1"),
                    PolicyVersion::Version1,
                    PolicyEffect::Allow,
                    vec!["get_first"],
                    vec!["resource_one"]
                )
                .unwrap(),
                &zephir_policy!(
                    String::from("p2"),
                    PolicyVersion::Version1,
                    PolicyEffect::Allow,
                    vec!["get_second"],
                    vec!["resource_one"]
                )
                .unwrap(),
            ],
            Option::Some(String::from("get_first")),
            Option::None,
        );

        assert_eq!(res.outcome(), AllowedOutcome::Abstain);

        let mut partial = PartialPolicy::default();
        partial.effect = PolicyEffect::Allow;
        partial.resources = Option::Some(vec![String::from("resource_one")]);

        let mut json = Map::new();
        json.insert(String::from("outcome"), Value::from("ABSTAIN"));
        json.insert(String::from("partials"), Value::from(vec![partial.clone()]));
        assert_eq!(res.to_json(), json);
    }

    #[test]
    fn should_return_full_deny() {
        let res = allowed(
            vec![
                &zephir_policy!(
                    String::from("p1"),
                    PolicyVersion::Version1,
                    PolicyEffect::Deny,
                    vec!["get_first"],
                    vec!["resource_one"]
                )
                .unwrap(),
                &zephir_policy!(
                    String::from("p2"),
                    PolicyVersion::Version1,
                    PolicyEffect::Allow,
                    vec!["get_second"],
                    vec!["resource_one"]
                )
                .unwrap(),
            ],
            Option::Some(String::from("get_first")),
            Option::Some(String::from("resource_onw")),
        );

        assert_eq!(res.outcome(), AllowedOutcome::Denied);

        let mut json = Map::new();
        json.insert(String::from("outcome"), Value::from("DENIED"));
        json.insert(String::from("partials"), Value::from(Vec::<PartialPolicy>::new()));
        assert_eq!(res.to_json(), json);
    }
}
