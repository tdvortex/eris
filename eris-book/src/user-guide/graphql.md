# GraphQL

If you prefer to work with a [GraphQL](https://graphql.org/) API structure, Eris also offers a single GraphQL endpoint at "{instance domain}/api/graphql". This is generated and processed by [juniper](https://github.com/graphql-rust/juniper).

GraphQL is too complex to summarize statically, but interactive documentation is hosted at "{instance domain}/api-docs/graphql".

Because GraphQL combines multiple queries into one POST request, the authentication level required to fulfill a request is the highest authorization level required for any piece of the request. 

Eris supports queries and mutations, but not subscriptions.

Most Eris content is public, meaning Query requests typically can be performed without authentication. However, the queries "listChannels" and "listUsers" are admin-only.

Eris mutations all require a verified user session, and some actions require a verified admin user session.