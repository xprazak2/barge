use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Routes {
    table: HashMap<i32, Route>,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub hops: i32,
    pub node_name: i32,
    pub direction: i32,
}

impl Routes {
    pub fn new() -> Self {
        Self{ table: HashMap::new() }
    }

    pub fn add(&mut self, route: Route) -> Option<Route>{
        self.table.insert(route.node_name, route)
    }

    pub fn remove(&mut self, key: &i32) -> Option<Route>{
        self.table.remove(key)
    }

    pub fn list(&self) -> Vec<Route> {
        self.table.values().map(|item| item.clone()).collect()
    }

    pub fn get(&self, key: &i32) -> Option<&Route> {
        self.table.get(key)
    }
}

impl Route {
    pub fn new(node_name: i32, hops: i32, direction: i32) -> Self {
        Self{ node_name, hops, direction }
    }
}