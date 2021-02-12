use crate::compiler::compiled_policy::CompiledPolicy;
use crate::utils::glob_to_regex;

pub struct Compiler {}

impl Compiler {
    pub fn compile(actions: &Vec<String>, resources: &Vec<String>) -> CompiledPolicy {
        let compiled_actions = actions
            .into_iter()
            .map(|a| glob_to_regex::from_string(a.to_string()))
            .collect();

        let any_resource = resources.into_iter().any(|v| v == r"*");
        let compiled_resources = if any_resource {
            vec![]
        } else {
            resources
                .into_iter()
                .map(|a| glob_to_regex::from_string(a.to_string()))
                .collect()
        };

        CompiledPolicy::new(compiled_actions, compiled_resources)
    }
}
