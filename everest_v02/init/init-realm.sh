#!/bin/bash

/opt/keycloak/bin/kcadm.sh config credentials --server http://keycloak:5080 --realm master --user admin --password admin

/opt/keycloak/bin/kcadm.sh create realms -s realm=myrealm -s enabled=true

/opt/keycloak/bin/kcadm.sh create clients -r myrealm -s clientId=myclient -s enabled=true -s 'redirectUris=["http://localhost:5080/*"]'

/opt/keycloak/bin/kcadm.sh create roles -r myrealm -s name=user
/opt/keycloak/bin/kcadm.sh create roles -r myrealm -s name=admin
/opt/keycloak/bin/kcadm.sh create roles -r myrealm -s name=operator
/opt/keycloak/bin/kcadm.sh create roles -r myrealm -s name=partner