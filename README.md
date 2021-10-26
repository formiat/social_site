# social_site

Social site with video-chats using WebRTC


Currently in development


## How to run:

1. install Rust nightly
2. install PostgreSQL
3. install Diesel CLI
4. go to project root
5. set your DB user and password in variable "DATABASE_URL" in ".env" file
6. run "diesel setup"
7. run "cargo run"
8. go to http://localhost:8000 with your browser to check whether server is running


## API

1. user add
   - type: POST
   - url: /api/v1/user
   - body:
    ```json
    {
        "login":"login3",
        "password_hash":"789"
    }
    ```

2. room add
   - type: POST
   - url: /api/v1/room
   - body:
    ```json
    {
      "user_1_id":1,
      "user_2_id":3
    }
    ```

3. user list
   - type: GET
   - url: /api/v1/user

4. room list
   - type: GET
   - url: /api/v1/room



## Front-end:

1. user list - /users.html
2. room list - /rooms.html

