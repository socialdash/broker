## Broker - Real-time Zero-Code API Server

[![crates.io](https://meritbadge.herokuapp.com/broker)](https://crates.io/crates/broker)

### Purpose

The purpose of this library is to be your real-time zero-code API server. 

Broker is a SSE message broker that requires you write no backend code to have a full real-time API.

Broker is born from the need that rather than building a complex REST API with web-sockets and a SQL database to provide reactive web forms (like for React) there must be a simpler way.

Broker follows an insert-only/publish/subscribe paradigm rather than a REST CRUD paradigm. 


### How it works

In Broker you insert an event and its data via a JSON POST request (/insert) with a timestamp. Broker publishes the event when the timestamp is reached to the event stream via SSE (/events). Broker keeps all versions in its database that can be viewed in a GET request (/collections/{event}). Broker can also cancel future events in a GET request (cancel/{uuid}).

When the client first subscribes to the SSE connection (/events) all the latest events and data is sent to the client. Combined with sending the latest event via SSE when subscribed negates the necessity to do any GET API requests in the lifecycle of an event.

The side-effect of this system is that the latest event is the schema. Old events are saved in the database and are not changed but the latest event is the schema for the front-end. This is pure NoSQL as the backend is agnostic to the event data.


#### API

``` /events ```
- connect your sse-client to this endpoint

```/insert ```
- POST JSON to insert an event
```json
{"event":{...}, "published": false, "timestamp":{...}, "data":{...}}
```
- where {...} is for the event a string, timestamp is the epoch unix timestamp when you want the event to become the current event, and data is any JSON you want

``` /collections/{event} ```
- do a GET request where {event} is the name of the event you want the event log

``` /cancel/{uuid} ```
- do a GET request where (uuid) is the event uuid you want to cancel - this will mark the event published and it will not be published to the event stream - the event uuid can be obtained from the response from /insert or from /collection/{event}.

### Features

* Very performant with a low memory footprint
* Real-time Event Stream via SSE
* CORS support
* Handles SSE client timeouts
* Handles future events via Epoch UNIX timestamp
* Stateful immutable event persistence
* Insert event via JSON POST request 
* Sync latest events on SSE client connection
* Event log via GET request
* Event cancellation via GET request

### Use

```rust
use broker::{broker_run};

#[actix_rt::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    broker_run("http://localhost:3000".to_owned()).await
}
```

- the only param is the origin you want to allow - wildcard is not supported
- the PORT needs to be passed in as an environment variable
- the ORIGIN needs to be passed in as an environment variable
- the EXPIRY (for jwts) needs to be passed in as an environment variable
- the file database saves to ``` ./tmp ``` of the project root


### View Example

- [View Example React Code](https://github.com/apibillme/broker/blob/master/example/src/App.js)

### Run Example

- ``` make ```
- ``` make client ```

### Demo

- https://broker-demo.apibill.me

- **Real-time events across all connected clients:** Open the demo in two browser windows watching both. Type in the name field in one and both browser windows will display the name you typed.

- **Sync latest events on client connection:** Refresh one of the browsers and the name will be still be the current name and not blank.

### Under the Hood

- [actix-web](https://crates.io/crates/actix-web) - web framework
- [sled](https://crates.io/crates/sled) - embedded database
- [sse-actix-web](https://crates.io/crates/sse-actix-web) - sse server

### Inspiration

* [React Hooks](https://reactjs.org/docs/hooks-intro.html)
* [Meteor](https://meteor.com)
* [MongoDB](https://www.mongodb.com/)
* [Pusher](https://pusher.com)
* [Event Sourcing](https://microservices.io/patterns/data/event-sourcing.html)
* [Best in Place](https://github.com/bernat/best_in_place)
* [Brock Whitten](https://www.youtube.com/watch?v=qljYMEfVukU)
