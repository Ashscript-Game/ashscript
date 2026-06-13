use std::hash::Hash;

use hashbrown::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use crate::resource::{self, Resource};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Storage {
    pub resources: HashMap<Resource, u32>,
    #[serde(skip)]
    pub future_resources: HashMap<Resource, u32>,
    /// resources that are allowed to be inserted into the structure. If none, accept all resources
    pub allowed_inputs: Option<HashSet<Resource>>,
    pub capacity: u32,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn is_allowed(&self, resource: &Resource) -> bool {
        let Some(allowed_inputs) = &self.allowed_inputs else {
            return true;
        };

        allowed_inputs.contains(resource)
    }

    pub fn has_sufficient_many(&self, cost: &HashMap<Resource, u32>) -> bool {
        for (resource, amount) in cost.iter() {
            let Some(has) = self.resources.get(resource) else {
                return false;
            };

            if has < amount {
                return false;
            }
        }

        true
    }

    pub fn has_sufficient(&self, resource: &Resource, amount: &u32) -> bool {
        let Some(has) = self.resources.get(resource) else {
            return false;
        };

        if has < amount {
            return false;
        }

        true
    }

    pub fn subtract_checked(&mut self, resource: &Resource, amount: &u32) -> Result<(), Resource> {
        if !self.is_allowed(resource) {
            return Err(*resource);
        }

        let has = self.resources.get(resource).unwrap_or(&0);

        if has < amount {
            return Err(*resource);
        }

        self.resources.insert(*resource, has - amount);
        Ok(())
    }

    pub fn add_checked(&mut self, resource: &Resource, amount: &u32) -> Result<(), Resource> {
        if !self.is_allowed(resource) {
            return Err(*resource);
        }

        let has = self.resources.get(resource).unwrap_or(&0);

        if has + amount > self.capacity {
            return Err(*resource);
        }

        self.resources.insert(*resource, has + amount);
        Ok(())
    }

    /// Returns an error if the result goes below zero
    pub fn subtract_many_checked(&mut self, cost: &HashMap<Resource, u32>) -> Result<(), Resource> {
        for (resource, amount) in cost.iter() {
            self.subtract_checked(resource, amount)?;
        }

        Ok(())
    }

    /// Returns an error if the result exceeds the capacity of the structure
    pub fn add_many_checked(&mut self, cost: &HashMap<Resource, u32>) -> Result<(), Resource> {
        for (resource, amount) in cost.iter() {
            self.add_checked(resource, amount)?;
        }

        Ok(())
    }
}
