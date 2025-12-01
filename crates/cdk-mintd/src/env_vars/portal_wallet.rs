//! PortalWallet environment variables

use std::env;

use cdk::nuts::CurrencyUnit;

use crate::config::PortalWallet;

// Fake Wallet environment variables
pub const ENV_PORTAL_WALLET_SUPPORTED_UNITS: &str = "CDK_MINTD_PORTAL_WALLET_SUPPORTED_UNITS";

impl PortalWallet {
    pub fn from_env(mut self) -> Self {
        // Supported Units - expects comma-separated list
        if let Ok(units_str) = env::var(ENV_PORTAL_WALLET_SUPPORTED_UNITS) {
            if let Ok(units) = units_str
                .split(',')
                .map(|s| s.trim().parse())
                .collect::<Result<Vec<CurrencyUnit>, _>>()
            {
                self.supported_units = units;
            }
        }

        self
    }
}
