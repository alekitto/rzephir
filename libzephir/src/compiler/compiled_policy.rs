use regex::Regex;

#[derive(Clone)]
pub struct CompiledPolicy {
    actions: Vec<Regex>,
    resources: Vec<Regex>,

    pub all_resources: bool,
}

impl CompiledPolicy {
    /// Creates a new compiled policy
    pub fn new(actions: Vec<Regex>, resources: Vec<Regex>) -> CompiledPolicy {
        let all_resources = resources.is_empty();

        CompiledPolicy {
            actions,
            resources,
            all_resources,
        }
    }

    pub fn match_action<T: ToString>(&self, action: &T) -> bool {
        for regex in &self.actions {
            if regex.is_match(action.to_string().as_str()) {
                return true;
            }
        }

        false
    }

    pub fn match_resource<T: ToString>(&self, resource: Option<T>) -> Option<bool> {
        if self.all_resources {
            return Option::Some(true);
        }

        if resource.is_none() {
            return Option::None;
        }

        let value = resource.unwrap();
        let string = value.to_string();
        let res = string.as_str();

        for regex in &self.resources {
            if regex.is_match(res) {
                return Option::Some(true);
            }
        }

        Option::Some(false)
    }
}
