# VersionReference

## Properties

| Name                | Type                                                    | Description                                                                                       | Notes      |
| ------------------- | ------------------------------------------------------- | ------------------------------------------------------------------------------------------------- | ---------- |
| **url**             | Option<**String**>                                      | URL to get the full metadata of this version                                                      | [optional] |
| **files**           | Option<**::std::collections::HashMap<String, String>**> | Map of file types (download, manifest, icon, readme, license, changelog) to their respective URLs | [optional] |
| **version**         | Option<**String**>                                      |                                                                                                   | [optional] |
| **target_platform** | Option<**String**>                                      | Name of the target platform                                                                       | [optional] |
| **engines**         | Option<**::std::collections::HashMap<String, String>**> | Map of engine names to the respective version constraints                                         | [optional] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
