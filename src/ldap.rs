use crate::config::CONFIG;
use crate::permission::Permission;
use ldap3::{LdapConnAsync, LdapError, LdapResult, ResultEntry, Scope, SearchEntry, SearchResult};

use crate::traits::authenticate::Authenticate;
use crate::traits::authorize::Authorize;

impl Authorize for LdapAuthenticate {
    /// Resolve the permissions for a user.
    ///
    /// If an error occurs during the permission lookup, an empty vector is returned.
    ///
    /// # Arguments
    /// * `identifier` - The identifier of the user to resolve permissions for.
    /// # Returns
    /// * A vector of `Permission` objects for the user.
    async fn resolve_permission(&self, identifier: &str) -> Vec<Permission> {
        let permissions: Vec<Permission> = vec![];

        // Lookup the permissions for the user
        let search_result = self.permission_lookup(identifier).await;

        // Todo: Implement the authorization mapping

        permissions
    }
}

impl Authenticate for LdapAuthenticate {
    /// Authenticate a user against the LDAP server.
    ///
    /// # Arguments
    /// * `username` - The username of the user to authenticate.
    /// * `password` - The password of the user to authenticate.
    /// # Returns
    /// * `true` if the user is authenticated, `false` otherwise.
    async fn authenticate(&self, username: &str, password: &str) -> bool {
        let (conn, mut ldap) = match self.create_ldap_connection().await {
            Ok((conn, ldap)) => (conn, ldap),
            Err(_) => return false,
        };

        ldap3::drive!(conn);

        // The bind_dn is the user's username with the AD_FORMAT appended
        // Example: CN=jsmith,OU=Users,OU=Accounts,DC=example,DC=com
        let bind_dn: String = format!("CN={},{}", username, CONFIG.ad_base_dn);

        let is_authenticated: bool = match ldap.simple_bind(&bind_dn, password).await {
            Ok(res) => {
                if res.success().is_ok() {
                    log::debug!("Bind successful: Authenticated");
                    true
                } else {
                    log::debug!("Bind failed: Invalid credentials");
                    false
                }
            }
            Err(err) => {
                log::error!("Bind failed: {}", err);
                false
            }
        };

        self.unbind_ldap(&mut ldap).await;
        return is_authenticated;
    }
}

pub struct LdapAuthenticate {}

impl LdapAuthenticate {
    pub fn new() -> LdapAuthenticate {
        LdapAuthenticate {}
    }

    pub(crate) async fn create_ldap_connection(
        &self,
    ) -> Result<(LdapConnAsync, ldap3::Ldap), LdapError> {
        match LdapConnAsync::new(&CONFIG.ldap_url).await {
            Ok((conn, ldap)) => {
                Ok((conn, ldap))
            },
            Err(err) => {
                log::error!(
                    "Error connecting to LDAP server {}: {}",
                    &CONFIG.ldap_url,
                    err
                );
                Err(err)
            }
        }
    }

    pub(crate) async fn unbind_ldap(&self, ldap: &mut ldap3::Ldap) -> () {
        match ldap.unbind().await {
            Ok(_) => {
            }
            Err(err) => {
                log::error!("Failed to unbind from LDAP server: {:?}", err);
            }
        }
    }

    pub(crate) async fn unpack_search_results(&self, search_result: Result<SearchResult, LdapError>) -> Vec<ResultEntry> {
        return match search_result {
            Ok(result) => match result.success() {
                Ok((entries, _)) => {
                    log::debug!("Vector of entries: {:?}", entries);
                    entries
                }
                Err(e) => {
                    log::error!("No results: {}", e);
                    vec![]
                }
            },
            Err(e) => {
                log::error!("LdapError: {}", e);
                vec![]
            }
        };
    }

    /// Lookup the permissions for a user.
    pub(crate) async fn permission_lookup(&self, username: &str) -> () {
        let (conn, mut ldap) = match self.create_ldap_connection().await {
            Ok((conn, ldap)) => (conn, ldap),
            Err(err) => {
                log::error!("Error creating LDAP connection: {}", err);
                // Return early if there is an error creating the LDAP connection
                return;
            }
        };

        ldap3::drive!(conn);

        let filter = &CONFIG.ad_format;
        let attrs = CONFIG.ad_attrs.clone();


        log::debug!("Filter: {:?}", filter);
        log::debug!("Attributes: {:?}", attrs);

        let search_result = ldap
            .search(&CONFIG.ad_base_dn, Scope::Subtree, &filter, attrs)
            .await;

        self.unbind_ldap(&mut ldap).await;

        // Handle the Result of the search operation
        let res = self.unpack_search_results(search_result).await;

        for entry in res {
            let search_entry = SearchEntry::construct(entry);
            log::debug!("Search Entry: {:?}", search_entry);
        }
    }
}
