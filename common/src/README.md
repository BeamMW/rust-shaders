# BVM Functions Implementation Status

This document tracks the implementation status of BVM (Beam Virtual Machine) functions and types in the Rust bindings.

## Types and Structs Implementation Status

### Primitive types
- [x] `Height` - 64-bit unsigned integer (`u64`), denotes blockchain height
- [x] `Amount` - 64-bit unsigned integer (`u64`), denotes value in basic units (groths)
- [x] `AssetID` - 32-bit integer (`u32`), used as the asset identifier. Value `0` is reserved for _Beam_
- [x] `Timestamp` - 64-bit unsigned integer (`u64`), UTC time in seconds

### Opaque types
- [x] `PubKey` - 33-byte long representation of a public key (secp256k1 group element, i.e. point)
- [x] `ShaderID` - 32-byte long shader ID (derived from its compiled bytecode)
- [x] `ContractID` - 32-byte long contract ID
- [ ] `HashObj` - opaque handle, represents a hash processor object managed by BVM
- [ ] `Secp_scalar` - opaque handle, represents a secp256k1 scalar object managed by BVM
- [x] `Secp_point` - opaque handle, represents a secp256k1 point object managed by BVM

### Additional types (implemented in Rust bindings)
- [x] `HashValue` - 32-byte hash value
- [x] `SecpScalarData` - 32-byte secp scalar data
- [x] `KeyID` - Key identifier (alias for SigRequest)

### Structs and Complex types
- [x] `FundsChange` - Structure for fund changes (amount, asset ID, consume flag)
- [x] `SigRequest` - Structure for signature requests (ID pointer and size)
- [ ] `BlockHeader` - Block header structure
- [x] `KeyTag` - Key tag constants and utilities
- [x] `Merkle::Node` - Merkle tree node structure
- [x] `HeightPos` - Height and position structure
- [x] `SecpPointData` - Secp point data structure (x coordinates and y flag)
- [x] `KeyPrefix` - Key prefix structure (contract ID and tag)
- [x] `Key<T>` - Generic key structure with prefix and contract-specific key
- [x] `VarReaderEx<FLEXIBLE>` - Variable reader with flexible enumeration
- [x] `LogReader` - Log reader structure
- [x] `ContractsWalker` - Contract walker for enumeration

### Type Implementation Statistics
- **Total Types**: 15
- **Implemented**: 12
- **Missing**: 3

### Missing Types:
- `HashObj` - Hash processor object handle
- `Secp_scalar` - Secp256k1 scalar object handle  
- `BlockHeader` - Block header structure

## Common Functions (for both Contract and App shaders)

### General
- [x] `Halt` - Halt execution
- [ ] `get_Height` - Get current blockchain height
- [ ] `get_HdrInfo` - Get block header info
- [ ] `get_HdrFull` - Get full block header
- [ ] `get_RulesCfg` - Get rules configuration
- [ ] `Write` - Write data to stream

### Raw memory utility functions
- [x] `Memcpy` - Memory copy
- [x] `Memset` - Memory set
- [x] `Memcmp` - Memory compare
- [x] `Memis0` - Check if memory is zero
- [x] `Strlen` - String length
- [ ] `Strcmp` - String comparison

### Memory allocation
- [ ] `StackAlloc` - Stack memory allocation
- [ ] `StackFree` - Stack memory deallocation
- [x] `Heap_Alloc` - Heap memory allocation
- [x] `Heap_Free` - Heap memory deallocation

### Hash functions
- [ ] `HashCreateSha256` - Create SHA256 hash object
- [ ] `HashCreateBlake2b` - Create Blake2b hash object
- [ ] `HashCreateKeccak` - Create Keccak hash object
- [ ] `HashWrite` - Write data to hash object
- [ ] `HashGetValue` - Get hash value
- [ ] `HashFree` - Free hash object
- [ ] `HashClone` - Clone hash object
- [ ] `VerifyBeamHashIII` - Verify Beam hash III

### secp256k1 (elliptic curve cryptography) functions

#### Scalar functions
- [ ] `Secp_Scalar_alloc` - Allocate secp scalar
- [ ] `Secp_Scalar_free` - Free secp scalar
- [ ] `Secp_Scalar_import` - Import secp scalar
- [ ] `Secp_Scalar_export` - Export secp scalar
- [ ] `Secp_Scalar_neg` - Negate secp scalar
- [ ] `Secp_Scalar_add` - Add secp scalars
- [ ] `Secp_Scalar_mul` - Multiply secp scalars
- [ ] `Secp_Scalar_inv` - Invert secp scalar
- [ ] `Secp_Scalar_set` - Set secp scalar value

#### Point functions
- [ ] `Secp_Point_alloc` - Allocate secp point
- [ ] `Secp_Point_free` - Free secp point
- [x] `Secp_Point_Import` - Import secp point
- [x] `Secp_Point_Export` - Export secp point
- [ ] `Secp_Point_neg` - Negate secp point
- [x] `Secp_Point_add` - Add secp points
- [ ] `Secp_Point_mul` - Multiply secp point by scalar
- [ ] `Secp_Point_IsZero` - Check if secp point is zero
- [ ] `Secp_Point_mul_G` - Multiply by generator G
- [ ] `Secp_Point_mul_J` - Multiply by generator J
- [ ] `Secp_Point_mul_H` - Multiply by generator H

## Contract-only functions

### General
- [x] `LoadVar` - Load variable
- [ ] `LoadVarEx` - Extended variable loading
- [x] `SaveVar` - Save variable
- [x] `EmitLog` - Emit log entry
- [ ] `UpdateShader` - Update shader

### Cross-contract
- [ ] `CallFar` - Cross-contract call
- [ ] `get_CallDepth` - Get call depth
- [ ] `get_CallerCid` - Get caller contract ID
- [ ] `RefAdd` - Add reference
- [ ] `RefRelease` - Release reference

### Funds and signature management
- [x] `AddSig` - Add signature
- [x] `FundsLock` - Lock funds
- [x] `FundsUnlock` - Unlock funds

### Asset management
- [ ] `AssetCreate` - Create asset
- [ ] `AssetEmit` - Emit asset
- [ ] `AssetDestroy` - Destroy asset

## Application-only functions

### Variable management
- [x] `Vars_Enum` - Enumerate variables
- [x] `Vars_MoveNext` - Move to next variable
- [x] `Vars_Close` - Close variable enumeration
- [x] `VarGetProof` - Get variable proof

### Log management
- [x] `Logs_Enum` - Enumerate logs
- [x] `Logs_MoveNext` - Move to next log
- [x] `Logs_Close` - Close log enumeration
- [ ] `LogGetProof` - Get log proof

### Key management
- [x] `DerivePk` - Derive public key
- [x] `get_Pk` - Get public key

### Application JSON response
- [x] `DocAddGroup` - Add document group
- [x] `DocCloseGroup` - Close document group
- [x] `DocAddText` - Add text to document
- [x] `DocAddNum32` - Add 32-bit number to document
- [x] `DocAddNum64` - Add 64-bit number to document
- [x] `DocAddArray` - Add array to document
- [x] `DocCloseArray` - Close document array
- [x] `DocAddBlob` - Add blob to document

### Application parameters
- [x] `DocGetText` - Get text from document
- [x] `DocGetNum32` - Get 32-bit number from document
- [x] `DocGetNum64` - Get 64-bit number from document
- [x] `DocGetBlob` - Get blob from document

### Transaction management
- [ ] `SelectContext` - Select context
- [x] `GenerateKernel` - Generate kernel
- [ ] `GenerateKernelAdvanced` - Advanced kernel generation
- [ ] `GenerateRandom` - Generate random data

### Nonce management
- [ ] `SlotInit` - Initialize slot
- [ ] `get_SlotImage` - Get slot image
- [ ] `get_SlotImageEx` - Get extended slot image
- [ ] `get_BlindSk` - Get blind secret key
- [ ] `get_PkEx` - Get extended public key

### Communication
- [ ] `Comm_Listen` - Communication listen
- [ ] `Comm_Send` - Communication send
- [ ] `Comm_Read` - Communication read
- [ ] `Comm_WaitMsg` - Wait for communication message

## Overall Implementation Statistics

### Functions
- **Total Functions**: 87
- **Implemented**: 40
- **Missing**: 47

### Types and Structs
- **Total Types**: 15
- **Implemented**: 12
- **Missing**: 3

### Combined Statistics
- **Total BVM API Items**: 102
- **Implemented**: 52
- **Missing**: 50

### Functions by Category:
- **Common Functions**: 12/40 implemented
- **Contract-only Functions**: 6/15 implemented
- **Application-only Functions**: 22/32 implemented

## Notes

- Functions marked with [x] are implemented in the Rust bindings
- Functions marked with [ ] are missing from the Rust bindings
- This list is based on the BVM functions documentation in the shader-sdk wiki
