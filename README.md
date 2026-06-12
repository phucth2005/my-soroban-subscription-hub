# ChainSubscription Hub

## Project Title
ChainSubscription Hub

## Project Description
ChainSubscription Hub is a decentralized smart contract platform to manage subscription plans and user subscriptions with automated renewals. Built using Soroban on the Stellar blockchain, it provides transparent management, enforcing subscription rules, renewal cycles, and pausing or canceling subscriptions securely and trustlessly. 

The contract is developed in Rust, utilizes strict `#![no_std]` environment constraints for WebAssembly (WASM) compatibility on-chain, and features automated lifecycle tracking through Stellar ledger timestamps.

## Project Vision
The vision of ChainSubscription Hub is to offer subscription-based businesses a decentralized, secure, and automated way to handle user subscriptions and renewals without relying on centralized intermediaries. This guarantees transparent access management and payment enforcement, increasing user trust and business reliability.

## Key Features
* **Plan Management**: Admins create and manage subscription plans specifying duration (configured in seconds for accelerated workshop testing) and pricing registry.
* **User Subscriptions**: Users can subscribe to named tiers (`Basic`, `Standard`, `Premium`) with options to explicitly toggle auto-renewal.
* **Automated Renewal**: Subscriptions auto-renew based on block timestamp expiration. The execution triggers state extension conditionally if `auto_renew` is flagged true (payment token verification is treated as an external service logic for this version).
* **Subscription Cancellation**: Users can cancel subscriptions immediately disabling future automated renewal cycles.
* **Immutable Records**: All active plans and user subscription states are recorded via explicit on-chain `DataKey` structures for full public auditability.
* **Access Control**: Identity verification enforced natively via Soroban SDK `require_auth()` to distinct Admin-restricted modifications and User-restricted subscription control.
* **Transparent Status**: Publicly accessible query methods for immediate state inspection of individual users or plan profiles.

## Usage Instructions
1. **Set Admin**: Deploy the contract and call the `initialize` function once to map the permanent `Admin` identity.
2. **Create Plans**: Admin invokes `create_plan` passing designated configurations (e.g., `"Basic"`, price: `10`, duration: `60` seconds for rapid workshop demonstration).
3. **Subscribe**: Users execute `subscribe` targeting a specific active plan name and setting the `auto_renew` boolean parameter.
4. **Auto Renew**: Anyone or an off-chain cron-scheduler can trigger `auto_renew(user_address)`. The contract validates expiration via ledger timestamps and auto-extends the sub if conditions match.
5. **Cancel**: Users invoke `cancel_subscription` to change their state mapping to inactive and opt-out of future auto-renewals.
6. **Query**: Public invocations of `get_subscription` and `get_plan` return detailed structured state profiles instantly.

## Future Scope
* **Payment Integration**: Integrate native Stellar assets (SEP-41 Soroban Tokens) or off-chain payment oracles to strictly enforce financial settlement alongside state updates.
* **Multi-tier Subscriptions**: Support multi-level or bundled plans.
* **Trial Periods**: Add trial subscriptions and discount promo codes.
* **User Dashboards**: Build interfaces for users and admins to manage subscriptions.
* **Notification System**: Add alerts for renewals, cancellations, or payment failures.
* **Cross-Platform Sync**: Sync subscriptions between decentralized services.
* **Compliance Tools**: Automate tax and regulatory compliance reporting.

## Technology Stack
* **Rust & Soroban SDK**: Embedded with `#![no_std]` core attributes for minimal, secure, and gas-efficient smart contract design.
* **Stellar Blockchain**: For decentralized, safe, and immutable state ledger management.
* **Cargo Unit Testing**: Features an automated robust isolated simulation suite inside `test.rs` utilizing `mock_all_auths` and simulated ledger block timestamp manipulation.

## Contribution
Community contributions are welcomed from blockchain developers and subscription platform experts. Fork and submit pull requests to assist in further development.

## License
This project is licensed under the MIT License.

## Contract Detail
* **ID**: CA53IH3T6HZQMDMO75KCVPZXPJHMMYXUDDDXI43DZS4YE66IPVIZUH25