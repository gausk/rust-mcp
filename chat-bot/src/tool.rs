use std::collections::HashMap;
use std::sync::Arc;

use rmcp::RoleClient;
use rmcp::service::RunningService;

#[derive(Default)]
pub struct ToolSet {
    pub tools: HashMap<String, Arc<dyn Tool>>,
    pub clients: HashMap<String, RunningService<RoleClient, ()>>,
}

pub trait Tool: Sync + Send {}
