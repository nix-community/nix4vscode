# Namespace

## Properties

| Name           | Type                                                    | Description                                                                             | Notes      |
| -------------- | ------------------------------------------------------- | --------------------------------------------------------------------------------------- | ---------- |
| **success**    | Option<**String**>                                      | Indicates success of the operation (omitted if a more specific result type is returned) | [optional] |
| **warning**    | Option<**String**>                                      | Indicates a warning; when this is present, other properties can still be used           | [optional] |
| **error**      | Option<**String**>                                      | Indicates an error; when this is present, all other properties should be ignored        | [optional] |
| **name**       | **String**                                              | Name of the namespace                                                                   |
| **extensions** | Option<**::std::collections::HashMap<String, String>**> | Map of extension names to their metadata URLs (not required for creating)               | [optional] |
| **verified**   | **bool**                                                | Indicates whether the namespace has an owner (not required for creating)                |
| **access**     | Option<**String**>                                      | Access level of the namespace. Deprecated: namespaces are now always restricted         | [optional] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
