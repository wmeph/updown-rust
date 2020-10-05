# updown-rust
A Rust client for [updown.io](https://updown.io), with added CLI 

This requires an API key for an active updown.io account in order to work. 

# API

The API is subject to change, but it provides:

 + Structs for the messages used in the different HTTP requests and responses as defined in [the updown API](https://updown.io/api).
 + A Configuration struct to hold references to keys
 + A Client struct with methods to call the different HTTP requests used for the different updown API functions
 
The Client requires an API key that may be entered programatically or may come from a config file (handled by [confy](https://docs.rs/confy/0.4.0/confy). A read-only key can also be supplied, though it's not used at the moment.

The messages are all serializable to JSON. 

## Examples


### Create a Client from a config

TODO (API is still not very nice!) 

### Send a request for downtimes from a client

TODO (Client methods are coupled to arg params, which must be fixed!)

# CLI summary

 + Use `updown` + a subcommand [+parameters] to perform requests.
 + Available parameters must be specified in long-form. The names correspond to the parameters listed in the [updown API](https://updown.io/api).
 + The response is given in full in JSON. No pretty-printing options or verbosity level is offered yet, but they're on the TODO list.

## Examples

### Configure settings for a new API key

`updown config :API_KEY`

### Update the period and change the alias

`updown update :TOKEN --period=3600 --alias="my new alias"`

### Inspect all available checks

`updown all`

USAGE:
    updown [token_or_url] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <token_or_url>    

SUBCOMMANDS:
    add          
    all          
    check        
    config       
    delete       
    downtimes    
    help         Prints this message or the help of the given subcommand(s)
    metrics  




