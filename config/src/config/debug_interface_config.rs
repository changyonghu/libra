// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::utils;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct DebugInterfaceConfig {
    pub admission_control_node_debug_port: u16,
    // This has similar use to the core-node-debug-server itself
    pub metrics_server_port: u16,
    pub public_metrics_server_port: u16,
    pub address: String,
    pub libra_trace: LibraTraceConfig,
}

impl Default for DebugInterfaceConfig {
    fn default() -> DebugInterfaceConfig {
        DebugInterfaceConfig {
            admission_control_node_debug_port: 6191,
            metrics_server_port: 9101,
            public_metrics_server_port: 9102,
            address: "0.0.0.0".to_string(),
            libra_trace: LibraTraceConfig::default(),
        }
    }
}

impl DebugInterfaceConfig {
    pub fn randomize_ports(&mut self) {
        self.admission_control_node_debug_port = utils::get_available_port();
        self.metrics_server_port = utils::get_available_port();
        self.public_metrics_server_port = utils::get_available_port();
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct LibraTraceConfig {
    pub sampling: SamplingConfig,
}

impl Default for LibraTraceConfig {
    fn default() -> LibraTraceConfig {
        LibraTraceConfig {
            sampling: SamplingConfig::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct SamplingConfig {
    pub txn: String,
    pub block: String,
}

impl Default for SamplingConfig {
    fn default() -> SamplingConfig {
        SamplingConfig {
            txn: "1/100".to_string(),
            block: "1/1".to_string(),
        }
    }
}
