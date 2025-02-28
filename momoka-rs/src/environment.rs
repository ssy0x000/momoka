use std::str::FromStr;

use ethers::types::Address;

/// Represents different deployment environments.
#[derive(Debug, Clone)]
pub enum Deployment {
    PRODUCTION,
    STAGING,
    LOCAL,
}

impl FromStr for Deployment {
    type Err = ();

    /// Parses a string into a `Deployment` enum variant.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice representing the deployment value.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `Deployment` variant if successful, or an error if parsing fails.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PRODUCTION" => Ok(Deployment::PRODUCTION),
            "STAGING" => Ok(Deployment::STAGING),
            "LOCAL" => Ok(Deployment::LOCAL),
            _ => Err(()),
        }
    }
}

/// Contains information about the environment configuration.
#[derive(Debug)]
pub struct EnvironmentInfo {
    /// The environment value.
    pub environment: Environment,
    /// The URL of the node.
    pub node_url: String,
    /// The deployment value.
    pub deployment: Deployment,
}

/// Represents different network environments.
#[derive(Debug, Clone)]
pub enum Environment {
    POLYGON,
    MUMBAI,
    SANDBOX,
}

impl FromStr for Environment {
    type Err = ();

    /// Parses a string into an `Environment` enum variant.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice representing the environment value.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `Environment` variant if successful, or an error if parsing fails.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "POLYGON" => Ok(Environment::POLYGON),
            "MUMBAI" => Ok(Environment::MUMBAI),
            "SANDBOX" => Ok(Environment::SANDBOX),
            _ => Err(()),
        }
    }
}

/// Maps an Ethereum environment to its corresponding chain ID.
///
/// # Arguments
///
/// * `environment` - The Ethereum environment to map to a chain ID.
///
/// # Returns
///
/// The chain ID corresponding to the provided Ethereum environment.
///
/// # Errors
///
/// An error is returned if the provided environment is invalid.
#[allow(dead_code, unreachable_patterns)]
pub fn environment_to_chain_id(environment: Environment) -> Result<u32, &'static str> {
    match environment {
        Environment::POLYGON => Ok(137),
        Environment::MUMBAI | Environment::SANDBOX => Ok(80001),
        _ => Err("Invalid environment"),
    }
}

/// Maps an Ethereum environment to its corresponding Lens Hub contract address.
///
/// # Arguments
///
/// * `environment` - The Ethereum environment to map to a Lens Hub contract address.
///
/// # Returns
///
/// The Lens Hub contract address corresponding to the provided Ethereum environment.
///
/// # Errors
///
/// An error is returned if the provided environment is invalid.
#[allow(unreachable_patterns)]
pub fn environment_to_lens_hub_contract(
    environment: &Environment,
) -> Result<Address, &'static str> {
    match environment {
        Environment::POLYGON => {
            Ok(Address::from_str("0xDb46d1Dc155634FbC732f92E853b10B288AD5a1d").unwrap())
        }
        Environment::MUMBAI => {
            Ok(Address::from_str("0x60Ae865ee4C725cd04353b5AAb364553f56ceF82").unwrap())
        }
        Environment::SANDBOX => {
            Ok(Address::from_str("0x7582177F9E536aB0b6c721e11f383C326F2Ad1D5").unwrap())
        }
        _ => Err("Invalid environment"),
    }
}
