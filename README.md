# Shade Admin Contracts

Shade Admin is used to centrally authorize the owners of a contracts. A contract can query the Shade Admin Contract to confirm whether the original caller has the relevant permissions against the calling contract.

## Structure

The Shade Admin should contain the structure:

|contractHash|MsgName|CallerAddress|
|-|-|-|
|contractName1||`authorizedCallerAddress1``authorizedCallerAddress2`|
|contractName2||`authorizedCallerAddress1`|
|contractName3||`authorizedCallerAddress1``authorizedCallerAddress2`|
|_all||`superAdminCallerAddress1`|

When is_authorized is invoked (which should have 2 variables (contractToBeAuthorizedFor, callerToBeAuthorized)), it will look up the state to confirm whether the caller is authorized as an admin or not.

Super admins can authorize against any contract and also update the authorized addresses against a contract name.

## Functions
`is_authorized(contractToBeAuthorizedFor, callerToBeAuthorized, functionName = optional)`

`remove_authorization(contractHash, callerAddress, functionName = optional)`

`add_authorization(contractHash, callerAddress, functionName = optional)`

`add_superadmin(contractHash, callerAddress, functionName = optional)`

`remove_superadmin(contractHash, callerAddress, functionName = optional)`
