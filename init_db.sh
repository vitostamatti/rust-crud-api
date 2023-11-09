>&2 echo "Starting Postgres database"

docker compose up -d 

var=$(cat .env | grep DATABASE_URL)

>&2 echo "Exporting ${var} "

export ${var}

>&2 echo "Running sqlx migrations"

sqlx migrate run

>&2 echo "Ready to go!"