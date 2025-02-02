## Messages in the scope of integrations

Integrations are passed the TX end of a channel in their `new` associated functions. This channel can be used for sending events to homectl core.

The type of messages that can be sent is defined in the Message enum,
in `types/src/event.rs`.

Useful messages for integrations are:

## IntegrationDeviceRefresh
Serves two purposes: both registering new devices to homectl and sending device state updates to homectl.

### Registration
Send this message to homectl in the `register` associated function of your
integration for each device that you've discovered.

### Device state update
Send this message to homectl whenever you gather information about current
device state, for example through polling. It doesn't matter if state actually
changed from when you last sent this event, homectl core will take care of
diffing the state for you.
