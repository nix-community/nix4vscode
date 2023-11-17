# User

## Properties

| Name           | Type               | Description                                                                             | Notes      |
| -------------- | ------------------ | --------------------------------------------------------------------------------------- | ---------- |
| **success**    | Option<**String**> | Indicates success of the operation (omitted if a more specific result type is returned) | [optional] |
| **warning**    | Option<**String**> | Indicates a warning; when this is present, other properties can still be used           | [optional] |
| **error**      | Option<**String**> | Indicates an error; when this is present, all other properties should be ignored        | [optional] |
| **login_name** | **String**         | Login name                                                                              |
| **full_name**  | Option<**String**> | Full name                                                                               | [optional] |
| **avatar_url** | Option<**String**> | URL to the user's avatar image                                                          | [optional] |
| **homepage**   | Option<**String**> | URL to the user's profile page                                                          | [optional] |
| **provider**   | Option<**String**> | Authentication provider (e.g. github)                                                   | [optional] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
