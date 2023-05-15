# Upcoming

# 0.1.2

- BUGFIX: Client occasionally used to get stuck in a hot loop after network
  disconnect.
- Tweak backoff params for better performance across network disconnect.
- Minor improvements to convex_chat_client example
- Minor fix to running tests
- Bump tokio-tungstenite to 0.18

# 0.1.1

- Fix race between mutation result and dropping a subscription.
- Minor logging/error message improvements.

# 0.1.0

- Initial release.
- Support for queries, subscriptions, mutations, actions
