#!/bin/bash

docker build -t ldap ldap
docker rm -f ldap || echo "No container to remove"

docker run \
	--network=host \
	--name ldap \
	-d \
	-e LOG_LEVEL=debug \
	-p 389:389 \
	-p 636:636 \
	ldap

sleep 5
docker exec ldap ldapadd -x -D "cn=admin,dc=example,dc=com" -w password -f users.ldif
