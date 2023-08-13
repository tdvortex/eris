# REST OpenApi

Eris exposes multiple REST endpoints, under the "/api/" root.

Note that most of these routes also have a non-API endpoint, and the return format varies based on the Accept header made with the request.

For example:
* A GET request to "{instance domain}/users/{user_id}" with an Accept header of "text/html" will return an HTML document with that user's profile, to be rendered in the browser. 
* A GET request to "{instance domain}/users/{user_id}" with an Accept header of "application/ld+json;profile="https:///www.w3.org/ns/activitystreams" will return the JSON ActivityStreams document for that Person.
* A GET request to "{instance domain}/api/users/{user_id}" with an Accept header of "application/json" will return a JSON payload describing the user, but this will *not* be in the same format as the ActivitySteams representation. 

## OpenAPI

In addition to this book, documentation for REST endpoints is generated using [utoipa](https://github.com/juhaku/utoipa) and served using [Swagger UI](https://swagger.io/) at "{instance domain}/api-docs/swagger-ui".

## Routes

Methods are denoted as public, verified, or admin based on required auth level.

**Instance**:

* /api: GET (public), PUT (admin)
* /api/banned: POST(admin)
* /api/unban: POST(admin)

**Channel**:

* /api/channels: GET(admin)
* /api/channels/{guild_id}/{channel_id}: GET(admin), DELETE(admin)
* /api/channels/{guild_id}/{channel_id}/following: GET(public)

**User**:

* /users: GET (admin) and POST (verified) operations.
* /users/{user_id}: GET(public), PUT(verified), DELETE(verified or admin)
* /users/{user_id}/liked: GET(public), POST(verified)
* /users/{user_id}/unlike: POST(verified)
* /users/{user_id}/shared: GET(public), POST(verified)
* /users/{user_id}/unshare: POST(verified)
* /users/{user_id}/followers: GET(public)
* /users/{user_id}/blocked: POST(verified)
* /users/{user_id}/unblock: POST(verified)

**Post**:

* /users/{user_id}/posts/: GET(public), POST(verified)
* /users/{user_id}/posts/{post_id}: GET(public), PUT(verified), DELETE(verified)
* /users/{user_id}/posts/{post_id}/likes: GET(public)
* /users/{user_id}/posts/{post_id}/shares: GET(public)