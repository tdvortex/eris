# Authentication

Eris user accounts (within an instance) are one-to-one with Discord accounts. This means that Eris relies on Discord to authenticate users. 

Eris stores no personal information about you except for your Discord [snowflake](https://discord.com/developers/docs/reference#snowflakes), a unique 64-bit number tied to when your Discord account was first created. 

Eris will **never** ask for your Discord password, your email address, or your real name. 

## Application commands

Because application commands are always sent directly from Discord to Eris, Eris treats all user data in that payload as already authenticated. So if you're using application commands, no extra steps are required.

Eris verifies the authenticity of incoming application command requests by using Discord's [Ed25519](https://discord.com/developers/docs/interactions/receiving-and-responding#security-and-authorization) signature on the message body and its timestamp (as required by Discord). 

## OAuth2

If you want to access Eris from a non-Discord client, such as a web browser or through the REST or GraphQL APIs, then depending on what actions you want to take, you may need to authenticate with Discord using [OAuth2](https://discord.com/developers/docs/topics/oauth2#oauth2)'s authorization code flow.

The process works like this:

1. Direct your client to {instance domain}/login.
2. Eris will generate a cookie, a random number unique to this client, and ask your browser to set it with a [high level of security](https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies) (Secure + HttpOnly + SameSite=Lax). This ensures that only your client and Eris know the value of this cookie, it is only sent over secure (HTTPS) connections, and it can't be stolen by JavaScript.
3. Eris will forward your browser to Discord's OAuth2 url, "https://discord.com/oauth2/authorize", with these query parameters:
    * response_type = code
    * client_id = {instance's client_id}
    * scope = identify (so Eris can see your user id)
    * state = cryptographic hash of the cookie
    * redirect_uri = {instance domain}
    * prompt = none (this makes re-authorization automatic)
4. Once you approve the "identify" scope (automatic if you previously did), Discord will redirect you to {instance domain} along with "code={authorization code}" and the "state" value.
5. Eris will check to make sure that your state matches your cookie hash, as a safeguard against [cross-site request forgery (XSRF)](https://discord.com/developers/docs/topics/oauth2#state-and-security.)
6. If the state and cookie hash match, Eris sends the authorization code, along with a client secret specific to the instance, to https://discord.com/api/oauth2/token
7. If the authorization code is valid, Discord will respond with an access token.
8. Eris will then use that access token to request "/api/users/@me" from Discord using that access token.
9. Discord responds with your user data, **excluding** your email address.
10. Eris checks to make sure it has your user id connected to an account, and if so, your cookie is now authenticated.
11. On any future requests until the cookie expires, you can provide that cookie, and Eris will know the id connected to your account.
12. You can end your session and invalidate the cookie by visiting {instance domain}/logout.