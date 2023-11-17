# VersionReferences

## Properties

| Name           | Type                                                            | Description                                                                                                   | Notes      |
| -------------- | --------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- | ---------- |
| **success**    | Option<**String**>                                              | Indicates success of the operation (omitted if a more specific result type is returned)                       | [optional] |
| **warning**    | Option<**String**>                                              | Indicates a warning; when this is present, other properties can still be used                                 | [optional] |
| **error**      | Option<**String**>                                              | Indicates an error; when this is present, all other properties should be ignored                              | [optional] |
| **offset**     | **i32**                                                         | Number of skipped entries according to the version references request                                         |
| **total_size** | **i32**                                                         | Total number of version references the extension has                                                          |
| **versions**   | [**Vec<crate::models::VersionReference>**](VersionReference.md) | Essential metadata of all available versions, limited to the size specified in the version references request |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
