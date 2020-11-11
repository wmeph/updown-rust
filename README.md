# updown-rust
A Rust client for [updown.io](https://updown.io), with added CLI 

This requires an API key for an active updown.io account in order to work.

Build it with [cargo](https://doc.rust-lang.org/cargo/commands/cargo-build.html). The executable (in /target/release or /target/debug) can be put in your path for the CLI.

This isn't yet available as a library, but that is on the todo list!

# CLI summary

 + Use `updown` + a subcommand [+parameters] to perform requests.
 + Available parameters must be specified in long-form (i.e. --parameter, not -p). The names correspond to the parameters listed in the [updown API](https://updown.io/api).
 + The response is given in full in JSON. No pretty-printing options or verbosity level is offered yet, but they're on the TODO list.


## Usage Summary

```
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
```

## Examples

### Configure settings for a new API key

<pre>updown config <b>your-api-key</b> <b>your-private-api-key</b> <b>your-user-agent</b> 
<p>(user-agent and private-api-key are currently ignored.)</pre>

This will update the configuration file used by updown-rust, or it will create one if it doesn't exist. The details are handed off to [confy](https://docs.rs/confy/0.4.0/confy/)

### Update the period and change the alias

<pre>updown update <b>your-token</b> --period=3600 --alias="my new alias"
{"token":"v9et","url":"https://www.some_url_or_other.com","alias":"something or other","period":3600,"apdex_t":0.25,"string_match":"","enabled":true,"published":false,"disabled_locations":[],"last_check_at":"2020-10-10T00:25:42Z","custom_headers":{},"http_verb":"GET/HEAD"}</pre>

### Inspect all available checks

<pre> updown all
[{"token":<b>"your-token"</b>,"url":"http://10.255.255.1/","period":3600,"apdex_t":0.5,"enabled":true,
"published":false,"disabled_locations":[],"last_check_at":"2020-10-09T21:20:39Z",
"custom_headers":{},"http_verb":"GET/HEAD"},{"token":"v9et","url":"https://www.theatlantic.com/",
"alias":"theatlantic","period":3600,"apdex_t":0.25,"string_match":"","enabled":true,
"published":false,"disabled_locations":[],"last_check_at":"2020-10-09T21:26:28Z",
"+"custom_headers":{},"http_verb":"GET/HEAD"}]`
</pre>

### Ask for Metrics, grouped by time
<pre> updown metrics <b>"your-token"</b> --group time                                                                                                             
{"2020-11-03T12:00:00Z":{"apdex":0.994,"requests":{"samples":2880,"failures":0,"satisfied":2846,"tolerated":34,"by_response_time":
{"under125":826,"under250":1670,"under500":2846,"under1000":2870,"under2000":2880,"under4000":2880},"timings":null},"timings":
{"redirect":1245,"namelookup":0,"connection":156,"handshake":209,"response":211,"total":1821}},"2020-10-31T12:00:00Z":{"apdex":0.997,"requests":
{"samples":2878,"failures":0,"satisfied":2858,"tolerated":20,"by_response_time":
{"under125":932,"under250":1766,"under500":2858,"under1000":2877,"under2000":2878,"under4000":2878},"timings":null},"timings":
{"redirect":979,"namelookup":0,"connection":143,"handshake":162,"response":192,"total":1478}},"2020-11-10T10:00:00Z":{"apdex":0.988,"requests":
{"samples":120,"failures":0,"satisfied":117,"tolerated":3,"by_response_time":
{"under125":24,"under250":61,"under500":117,"under1000":118,"under2000":120,"under4000":120},"timings":null},"timings":
{"redirect":1450,"namelookup":0,"connection":160,"handshake":229,"response":256,"total":2095}},
... }
</pre>

### Ask for Downtimes
<pre> updown downtimes <b>"your-token"</b>


# API

The API is subject to change, but it provides:

 + Structs for the messages used in the different HTTP requests and responses as defined in [the updown API](https://updown.io/api).
 + A Configuration struct to hold references to keys
 + A Client struct with methods to call the different HTTP requests used for the different updown API functions
 
The Client requires an API key that may be entered programatically or may come from a config file (handled by [confy](https://docs.rs/confy/0.4.0/confy). A read-only key can also be supplied, though it's not used at the moment.

The messages are all serializable to JSON. 

## Examples

### Create a Client with keys and user agent (with no user details)
```rust
let client = Client::new("your-public-api-key", "your-private-api-key", "your-user-agent");
```

### Ask for Metrics
```rust
let params = MetricsParamsBuilder::default()
        .api_key(client.api_key)
        .token("your-token")
        .build().unwrap();
let mut result = client.metrics(&params);
    let metrics = result.await.unwrap();
    println!("{:?}", serde_json::to_string(&metrics));
```

### Create a Client from a config
```rust
let config = ConfigBuilder::default().api_key("your-api-key")).private_api_key("your-api-key").user_agent("your-user-agent").build();
// Maybe unwrap the config, or match the result
let client = Client::new(&config.api_key, None, None);
// Use the client as above
```

### Ask for Downtimes

```rust
let params = DowntimesParamsBuilder::default()
        .api_key(client.api_key)
        .token("your-token")
        .build().unwrap();
let mut result = client.downtimes(&params);
    let downtimes = result.await.unwrap();
    println!("{:?}", serde_json::to_string(&downtimes));
```






