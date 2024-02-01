#!/bin/bash

docker build -t ldap ldap
docker rm -f ldap || echo "No container to remove"

docker run \
	--network=host \
	--name ldap \
	-p 389:389 \
	-p 636:636 \
	-v //users.ldif:/home/ldif/users.ldif \
	ldap
