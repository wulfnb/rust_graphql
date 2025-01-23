# rust_graphql

## run application

> cargo build

> cargo run

## Quries

### Fetch all users:

```graphql
query {
  users {
    id
    name
    email
  }
}
```

Fetch a single user by ID:

```graphql
query {
  user(id: "1") {
    id
    name
    email
  }
}
```

## Mutations:

### Create a new user:

```graphql
mutation {
  createUser(id: "3", name: "Alice", email: "alice@example.com") {
    id
    name
    email
  }
}
```

### Update an existing user:

```graphql
mutation {
  updateUser(id: "1", name: "John Smith") {
    id
    name
    email
  }
}
```

### Delete a user:

```graphql
mutation {
  deleteUser(id: "2") {
    id
    name
    email
  }
}
```
