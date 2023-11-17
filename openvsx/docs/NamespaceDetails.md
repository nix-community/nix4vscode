# NamespaceDetails

## Properties

| Name             | Type                                                          | Description                                                                             | Notes      |
| ---------------- | ------------------------------------------------------------- | --------------------------------------------------------------------------------------- | ---------- |
| **success**      | Option<**String**>                                            | Indicates success of the operation (omitted if a more specific result type is returned) | [optional] |
| **warning**      | Option<**String**>                                            | Indicates a warning; when this is present, other properties can still be used           | [optional] |
| **error**        | Option<**String**>                                            | Indicates an error; when this is present, all other properties should be ignored        | [optional] |
| **name**         | **String**                                                    | Name of the namespace                                                                   |
| **display_name** | Option<**String**>                                            | Display name of the namespace                                                           | [optional] |
| **description**  | Option<**String**>                                            | Description of the namespace                                                            | [optional] |
| **logo**         | Option<**String**>                                            | Logo URL of the namespace                                                               | [optional] |
| **website**      | Option<**String**>                                            | Website URL of the namespace                                                            | [optional] |
| **support_link** | Option<**String**>                                            | Support URL of the namespace                                                            | [optional] |
| **social_links** | Option<**::std::collections::HashMap<String, String>**>       | Map of social network names to their profile URLs                                       | [optional] |
| **extensions**   | Option<[**Vec<crate::models::SearchEntry>**](SearchEntry.md)> | Map of extension names to their metadata URLs                                           | [optional] |
| **verified**     | **bool**                                                      | Indicates whether the namespace has an owner                                            |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
