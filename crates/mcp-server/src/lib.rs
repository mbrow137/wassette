// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

pub use wassette::LifecycleManager;

mod components;
mod logging;
mod prompts;
mod resources;
mod tools;

#[cfg(test)]
mod logging_tests;

pub use logging::{data, McpLogger};
pub use prompts::handle_prompts_list;
pub use resources::handle_resources_list;
pub use tools::{handle_tools_call, handle_tools_list};
