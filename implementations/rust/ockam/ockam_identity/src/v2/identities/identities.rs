use super::super::identities::{IdentitiesKeys, IdentitiesRepository, IdentitiesVault};
use super::super::{
    IdentitiesBuilder, IdentitiesCreation, IdentitiesReader, IdentitiesStorage, PurposeKeys,
};
use ockam_core::compat::sync::Arc;
use ockam_vault::Vault;

/// This struct supports all the services related to identities
#[derive(Clone)]
pub struct Identities {
    pub(crate) vault: Arc<dyn IdentitiesVault>,
    pub(crate) identities_repository: Arc<dyn IdentitiesRepository>,
}

impl Identities {
    /// Return the identities vault
    pub fn vault(&self) -> Arc<dyn IdentitiesVault> {
        self.vault.clone()
    }

    /// Return the identities repository
    pub fn repository(&self) -> Arc<dyn IdentitiesRepository> {
        self.identities_repository.clone()
    }

    pub fn purpose_keys(&self) -> Arc<PurposeKeys> {
        Arc::new(PurposeKeys::new(self.vault.clone(), self.identities_keys()))
    }

    /// Return the identities keys management service
    pub fn identities_keys(&self) -> Arc<IdentitiesKeys> {
        Arc::new(IdentitiesKeys::new(self.vault.clone()))
    }

    /// Return the identities creation service
    pub fn identities_creation(&self) -> Arc<IdentitiesCreation> {
        Arc::new(IdentitiesCreation::new(
            self.repository(),
            self.vault.clone(),
        ))
    }

    /// Return the identities reader
    pub fn identities_reader(&self) -> Arc<dyn IdentitiesReader> {
        self.repository().as_identities_reader()
    }

    // /// Return the identities credentials service
    // pub fn credentials(&self) -> Arc<dyn Credentials> {
    //     Arc::new(self.clone())
    // }
    //
    // /// Return the identities credentials server
    // pub fn credentials_server(&self) -> Arc<dyn CredentialsServer> {
    //     Arc::new(CredentialsServerModule::new(self.credentials()))
    // }
}

impl Identities {
    /// Create a new identities module
    pub(crate) fn new(
        vault: Arc<dyn IdentitiesVault>,
        identities_repository: Arc<dyn IdentitiesRepository>,
    ) -> Identities {
        Identities {
            vault,
            identities_repository,
        }
    }

    /// Return a default builder for identities
    pub fn builder() -> IdentitiesBuilder {
        IdentitiesBuilder {
            vault: Vault::create(),
            repository: IdentitiesStorage::create(),
        }
    }
}