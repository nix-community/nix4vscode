# \RegistryApiApi

All URIs are relative to *https://open-vsx.org*

| Method                                                                   | HTTP request                                                              | Description                                                                                              |
| ------------------------------------------------------------------------ | ------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| [**create_namespace1**](RegistryApiApi.md#create_namespace1)             | **POST** /api/-/namespace/create                                          | Create a namespace                                                                                       |
| [**get_extension**](RegistryApiApi.md#get_extension)                     | **GET** /api/{namespace}/{extension}                                      | Provides metadata of the latest version of an extension                                                  |
| [**get_extension1**](RegistryApiApi.md#get_extension1)                   | **GET** /api/{namespace}/{extension}/{version}                            | Provides metadata of a specific version of an extension                                                  |
| [**get_extension2**](RegistryApiApi.md#get_extension2)                   | **GET** /api/{namespace}/{extension}/{targetPlatform}                     | Provides metadata of the latest version of an extension                                                  |
| [**get_extension3**](RegistryApiApi.md#get_extension3)                   | **GET** /api/{namespace}/{extension}/{targetPlatform}/{version}           | Provides metadata of a specific version of an extension                                                  |
| [**get_file**](RegistryApiApi.md#get_file)                               | **GET** /api/{namespace}/{extension}/{version}/file/\*\*                  | Access a file packaged by an extension                                                                   |
| [**get_file1**](RegistryApiApi.md#get_file1)                             | **GET** /api/{namespace}/{extension}/{targetPlatform}/{version}/file/\*\* | Access a file packaged by an extension                                                                   |
| [**get_namespace**](RegistryApiApi.md#get_namespace)                     | **GET** /api/{namespace}                                                  | Provides metadata of a namespace                                                                         |
| [**get_namespace_details**](RegistryApiApi.md#get_namespace_details)     | **GET** /api/{namespace}/details                                          |
| [**get_namespace_logo**](RegistryApiApi.md#get_namespace_logo)           | **GET** /api/{namespace}/logo/{fileName}                                  | Provides logo of a namespace                                                                             |
| [**get_public_key**](RegistryApiApi.md#get_public_key)                   | **GET** /api/-/public-key/{publicId}                                      | Access a public key file                                                                                 |
| [**get_query**](RegistryApiApi.md#get_query)                             | **GET** /api/-/query                                                      | Provides metadata of extensions matching the given parameters                                            |
| [**get_query_v2**](RegistryApiApi.md#get_query_v2)                       | **GET** /api/v2/-/query                                                   | Provides metadata of extensions matching the given parameters                                            |
| [**get_reviews**](RegistryApiApi.md#get_reviews)                         | **GET** /api/{namespace}/{extension}/reviews                              | Returns the list of reviews of an extension                                                              |
| [**get_version_references**](RegistryApiApi.md#get_version_references)   | **GET** /api/{namespace}/{extension}/{targetPlatform}/version-references  | Provides a list of version references matching an extension                                              |
| [**get_version_references1**](RegistryApiApi.md#get_version_references1) | **GET** /api/{namespace}/{extension}/version-references                   | Provides a list of version references matching an extension                                              |
| [**get_versions**](RegistryApiApi.md#get_versions)                       | **GET** /api/{namespace}/{extension}/{targetPlatform}/versions            | Provides a map of versions matching an extension                                                         |
| [**get_versions1**](RegistryApiApi.md#get_versions1)                     | **GET** /api/{namespace}/{extension}/versions                             | Provides a map of versions matching an extension                                                         |
| [**post_query**](RegistryApiApi.md#post_query)                           | **POST** /api/-/query                                                     | Provides metadata of extensions matching the given parameters. Deprecated: use GET /api/-/query instead. |
| [**publish**](RegistryApiApi.md#publish)                                 | **POST** /api/user/publish                                                | Publish an extension by uploading a vsix file                                                            |
| [**publish1**](RegistryApiApi.md#publish1)                               | **POST** /api/-/publish                                                   | Publish an extension by uploading a vsix file                                                            |
| [**search**](RegistryApiApi.md#search)                                   | **GET** /api/-/search                                                     | Search extensions via text entered by a user                                                             |
| [**verify_token**](RegistryApiApi.md#verify_token)                       | **GET** /api/{namespace}/verify-pat                                       | Check if a personal access token is valid and is allowed to publish in a namespace                       |

## create_namespace1

> create_namespace1(token, namespace)
> Create a namespace

### Parameters

| Name          | Type                          | Description             | Required   | Notes |
| ------------- | ----------------------------- | ----------------------- | ---------- | ----- |
| **token**     | **String**                    | A personal access token | [required] |
| **namespace** | [**Namespace**](Namespace.md) |                         | [required] |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: application/json
-   **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_extension

> crate::models::Extension get_extension(namespace, extension)
> Provides metadata of the latest version of an extension

### Parameters

| Name          | Type       | Description         | Required   | Notes |
| ------------- | ---------- | ------------------- | ---------- | ----- |
| **namespace** | **String** | Extension namespace | [required] |
| **extension** | **String** | Extension name      | [required] |

### Return type

[**crate::models::Extension**](Extension.md)

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: Not defined
-   **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_extension1

> crate::models::Extension get_extension1(namespace, extension, version)
> Provides metadata of a specific version of an extension

### Parameters

| Name          | Type       | Description         | Required   | Notes |
| ------------- | ---------- | ------------------- | ---------- | ----- |
| **namespace** | **String** | Extension namespace | [required] |
| **extension** | **String** | Extension name      | [required] |
| **version**   | **String** | Extension version   | [required] |

### Return type

[**crate::models::Extension**](Extension.md)

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: Not defined
-   **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_extension2

> crate::models::Extension get_extension2(namespace, extension, target_platform)
> Provides metadata of the latest version of an extension

### Parameters

| Name                | Type       | Description         | Required   | Notes |
| ------------------- | ---------- | ------------------- | ---------- | ----- |
| **namespace**       | **String** | Extension namespace | [required] |
| **extension**       | **String** | Extension name      | [required] |
| **target_platform** | **String** | Target platform     | [required] |

### Return type

[**crate::models::Extension**](Extension.md)

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: Not defined
-   **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_extension3

> crate::models::Extension get_extension3(namespace, extension, target_platform, version)
> Provides metadata of a specific version of an extension

### Parameters

| Name                | Type       | Description         | Required   | Notes |
| ------------------- | ---------- | ------------------- | ---------- | ----- |
| **namespace**       | **String** | Extension namespace | [required] |
| **extension**       | **String** | Extension name      | [required] |
| **target_platform** | **String** | Target platform     | [required] |
| **version**         | **String** | Extension version   | [required] |

### Return type

[**crate::models::Extension**](Extension.md)

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: Not defined
-   **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_file

> Vec<String> get_file(namespace, extension, version)
> Access a file packaged by an extension

### Parameters

| Name          | Type       | Description         | Required   | Notes |
| ------------- | ---------- | ------------------- | ---------- | ----- |
| **namespace** | **String** | Extension namespace | [required] |
| **extension** | **String** | Extension name      | [required] |
| **version**   | **String** | Extension version   | [required] |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: Not defined
-   **Accept**: _/_

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_file1

> Vec<String> get_file1(namespace, extension, target_platform, version)
> Access a file packaged by an extension

### Parameters

| Name                | Type       | Description         | Required   | Notes |
| ------------------- | ---------- | ------------------- | ---------- | ----- |
| **namespace**       | **String** | Extension namespace | [required] |
| **extension**       | **String** | Extension name      | [required] |
| **target_platform** | **String** | Target platform     | [required] |
| **version**         | **String** | Extension version   | [required] |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: Not defined
-   **Accept**: _/_

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_namespace

> crate::models::Namespace get_namespace(namespace)
> Provides metadata of a namespace

### Parameters

| Name          | Type       | Description    | Required   | Notes |
| ------------- | ---------- | -------------- | ---------- | ----- |
| **namespace** | **String** | Namespace name | [required] |

### Return type

[**crate::models::Namespace**](Namespace.md)

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: Not defined
-   **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_namespace_details

> crate::models::NamespaceDetails get_namespace_details(namespace)

### Parameters

| Name          | Type       | Description    | Required   | Notes |
| ------------- | ---------- | -------------- | ---------- | ----- |
| **namespace** | **String** | Namespace name | [required] |

### Return type

[**crate::models::NamespaceDetails**](NamespaceDetails.md)

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: Not defined
-   **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_namespace_logo

> Vec<String> get_namespace_logo(namespace, file_name)
> Provides logo of a namespace

### Parameters

| Name          | Type       | Description    | Required   | Notes |
| ------------- | ---------- | -------------- | ---------- | ----- |
| **namespace** | **String** | Namespace name | [required] |
| **file_name** | **String** | Logo file name | [required] |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: Not defined
-   **Accept**: image/jpeg, image/png

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_public_key

> String get_public_key(public_id)
> Access a public key file

### Parameters

| Name          | Type       | Description                    | Required   | Notes |
| ------------- | ---------- | ------------------------------ | ---------- | ----- |
| **public_id** | **String** | Public ID of a public key file | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: Not defined
-   **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_query

> crate::models::QueryResult get_query(namespace_name, extension_name, extension_version, extension_id, extension_uuid, namespace_uuid, include_all_versions, target_platform, size, offset)
> Provides metadata of extensions matching the given parameters

### Parameters

| Name                     | Type               | Description                                                                               | Required | Notes            |
| ------------------------ | ------------------ | ----------------------------------------------------------------------------------------- | -------- | ---------------- |
| **namespace_name**       | Option<**String**> | Name of a namespace                                                                       |          |
| **extension_name**       | Option<**String**> | Name of an extension                                                                      |          |
| **extension_version**    | Option<**String**> | Version of an extension                                                                   |          |
| **extension_id**         | Option<**String**> | Identifier in the form {namespace}.{extension}                                            |          |
| **extension_uuid**       | Option<**String**> | Universally unique identifier of an extension                                             |          |
| **namespace_uuid**       | Option<**String**> | Universally unique identifier of a namespace                                              |          |
| **include_all_versions** | Option<**bool**>   | Whether to include all versions of an extension, ignored if extensionVersion is specified |          |
| **target_platform**      | Option<**String**> | Target platform                                                                           |          |
| **size**                 | Option<**i32**>    | Maximal number of entries to return                                                       |          | [default to 100] |
| **offset**               | Option<**i32**>    | Number of entries to skip (usually a multiple of the page size)                           |          | [default to 0]   |

### Return type

[**crate::models::QueryResult**](QueryResult.md)

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: Not defined
-   **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_query_v2

> crate::models::QueryResult get_query_v2(namespace_name, extension_name, extension_version, extension_id, extension_uuid, namespace_uuid, include_all_versions, target_platform, size, offset)
> Provides metadata of extensions matching the given parameters

### Parameters

| Name                     | Type               | Description                                                     | Required | Notes              |
| ------------------------ | ------------------ | --------------------------------------------------------------- | -------- | ------------------ |
| **namespace_name**       | Option<**String**> | Name of a namespace                                             |          |
| **extension_name**       | Option<**String**> | Name of an extension                                            |          |
| **extension_version**    | Option<**String**> | Version of an extension                                         |          |
| **extension_id**         | Option<**String**> | Identifier in the form {namespace}.{extension}                  |          |
| **extension_uuid**       | Option<**String**> | Universally unique identifier of an extension                   |          |
| **namespace_uuid**       | Option<**String**> | Universally unique identifier of a namespace                    |          |
| **include_all_versions** | Option<**String**> | Whether to include all versions of an extension                 |          | [default to links] |
| **target_platform**      | Option<**String**> | Target platform                                                 |          |
| **size**                 | Option<**i32**>    | Maximal number of entries to return                             |          | [default to 100]   |
| **offset**               | Option<**i32**>    | Number of entries to skip (usually a multiple of the page size) |          | [default to 0]     |

### Return type

[**crate::models::QueryResult**](QueryResult.md)

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: Not defined
-   **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_reviews

> crate::models::ReviewList get_reviews(namespace, extension)
> Returns the list of reviews of an extension

### Parameters

| Name          | Type       | Description         | Required   | Notes |
| ------------- | ---------- | ------------------- | ---------- | ----- |
| **namespace** | **String** | Extension namespace | [required] |
| **extension** | **String** | Extension name      | [required] |

### Return type

[**crate::models::ReviewList**](ReviewList.md)

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: Not defined
-   **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_version_references

> crate::models::VersionReferences get_version_references(namespace, extension, target_platform, size, offset)
> Provides a list of version references matching an extension

### Parameters

| Name                | Type            | Description                                                     | Required   | Notes           |
| ------------------- | --------------- | --------------------------------------------------------------- | ---------- | --------------- |
| **namespace**       | **String**      | Extension namespace                                             | [required] |
| **extension**       | **String**      | Extension name                                                  | [required] |
| **target_platform** | **String**      | Target platform                                                 | [required] |
| **size**            | Option<**i32**> | Maximal number of entries to return                             |            | [default to 18] |
| **offset**          | Option<**i32**> | Number of entries to skip (usually a multiple of the page size) |            | [default to 0]  |

### Return type

[**crate::models::VersionReferences**](VersionReferences.md)

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: Not defined
-   **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_version_references1

> crate::models::VersionReferences get_version_references1(namespace, extension, size, offset)
> Provides a list of version references matching an extension

### Parameters

| Name          | Type            | Description                                                     | Required   | Notes           |
| ------------- | --------------- | --------------------------------------------------------------- | ---------- | --------------- |
| **namespace** | **String**      | Extension namespace                                             | [required] |
| **extension** | **String**      | Extension name                                                  | [required] |
| **size**      | Option<**i32**> | Maximal number of entries to return                             |            | [default to 18] |
| **offset**    | Option<**i32**> | Number of entries to skip (usually a multiple of the page size) |            | [default to 0]  |

### Return type

[**crate::models::VersionReferences**](VersionReferences.md)

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: Not defined
-   **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_versions

> crate::models::Versions get_versions(namespace, extension, target_platform, size, offset)
> Provides a map of versions matching an extension

### Parameters

| Name                | Type            | Description                                                     | Required   | Notes           |
| ------------------- | --------------- | --------------------------------------------------------------- | ---------- | --------------- |
| **namespace**       | **String**      | Extension namespace                                             | [required] |
| **extension**       | **String**      | Extension name                                                  | [required] |
| **target_platform** | **String**      | Target platform                                                 | [required] |
| **size**            | Option<**i32**> | Maximal number of entries to return                             |            | [default to 18] |
| **offset**          | Option<**i32**> | Number of entries to skip (usually a multiple of the page size) |            | [default to 0]  |

### Return type

[**crate::models::Versions**](Versions.md)

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: Not defined
-   **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_versions1

> crate::models::Versions get_versions1(namespace, extension, size, offset)
> Provides a map of versions matching an extension

### Parameters

| Name          | Type            | Description                                                     | Required   | Notes           |
| ------------- | --------------- | --------------------------------------------------------------- | ---------- | --------------- |
| **namespace** | **String**      | Extension namespace                                             | [required] |
| **extension** | **String**      | Extension name                                                  | [required] |
| **size**      | Option<**i32**> | Maximal number of entries to return                             |            | [default to 18] |
| **offset**    | Option<**i32**> | Number of entries to skip (usually a multiple of the page size) |            | [default to 0]  |

### Return type

[**crate::models::Versions**](Versions.md)

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: Not defined
-   **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## post_query

> post_query(query_param)
> Provides metadata of extensions matching the given parameters. Deprecated: use GET /api/-/query instead.

### Parameters

| Name            | Type                            | Description | Required   | Notes |
| --------------- | ------------------------------- | ----------- | ---------- | ----- |
| **query_param** | [**QueryParam**](QueryParam.md) |             | [required] |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: application/json
-   **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## publish

> crate::models::Extension publish(body)
> Publish an extension by uploading a vsix file

### Parameters

| Name     | Type                   | Description                   | Required   | Notes |
| -------- | ---------------------- | ----------------------------- | ---------- | ----- |
| **body** | **std::path::PathBuf** | Uploaded vsix file to publish | [required] |

### Return type

[**crate::models::Extension**](Extension.md)

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: application/octet-stream
-   **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## publish1

> crate::models::Extension publish1(token, body)
> Publish an extension by uploading a vsix file

### Parameters

| Name      | Type                   | Description                   | Required   | Notes |
| --------- | ---------------------- | ----------------------------- | ---------- | ----- |
| **token** | **String**             | A personal access token       | [required] |
| **body**  | **std::path::PathBuf** | Uploaded vsix file to publish | [required] |

### Return type

[**crate::models::Extension**](Extension.md)

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: application/octet-stream
-   **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## search

> crate::models::SearchResult search(query, category, target_platform, size, offset, sort_order, sort_by, include_all_versions)
> Search extensions via text entered by a user

### Parameters

| Name                     | Type               | Description                                                                      | Required | Notes           |
| ------------------------ | ------------------ | -------------------------------------------------------------------------------- | -------- | --------------- |
| **query**                | Option<**String**> | Query text for searching                                                         |          |
| **category**             | Option<**String**> | Extension category as shown in the UI                                            |          |
| **target_platform**      | Option<**String**> | Target platform                                                                  |          |
| **size**                 | Option<**i32**>    | Maximal number of entries to return                                              |          | [default to 18] |
| **offset**               | Option<**i32**>    | Number of entries to skip (usually a multiple of the page size)                  |          | [default to 0]  |
| **sort_order**           | Option<**String**> | Descending or ascending sort order                                               |          |
| **sort_by**              | Option<**String**> | Sort key (relevance is a weighted mix of various properties)                     |          |
| **include_all_versions** | Option<**bool**>   | Whether to include information on all available versions for each returned entry |          |

### Return type

[**crate::models::SearchResult**](SearchResult.md)

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: Not defined
-   **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## verify_token

> crate::models::VerifyToken200Response verify_token(namespace, token)
> Check if a personal access token is valid and is allowed to publish in a namespace

### Parameters

| Name          | Type       | Description             | Required   | Notes |
| ------------- | ---------- | ----------------------- | ---------- | ----- |
| **namespace** | **String** | Namespace               | [required] |
| **token**     | **String** | A personal access token | [required] |

### Return type

[**crate::models::VerifyToken200Response**](verifyToken_200_response.md)

### Authorization

No authorization required

### HTTP request headers

-   **Content-Type**: Not defined
-   **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
