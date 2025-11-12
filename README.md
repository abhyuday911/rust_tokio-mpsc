to have non blocking bla-bla nam bhul gye.

## Version 0
- Create an mpsc channnel (send and receive at the same place).

## Version 1
- Send from api route & receive in fn engine.

## Architecture for what needs to be done.
- You have the mpsc in the main fnc, send the sender to the routes and send the receiver to the engine.
- another sender & receiver to send things from engine to the route probably.
