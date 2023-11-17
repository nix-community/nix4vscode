# SearchEntry

## Properties

| Name                 | Type                                                                    | Description                                                                                                               | Notes      |
| -------------------- | ----------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- | ---------- |
| **url**              | **String**                                                              | URL to get the full metadata of the extension                                                                             |
| **files**            | **::std::collections::HashMap<String, String>**                         | Map of file types (download, manifest, icon, readme, license, changelog) to their respective URLs                         |
| **name**             | **String**                                                              | Name of the extension                                                                                                     |
| **namespace**        | **String**                                                              | Namespace of the extension                                                                                                |
| **version**          | **String**                                                              | The latest published version                                                                                              |
| **timestamp**        | **String**                                                              | Date and time when this version was published (ISO-8601)                                                                  |
| **all_versions**     | Option<[**Vec<crate::models::VersionReference>**](VersionReference.md)> | Essential metadata of all available versions. Deprecated: only returns the last 100 versions. Use allVersionsUrl instead. | [optional] |
| **all_versions_url** | Option<**String**>                                                      | URL to get essential metadata of all available versions.                                                                  | [optional] |
| **average_rating**   | Option<**f64**>                                                         | Average rating                                                                                                            | [optional] |
| **review_count**     | Option<**i64**>                                                         | Number of reviews                                                                                                         | [optional] |
| **download_count**   | Option<**i32**>                                                         | Number of downloads of the extension package                                                                              | [optional] |
| **display_name**     | Option<**String**>                                                      | Name to be displayed in user interfaces                                                                                   | [optional] |
| **description**      | Option<**String**>                                                      |                                                                                                                           | [optional] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
