set -x;
PGPASSWORD=password psql -h localhost gaime gaimemaster -f drop.sql -f create.sql;



PGPASSWORD=password psql -h localhost gaime gaimemaster -f select.sql;

curl 'http://localhost:8000/games';

curl -d '{"username":"bob", "password":"ratatouille", "email:bob@gaime.org"}' -H 'Content-Type: application/json' -X POST 'http://localhost:8000/signup';

