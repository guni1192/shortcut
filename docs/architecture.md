# Basic Architecture

```mermaid
graph LR
    client[Shortcut Client]-- Create Shortcut -->api
    api[Shortcut API] -- Insert Shortcut --> DB((MySQL))
    browser[Browser] -- GET http://sc/guni --> gateway
    gateway[Shortcut Gateway]-- fetch url --> api
    api-- SELECT url from shortcut where key='guni' --> DB
    api-- return https://guni1192.com --> gateway
    gateway -- redirect url --> browser
```

* `Shortcut API` (`shortcut-api`) is gRPC server. It CRUD short URL from name.
* `Shortcut Gateway` (`shortcut-gateway`) is redirect to URL from name
* `Shortcut Client` (`sc`) is gRPC client cli.

## Shortcut API

## Shortcut Gateway

## Shortcut Client

## Improvement

* Shortcut Gateway cache mechanism for reducing latency.


## HA Stack

```mermaid
graph LR
    client[Shortcut Client]-- Create Shortcut -->api-lb
    api-lb[API Load Balancer] --> api
    api-lb[API Load Balancer] --> api2
    api[Shortcut API] --> DB((MySQL))
    api2[Shortcut API] --> DB((MySQL))
    browser[Browser] --> gateway
    gateway[Shortcut Gateway]-- fetch url --> api-lb
    gateway -- redirect url --> browser
```