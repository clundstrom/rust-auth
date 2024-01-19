use crate::config::CONFIG;
use ldap3::{LdapConnAsync, LdapError, Scope, SearchEntry, SearchResult};

use crate::traits::authenticate::Authenticate;

impl Authenticate for LdapAuthenticate {
    fn authenticate(&self, username: &str, password: &str) -> bool {
        self.username == username && self.password == password
    }
}

pub(crate) struct LdapAuthenticate {
    username: String,
    password: String,
}

impl LdapAuthenticate {
    pub(crate) fn new(username: &str, password: &str) -> LdapAuthenticate {
        LdapAuthenticate {
            username: username.to_owned(),
            password: password.to_owned(),
        }
    }

    pub(crate) async fn bind(&mut self) -> Result<(), LdapError> {
        let (conn, mut ldap) = match LdapConnAsync::new(&CONFIG.ldap_url).await {
            Ok((conn, ldap)) => (conn, ldap),
            Err(err) => {
                log::error!(
                    "Error connecting to LDAP server {}: {}",
                    &CONFIG.ldap_url,
                    err
                );
                return Err(err);
            }
        };

        ldap3::drive!(conn);

        let attrs = vec![
            "objectClass",
            "cn",
            "sn",
            "description",
            "displayName",
            "employeeType",
            "givenName",
            "jpegPhoto",
            "mail",
            "ou",
            "title",
            "uid",
            "userPassword",
        ];

        let search_entries: Result<SearchResult, LdapError> = ldap
            .search(
                "ou=people,dc=ldapmock,dc=local",
                Scope::Subtree,
                "(objectClass=inetOrgPerson)",
                vec!["*"],
            )
            .await;

        // Handle the Result of the search operation
        let res = match search_entries {
            Ok(result) => {
                // Check if the LDAP operation was successful
                match result.success() {
                    Ok((entries, _)) => entries, // Assuming `entries` is the desired data
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
