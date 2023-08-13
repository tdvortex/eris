# Joining an Eris instance

The easiest way to join an Eris instance is to be part of a Discord guild (aka "server") that already has an Eris instance application installed with open enrollment (the default). Find a channel where you have application command permissions, use the /join command, pick a WebFinger handle, and you're done!

If you aren't already in a guild that has an Eris instance application installed, but you have the Manage Server permissions in a Discord guild, you can add it to your guild. Depending on the instance you want to add, it might be publicly listed, or you may need to be given a private link, which the instance's developer can generate from Discord's OAuth2 > URL Generator page. 

The only scope (permission) that Eris needs to function is "applications.commands", which should give a URL like "https://discord.com/api/oauth2/authorize?client_id={CLIENT_ID}&scope=applications.commands".

Note that because Eris is federated, different instances will have different Discord apps, with different Client IDs. If you use Eris from two different instances, those are treated like two completely different accounts. 

Similarly, if you have two different Eris instance applications installed on the same Discord guild, you will have two different sets of application commands with the same names. When you use a command, your interaction will be specific to one of the two instances, depending on which app's commands you use. This is very confusing, so we recommend only installing one Eris application per Discord guild.

If you aren't in a server with Eris installed and don't want to set one up, you can still join an instance, although you won't get the full experience. Find the website URL for your instance, and go to its "/login" page. It will redirect you to Discord for authorization, and then you're ready to start making API calls to the REST or GraphQL endpoints.