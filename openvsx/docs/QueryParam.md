# QueryParam

## Properties

| Name                     | Type               | Description                                                                               | Notes                      |
| ------------------------ | ------------------ | ----------------------------------------------------------------------------------------- | -------------------------- |
| **namespace_name**       | Option<**String**> | Name of a namespace                                                                       | [optional]                 |
| **extension_name**       | Option<**String**> | Name of an extension                                                                      | [optional]                 |
| **extension_version**    | Option<**String**> | Version of an extension                                                                   | [optional]                 |
| **extension_id**         | Option<**String**> | Identifier in the form {namespace}.{extension}                                            | [optional]                 |
| **extension_uuid**       | Option<**String**> | Universally unique identifier of an extension                                             | [optional]                 |
| **namespace_uuid**       | Option<**String**> | Universally unique identifier of a namespace                                              | [optional]                 |
| **include_all_versions** | Option<**bool**>   | Whether to include all versions of an extension, ignored if extensionVersion is specified | [optional]                 |
| **target_platform**      | Option<**String**> | Name of the target platform                                                               | [optional]                 |
| **size**                 | Option<**i32**>    | Maximal number of entries to return                                                       | [optional][default to 100] |
| **offset**               | Option<**i32**>    | Number of entries to skip (usually a multiple of the page size)                           | [optional][default to 0]   |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
