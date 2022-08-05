# Shade Admin Contracts

Shade Admin is used to centrally authorize the permissions for contracts across the Shade Protocol ecosystem. A contract can query the Shade Admin Contract to confirm whether the original caller has the relevant permissions against the calling contract. 

A permission is some string that can only contain the characters A-Z, underscores, 0-9, and is not less than 10 characters. Ex: VAULT_ADMIN_1.

The most common flow in practice would be:

- `Governance / Super Admin -> Admin Auth`: Governance sends an execute msg to Admin Auth to register some user and grant them the BOND_LIMIT_ADMIN permission
- `Bonds -> Admin Auth`: Bonds queries Admin Auth with (sender address, BOND_LIMIT_ADMIN) to see if the sender has this role before allowing any bond limit specific operations

By default, the Super Admin (Governance contract) has permissions for everything so it does not need to be granted individual permissions. 

The purpose of this user specific permission delegation is to allow for the use cases that might come up in the future where governance can delegate some user (will usually be some multi-sig) with a specific set of responsibilities (like if we wanted to delegate some smaller group the ability to manage a farming contract or maintain parameters for a set of contracts or give a bot permissions to manage something). The behavior must be implemented by the consuming contract in advance so it is an opt-in on both ends.

## Structure

The Shade Admin contains the following data:

- Status(Active, Maintenance, Shutdown)
    - Active = business as usual
    - Maintenance = consumers cannot use it, but Super Admin can still manage the registry
    - Shutdown = consumers cannot use it, Super Admin can only toggle status and transfer Super Admin permissions
- Map of registered users to a list of their respective permissions
- List of registered users
- Super Admin

## Integrating

Consumers will want to implement the `validate_permission` function from [`shade_admin::querier`](./packages//shade_admin//src//querier.rs) and have a way to either store or reference the Admin Auth contract in their code. This function should be run before any authorized behavior is permitted. It will throw an error if the user does not have the requested permission. You should have the relevant permission names as constants in your contract so that you are always checking the Admin Auth contract for the proper permissions as they are case sensitive.
