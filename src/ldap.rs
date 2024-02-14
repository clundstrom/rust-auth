use crate::access::Access;
use crate::config::CONFIG;
use crate::permission::Permission;
use ldap3::{
    drive, Ldap, LdapConnAsync, LdapError, Scope, SearchEntry,
    SearchResult,
};

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

        if search_result.len() == 0 {
            log::info!("No permissions found for user: {}", identifier);
            return permissions;
        }

        let permissions: Vec<Permission> = Self::parse_search_entry(search_result);

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

        let ldap = match self.ldap.as_mut() {
            Some(ldap) => ldap,
            None => {
                log::error!("LDAP connection not initialized");
                return false;
            }
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
    ldap: Option<Ldap>,
}

impl LdapAuthenticate {
    pub fn new() -> LdapAuthenticate {
        Self { ldap: None }
    }

    /// Extract the permissions from the search results
    ///
    /// Map the search results to a vector of Permission objects
    /// Default the access_type to READ
    pub fn parse_search_entry(entries: Vec<SearchEntry>) -> Vec<Permission> {
        let mut permissions: Vec<Permission> = vec![];

        for entry in entries {
            log::debug!("DN: {}", entry.dn);

            if let Some(member_of) = entry.attrs.get("memberOf") {
                for group in member_of {
                    let group_name: Vec<&str> = group.split(",").collect();
                    let group_name: Vec<&str> = group_name[0].split("=").collect();
                    let group_name: &str = group_name[1];
                    log::debug!("Group Name: {}", group_name);

                    let perm = Permission {
                        name: group_name.to_string(),
                        description: entry.dn.clone(),
                        access_type: Access::READ,
                    };

                    permissions.push(perm);
                }
            }
        }
        permissions
    }

    /// Create a new LDAP connection
    ///
    /// Static function to create a new LDAP connection.
    /// Returns a Result object containing the LDAP connection and the LDAP object.
    pub async fn initialize(&mut self) {
        let (conn, ldap) = match LdapConnAsync::new(&CONFIG.ldap_url).await {
            Ok((conn, ldap)) => {
                log::debug!("Connection successful.");
                (conn, ldap)
            }
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
        let ldap = match self.ldap.as_mut() {
            Some(ldap) => ldap,
            None => {
                log::warn!("LDAP connection not initialized");
                return;
            }
        };

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
    ) -> Vec<SearchEntry> {
        let entries = match search_result {
            Ok(result) => match result.success() {
                Ok((entries, _)) => {
                    log::info!("Search OK. Entries: {:?}", entries.len());
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

        // Construct vec of SearchEntry from ResultEntry using map
        let mut search_entries: Vec<SearchEntry> = vec![];

        for entry in entries {
            search_entries.push(SearchEntry::construct(entry));
        }

        search_entries
    }

    /// Lookup the permissions for a user.
    pub(crate) async fn permission_lookup(&mut self, identifier: &str) -> Vec<SearchEntry> {
        let filter: &str = &CONFIG.ad_filter_format;
        let attrs: Vec<String> = CONFIG.ad_attrs.clone();
        let bind_dn = format!("CN={},{}", identifier, CONFIG.ad_base_dn);

        log::debug!("Search base DN: {}", &CONFIG.ad_base_dn);
        log::debug!("Filter: {:?}", filter);
        log::debug!("Attributes: {:?}", attrs);

        let ldap: &mut Ldap = match self.ldap.as_mut() {
            Some(ldap) => ldap,
            None => {
                log::warn!("LDAP connection not initialized");
                return vec![];
            }
        };

        let search_result: Result<SearchResult, LdapError> =
            ldap.search(&bind_dn, Scope::Subtree, filter, attrs).await;

        self.unpack_search_results(search_result).await
    }
}
