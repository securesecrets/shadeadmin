# Shade Admin Contracts

Shade Admin is used to centrally authorize the owners of a contracts. A contract can query the Shade Admin Contract to confirm whether the original caller has the relevant permissions against the calling contract.

## Structure

The Shade Admin should contain the structure:

|contractHash|CallerAddress|
|-|-|
|contractName1|`authorizedCallerAddress1``authorizedCallerAddress2`|
|contractName2|`authorizedCallerAddress1`|
|contractName3|`authorizedCallerAddress1``authorizedCallerAddress2`|

When is_authorized is invoked (which should have 2 variables (contractToBeAuthorizedFor, callerToBeAuthorized)), it will look up the state to confirm whether the caller is authorized as an admin or not.

