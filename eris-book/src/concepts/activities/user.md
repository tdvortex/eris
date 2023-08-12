# User Activities

These are actions taken by a verified Discord user.

## Create user





The instance creates a new user (Person) profile, tied to a specific Discord user.

This is triggered by a Discord user executing "/join" slash command. If the instance's enrollment is set to "open", and the user doesn't already have a profile on the instance, then they will be prompted to pick a WebFinger handle, which must be unique on the instance. If the instance's enrollment is set to "closed", the request will be rejected.

This can also be executed with a POST request to "/users", or by using the "createUser" mutation in GraphQL. It requires a verified Discord user session.