# Rust rocket.rs - API with Authentication

An example of API written in Rust with the rocket.rs framework, with a JWT Authentication 

## Requirements

1. Configure Rust to satisfy rocket.rs dependencies

## Installation

1. First run the migration
    ```bash
    diesel migration run
    ```
2. Compile the code setting the DATABASE_URL environment variable
    ```bash
        export DATABASE_URL=mysql://username:password@localhost/heroes && cargo run
    ```

## API

### /auth/login
Get a jwt token for the user marcocastignoli
```bash
curl -X POST \
  http://localhost:8000/auth/login \
  -H 'content-type: application/json' \
  -d '{
	"username": "marcocastignoli",
	"password": "12345"
}'
```
### /user
Call a protected route (use the token returned from the /auth/login API)
```bash
curl -X GET \
  http://localhost:8000/user \
  -H 'authentication: eyJ0eXAiOiJKV1QiLCJraWQiOm51bGwsImFsZyI6IkhTMjU2In0.eyJpc3MiOm51bGwsInN1YiI6Im1hcmNvY2FzdGlnbm9saSIsImF1ZCI6bnVsbCwiZXhwIjpudWxsLCJuYmYiOm51bGwsImlhdCI6bnVsbCwianRpIjpudWxsfQ.fnp0D8Qh1bTFv1zKTVGAxwjtyTCOqKuarRzBQabjiCI'
```

## Thanks
Special thanks to @sean3z for this repo https://github.com/sean3z/rocket-diesel-rest-api-example and this tutorial https://medium.com/sean3z/building-a-restful-crud-api-with-rust-1867308352d8