# simplechat-server

## Message types and format

Messages to the server are sent in JSON. In addition to being valid JSON, they must also adhere to a certain schema, which is defined below. Any invalid message will be dropped by the server.

**Subscribing to channel**

When a client wishes to join a channel, they must send a JSON-payload like this:

```json
{
    "message_type": "join_channel",
    "channel": "<channel name>"
}
```

**Unsubscribing a channel**

When a client wants to stop receving messages from a channel, the following payload needs to be sent:

```json
{
    "message_type": "leave_channel",
    "channel": "<channel name>"
}
```

**Sending a message to a channel**

When sending a message, client supplies their username, message and channel:

```json
{
    "message_type": "message",
    "message": "<message text>",
    "channel": "<channel name>",
    "user": "<user name>"
}
```
If some sort of authentication was implemented, the `user`-field could contain some kind of authorization information, such as a token.
