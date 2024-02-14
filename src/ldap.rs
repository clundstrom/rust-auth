use crate::config::CONFIG;
use crate::permission::Permission;
use ldap3::{drive, LdapConnAsync, LdapError, LdapResult, ResultEntry, Scope, SearchEntry, SearchResult};

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
    async fn resolve_permission(&mut self, identifier: &str) -> Vec<Permission> {
        let permissions: Vec<Permission> = vec![];
        // Lookup the permissions for the user
        let search_result = self.permission_lookup(identifier).await;
        permissions
    }
}

impl Authenticate for LdapAuthenticate {
    /// Authenticate a user against the LDAP server.
    ///
    ///
    ///
    /// # Arguments
    /// * `username` - The username of the user to authenticate.
    /// * `password` - The password of the user to authenticate.
    /// # Returns
    /// * `true` if the user is authenticated, `false` otherwise.
    async fn authenticate(&mut self, username: &str, password: &str) -> bool {
        // The bind_dn is the user's username with the AD_FORMAT appended
        // Example: CN=jsmith,OU=Users,OU=Accounts,DC=example,DC=com
        let bind_dn: String = format!("CN={},{}", username, CONFIG.ad_base_dn);

        let mut ldap = match self.ldap.as_mut() {
            Some(ldap) => ldap,
            None => {
                log::error!("LDAP connection not initialized");
                return false
            },
        };

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

        return is_authenticated;
    }
}

pub struct LdapAuthenticate {
    ldap: Option<ldap3::Ldap>
}

impl LdapAuthenticate {
    pub fn new() -> LdapAuthenticate {
        Self { ldap: None }
    }

    /// Create a new LDAP connection
    ///
    /// Static function to create a new LDAP connection.
    /// Returns a Result object containing the LDAP connection and the LDAP object.
    pub async fn initialize(&mut self)
    {
        let (conn, ldap) = match LdapConnAsync::new(&CONFIG.ldap_url).await {
            Ok((conn, ldap)) => {
                log::debug!("Connection successful.");
                (conn, ldap)
            },
            Err(err) => {
                log::error!(
                    "Error connecting to LDAP server {}: {}",
                    &CONFIG.ldap_url,
                    err
                );
                panic!("Error connecting to LDAP server")
            }
        };

        drive!(conn);
        self.ldap = Some(ldap)
    }

    pub async fn unbind_ldap(&mut self) -> () {

        let ldap = self.ldap.as_mut().unwrap();

        match ldap.unbind().await {
            Ok(_) => {
                log::debug!("Unbind successful.");
            }
            Err(err) => {
                log::error!("Failed to unbind from LDAP server: {:?}", err);
            }
        }
    }

    pub(crate) async fn unpack_search_results(
        &self,
        search_result: Result<SearchResult, LdapError>,
    ) -> Vec<ResultEntry> {
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
    pub(crate) async fn permission_lookup(&mut self, identifier: &str) -> () {
        let filter = &CONFIG.ad_format;
        let attrs = CONFIG.ad_attrs.clone();

        log::debug!("Filter: {:?}", filter);
        log::debug!("Attributes: {:?}", attrs);

        let ldap = self.ldap.as_mut().unwrap();

        let search_result = ldap
            .search(&CONFIG.ad_base_dn, Scope::Subtree, &filter, attrs)
            .await;

        // Handle the Result of the search operation
        let res = self.unpack_search_results(search_result).await;

        for entry in res {
            let search_entry = SearchEntry::construct(entry);
            log::debug!("Search Entry: {:?}", search_entry);
        }
    }
}
