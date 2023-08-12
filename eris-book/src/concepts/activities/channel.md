# Channel Activities

## Create channel

A new channel is created, as a Service Actor.

This is triggered automatically the first time any slash command is used in a Discord channel. It can be disabled by setting "allow_new_channels" to false in the instance settings.

This **cannot** be performed from REST or GraphQL; using the slash command is required to prove that the instance has access to the channel.