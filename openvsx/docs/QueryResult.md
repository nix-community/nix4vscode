# QueryResult

## Properties

| Name           | Type                                                      | Description                                                                             | Notes      |
| -------------- | --------------------------------------------------------- | --------------------------------------------------------------------------------------- | ---------- |
| **success**    | Option<**String**>                                        | Indicates success of the operation (omitted if a more specific result type is returned) | [optional] |
| **warning**    | Option<**String**>                                        | Indicates a warning; when this is present, other properties can still be used           | [optional] |
| **error**      | Option<**String**>                                        | Indicates an error; when this is present, all other properties should be ignored        | [optional] |
| **offset**     | **i32**                                                   | Number of skipped entries according to the query                                        |
| **total_size** | **i32**                                                   | Total number of entries that match the query                                            |
| **extensions** | Option<[**Vec<crate::models::Extension>**](Extension.md)> | Extensions that match the given query (may be empty)                                    | [optional] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
