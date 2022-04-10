# Shade Admin Contracts

Shade Admin is used to centrally authorize the owners of a contracts. A contract can query the Shade Admin Contract to confirm whether the original caller has the relevant permissions against the calling contract.

## Structure

The Shade Admin should contain the structure:

|contractHash|CallerAddress|
|-|-|
|contractName1|`authorizedCallerAddress1``authorizedCallerAddress2`|
|contractName2|`authorizedCallerAddress1`|
|contractName3|`authorizedCallerAddress1``authorizedCallerAddress2`|
|_all|`superAdminCallerAddress1`|

When is_authorized is invoked (which should have 2 variables (contractToBeAuthorizedFor, callerToBeAuthorized)), it will look up the state to confirm whether the caller is authorized as an admin or not.

Super admins can authorize against any contract and also update the authorized addresses against a contract name therefore there needs to be the following functions
remove_authorization(contractHash, callerAddress)
add_authorization(contractHash, callerAddress)
add_superadmin(contractHash, callerAddress)
remove_superadmin(contractHash, callerAddress)
