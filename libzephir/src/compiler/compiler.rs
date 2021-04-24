use crate::compiler::compiled_policy::CompiledPolicy;
use crate::utils::glob_to_regex;

pub struct Compiler {}

impl Compiler {
    // unsafe fn get_from_cache(&self, id: &String) -> Option<&CacheEntry> {
    //     let const_ptr = &self.cache as *const Cache;
    //     let mut_ptr = const_ptr as *mut Cache;
    //
    //     (&mut *mut_ptr).touch(|entry: &CacheEntry| entry.id.cmp(id) == Ordering::Equal);
    //     const_ptr.as_ref().unwrap().front()
    // }
    //
    // unsafe fn store_in_cache(&self, id: &String, compiled: CompiledPolicy) -> Option<&CacheEntry> {
    //     let entry = CacheEntry {
    //         id: id.clone(),
    //         policy: compiled,
    //     };
    //
    //     let const_ptr = &self.cache as *const Cache;
    //     let mut_ptr = const_ptr as *mut Cache;
    //     (&mut *mut_ptr).insert(entry);
    //
    //     const_ptr.as_ref().unwrap().front()
    // }

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
