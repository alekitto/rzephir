use crate::policy::policy::{MatchablePolicy, PartialPolicy};
use crate::policy::PolicyVersion;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ResultType {
    Partial,
    Full,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ResultOutcome {
    Match = 0,
    NotMatch = 1,
}

#[derive(Clone)]
pub struct MatchResult {
    pub result_type: ResultType,
    pub outcome: ResultOutcome,

    partial: PartialPolicy,

    action_matches: Option<bool>,
    resource_matches: Option<bool>,
}

impl MatchResult {
    pub fn new() -> MatchResult {
        MatchResult {
            result_type: ResultType::Partial,
            outcome: ResultOutcome::NotMatch,
            partial: PartialPolicy::default(),
            action_matches: None,
            resource_matches: None,
        }
    }

    pub fn update_action(&mut self, result: bool) -> () {
        self.action_matches = Option::Some(result);
    }

    pub fn update_resource(&mut self, result: bool) -> () {
        self.resource_matches = Option::Some(result);
    }

    pub fn get_partial(&self) -> &PartialPolicy {
        &self.partial
    }

    pub fn is_match(&self) -> bool {
        self.outcome == ResultOutcome::Match
    }

    pub fn is_full(&self) -> bool {
        self.result_type == ResultType::Full
    }

    pub fn _update(&mut self, policy: &impl MatchablePolicy) -> () {
        self.partial.reset();
        self.partial.effect = policy.get_effect();

        if (self.action_matches.is_some() && !self.action_matches.unwrap())
            || (self.resource_matches.is_some() && !self.resource_matches.unwrap())
        {
            self.result_type = ResultType::Full;
            self.outcome = ResultOutcome::NotMatch;

            return;
        }

        if self.action_matches.unwrap_or(false) || self.resource_matches.unwrap_or(false) {
            self.outcome = ResultOutcome::Match;
        }

        if self.action_matches.is_some() && self.resource_matches.is_some() {
            self.result_type = ResultType::Full;
        } else {
            self.partial = PartialPolicy {
                version: PolicyVersion::Version1,
                effect: self.partial.effect.clone(),
                actions: if self.action_matches.is_some() {
                    None
                } else {
                    Option::Some(policy.get_actions())
                },
                resources: if self.resource_matches.is_some() {
                    None
                } else {
                    Option::Some(policy.get_resources())
                },
            }
        }
    }
}
