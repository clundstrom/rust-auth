#!/bin/bash

# if -b is passed, build the images
if [ "$1" == "-b" ]; then
  docker build -t ldap ldap
  docker build -t authio .
  docker build -t api example
fi

docker rm -f ldap authio api example_requests || echo "No container to remove"
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


docker run \
  --net auth \
  --name api \
  -d \
  -p 8000:8000 \
  api

sleep 5

docker run \
  --net auth \
  --name example_requests \
  -e LOGIN_URL=http://authio:8080/login \
  -e PROTECTED_URL=http://api:8000/protected \
  api python "example_request.py"
