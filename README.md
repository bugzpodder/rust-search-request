# rust-search-request
rust web app

Rust app that converts a predefined schema into a SQL query.

Example: https://hopp.sh/r/YXcClRRYHJ1v

```json
{
  "where_clause": {
    "wallet_name": {
      "ilike": "%foo"
    },
    "or": {
      "wallet_type": {
        "is_in": ["public", "private"]
      },
      "wallet_id": {
        "is_not_null": true
      }
    }
  },
  "order_by": [{
    "field": "wallet_id",
    "dir": "desc"
  }],
  "limit": 10,
  "offset": 5
}
```
