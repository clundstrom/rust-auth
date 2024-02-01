# Starts a ldap server on port 1389 that give {username: tester, password: password}
# dim-tool, perf-tool and lcm-tool access.
FROM osixia/openldap:1.5.0

WORKDIR /

ENV LDAP_ADMIN_USERNAME="admin"
ENV LDAP_ADMIN_PASSWORD="password"
ENV LDAP_DOMAIN="example.com"
ENV LDAP_ROOT="dc=example,dc=com"
ENV LDAP_SKIP_DEFAULT_TREE=no
ENV LDAP_TLS=false

COPY users.ldif .
