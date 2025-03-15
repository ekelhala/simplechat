# simplechat-server

## Message types and format

Messages to the server are sent in JSON. In addition to being valid JSON, they must also adhere to a certain schema, which is defined below. Any invalid message will be dropped by the server.

**Subscribing to channel**

When a client wishes to join a channel, they must send a JSON-payload like this:

```json
{
    "channel": "<channel name>"
}
```

**Sending a message to a channel**

When sending a message, client supplies their username, message and channel:

```json
{
    "message": "<message text>",
    "channel": "<channel name>",
    "user": "<user name>"
}
```
If some sort of authentication was implemented, the `user`-field could contain some kind of authorization information, such as a token.
