use crate::config::CONFIG;
use ldap3::{LdapConnAsync, LdapError, Scope, SearchEntry, SearchResult};
use crate::permission::Permission;

use crate::traits::authenticate::Authenticate;
use crate::traits::authorize::Authorize;

impl Authorize for LdapAuthenticate{
    async fn resolve_permission(&self, identifier: &str) -> Vec<Permission> {
        todo!()



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
        let bind_dn: String = format!("CN={},{}", username, &CONFIG.ad_format);

        let is_authenticated: bool = match ldap.simple_bind(&bind_dn, password).await {
            Ok(res) => {
                if res.success().is_ok() {
                    log::debug!("Authenticated");
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

        match ldap.unbind().await {
            Ok(_) => {
                log::debug!("Successfully unbound from LDAP server");
            }
            Err(err) => {
                log::error!("Failed to unbind from LDAP server: {:?}", err);
            }
        }
        return is_authenticated;
    }
}

pub(crate) struct LdapAuthenticate {}

impl LdapAuthenticate {
    pub(crate) fn new() -> LdapAuthenticate {
        LdapAuthenticate {}
    }

    pub(crate) async fn create_ldap_connection(&self) -> Result<(LdapConnAsync, ldap3::Ldap), LdapError> {
        match LdapConnAsync::new(&CONFIG.ldap_url).await {
            Ok((conn, ldap)) => Ok((conn, ldap)),
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

    pub(crate) async fn bind(&mut self, username: &str) -> Result<(), LdapError> {
        let (conn, mut ldap) = match self.create_ldap_connection().await {
            Ok((conn, ldap)) => (conn, ldap),
            Err(e) => return Err(e),
        };

        ldap3::drive!(conn);

        // The bind_dn is the user's username with the AD_FORMAT appended
        // Example: CN=jsmith,OU=Users,OU=Accounts,DC=example,DC=com
        let bind_dn = format!("CN={},{}", username, &CONFIG.ad_format);

        // Ldap Search
        let filter = format!("(&(objectClass=person)(cn={}))", username);
        let attrs = vec![
            "cn",
            "title",
            "memberOf",
            "mail",
            "thumbnailPhoto",
            "displayName",
        ];
        let search_entries: Result<SearchResult, LdapError> = ldap
            .search(&CONFIG.ad_base_dn, Scope::Subtree, &filter, attrs)
            .await;

        // Handle the Result of the search operation
        let res = match search_entries {
            Ok(result) => {
                // Check if the LDAP operation was successful
                match result.success() {
                    Ok((entries, _)) => {
                        println!("Entry {:?}", entries);
                        entries
                    } // Assuming `entries` is the desired data
                    Err(e) => {
                        // Handle LDAP operation failure
                        eprintln!("LDAP operation failed: {}", e);
                        return Err(e.into()); // Convert LDAP error to your function's error type
                    }
                }
            }
            Err(e) => {
                return Err(e.into()); // Convert LDAP error to your function's error type
            }
        };

        let result_length = res.len();

        if result_length == 0 {
            println!("No entries found");
        } else {
            println!("Found {} entries", result_length);
        }

        for entry in res {
            let search_entry = SearchEntry::construct(entry);
            println!("{:?}", search_entry);
            println!("");
        }

        ldap.unbind().await?;

        Ok(())
    }
}
