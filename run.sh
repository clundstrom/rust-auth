#!/bin/bash

# if -b is passed, build the images

if [ "$1" == "-b" ]; then
  docker build -t ldap ldap
  docker build -t authio .
fi

docker rm -f ldap authio || echo "No container to remove"
docker network create auth

docker run \
  --net auth \
	--name ldap \
	-d \
	-p 389:389 \
	-p 636:636 \
	ldap

sleep 5
docker exec ldap ldapadd -x -D "cn=admin,dc=example,dc=com" -w password -f users.ldif

sleep 5

docker run \
  --net auth \
  --name authio \
  -d \
  -p 8080:8080 \
  --env-file .env \
  -e LDAP_URL=ldap://ldap:389 \
  -e RUST_LOG=debug \
  -e RUST_BACKTRACE=1 \
  authio
