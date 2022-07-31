# XRPL Node Manager

**Work in progress**

Features:

- Start server
- Configuration - Change node configuration
- Log Level - Get or modify log verbosity.
- Server info
- Ledgers
- Stop - Shut down the rippled server.

## TODO

- can_delete - Allow online deletion of ledgers up to a specific ledger.
- download_shard - Download a specific shard of ledger history.
- ledger_cleaner - Configure the ledger cleaner service to check for corrupted data.
- ledger_request - Query a peer server for a specific ledger version.
- logrotate - Reopen the log file.
- node_to_shard - Copy data from the ledger store to the shard store.
- ledger_accept - Close and advance the ledger in stand-alone mode.
- peers - Get information about the peer servers connected.
- consensus_info - Get information about the state of consensus as it happens.
- feature - Get information about protocol amendments.
- fetch_info - Get information about the server's sync with the network.
- get_counts - Get statistics about the server's internals and memory usage.
- manifest - Get the latest public key information about a known validator.
- print - Get information about internal subsystems.
- validator_info - Get information about the server's validator settings, if configured as a validator.
- validator_list_sites - Get information about sites that publish validator lists.
- validators - Get information about the current validators.

## Stuff

```
curl -X POST 'http://localhost:8080/register' -H 'Content-Type: application/json' -d '{ "user_id": 1 }'
```

```
curl -X DELETE 'http://localhost:8080/register/e2fa90682255472b9221709566dbceba' 
```

```
curl -X POST 'http://localhost:8080/publish' \
    -H 'Content-Type: application/json' \
    -d '{"user_id": 1, "topic": "cats", "message": "are awesome"}'
```

## Contributing

Contributors are welcome, please fork and send pull requests! If you find a bug
or have any ideas on how to improve this project please submit an issue.
