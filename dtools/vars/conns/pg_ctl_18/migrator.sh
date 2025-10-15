. ${DT_VARS}/conns/pg_ctl_18/defaults.sh

user="tetrix_migrator"
password="1234567890"
database="tetrix"

GRANT="grant_user_migrator.sql"
REVOKE="revoke_user_migrator.sql"
AUX_CONN=${DT_VARS}/conns/pg_ctl_18/admin.sh

LOCALS=${DT_LOCAL_VARS}/conns/pg_ctl_18/migrator.sh
source_locals ${LOCALS}