# \DefaultApi

All URIs are relative to *http://example.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_field_to_individual**](DefaultApi.md#add_field_to_individual) | **PUT** /add_to_individual | Add Field into Individual
[**authenticate_get**](DefaultApi.md#authenticate_get) | **GET** /authenticate | Authenticate User
[**check_ticket_validity**](DefaultApi.md#check_ticket_validity) | **GET** /is_ticket_valid | Check Ticket Validity
[**execute_full_text_query**](DefaultApi.md#execute_full_text_query) | **POST** /query | Execute Full Text Query
[**get_individual**](DefaultApi.md#get_individual) | **GET** /get_individual | Get One Individual
[**get_membership**](DefaultApi.md#get_membership) | **GET** /get_membership | Get Membership of URI
[**get_multiple_individuals**](DefaultApi.md#get_multiple_individuals) | **POST** /get_individuals | Get More Individuals
[**get_operation_state**](DefaultApi.md#get_operation_state) | **GET** /get_operation_state | Get Operation State
[**get_rights_origin**](DefaultApi.md#get_rights_origin) | **GET** /get_rights_origin | Get Origin of Access Rights on URI
[**get_ticket_for_user**](DefaultApi.md#get_ticket_for_user) | **GET** /get_ticket_trusted | Get Ticket for Another User
[**put_individual**](DefaultApi.md#put_individual) | **PUT** /put_individual | Put One Individual
[**put_multiple_individuals**](DefaultApi.md#put_multiple_individuals) | **PUT** /put_individuals | Put More Individuals
[**remove_field_from_individual**](DefaultApi.md#remove_field_from_individual) | **PUT** /remove_from_individual | Remove Field from Individual
[**remove_individual**](DefaultApi.md#remove_individual) | **PUT** /remove_individual | Remove One Individual
[**set_field_in_individual**](DefaultApi.md#set_field_in_individual) | **PUT** /set_in_individual | Set Field into Individual



## add_field_to_individual

> models::AddFieldToIndividual200Response add_field_to_individual(add_field_to_individual_request)
Add Field into Individual

This endpoint adds a specific field to the information of an individual. It requires a user ticket and the individual's URI, along with the details of the field to be added, and supports optional parameters for event handling and transaction control. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_field_to_individual_request** | [**AddFieldToIndividualRequest**](AddFieldToIndividualRequest.md) |  | [required] |

### Return type

[**models::AddFieldToIndividual200Response**](addFieldToIndividual_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## authenticate_get

> models::AuthenticateGet200Response authenticate_get(login, password, secret)
Authenticate User

Authenticates a user with login and password, and optionally a secret.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login** | **String** | User's login name. | [required] |
**password** | **String** | User's password in hashed format. | [required] |
**secret** | Option<**String**> | Optional secret key for additional security. |  |

### Return type

[**models::AuthenticateGet200Response**](_authenticate_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_ticket_validity

> bool check_ticket_validity(ticket)
Check Ticket Validity

This endpoint checks if the provided ticket is valid. The ticket is passed as a query parameter. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket** | **String** | The unique identifier of the ticket. | [required] |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## execute_full_text_query

> models::ExecuteFullTextQuery200Response execute_full_text_query(execute_full_text_query_request)
Execute Full Text Query

This endpoint allows users to execute a full text query. It requires a ticket  and a query string, and supports various optional parameters to refine the search. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**execute_full_text_query_request** | [**ExecuteFullTextQueryRequest**](ExecuteFullTextQueryRequest.md) |  | [required] |

### Return type

[**models::ExecuteFullTextQuery200Response**](executeFullTextQuery_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_individual

> serde_json::Value get_individual(ticket, uri, reopen)
Get One Individual

This endpoint retrieves information about a specific individual based  on their unique identifier (URI). It requires a user ticket and the  individual's URI, and optionally allows reopening the query. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket** | **String** | The unique identifier of the user's ticket. | [required] |
**uri** | **String** | The unique identifier (URI) of the individual. | [required] |
**reopen** | Option<**bool**> | Optional flag to reopen the query. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_membership

> models::GetMembership200Response get_membership(ticket, uri)
Get Membership of URI

This endpoint retrieves membership information of a specific URI. It requires a user ticket and the URI for which the membership details are being queried. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket** | **String** | The unique identifier of the user's ticket. | [required] |
**uri** | **String** | The unique identifier (URI) for which the membership details are being queried. | [required] |

### Return type

[**models::GetMembership200Response**](getMembership_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_multiple_individuals

> Vec<serde_json::Value> get_multiple_individuals(get_multiple_individuals_request)
Get More Individuals

This endpoint retrieves information about multiple individuals based on their  unique identifiers (URIs). It requires a user ticket and an array of URIs,  and optionally allows reopening the query. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_multiple_individuals_request** | [**GetMultipleIndividualsRequest**](GetMultipleIndividualsRequest.md) |  | [required] |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_operation_state

> i64 get_operation_state(module_id, wait_op_id)
Get Operation State

This endpoint retrieves the state of a specified operation. It requires the module ID and the operation ID for which the state is being queried. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**module_id** | **i32** | The module ID associated with the operation. | [required] |
**wait_op_id** | **i64** | The operation ID for which the state is being queried. | [required] |

### Return type

**i64**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rights_origin

> Vec<serde_json::Value> get_rights_origin(ticket, uri)
Get Origin of Access Rights on URI

This endpoint retrieves information about the origin of access rights for a specific URI. It requires a user ticket and the URI for which the rights origins are being queried. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket** | **String** | The unique identifier of the user's ticket. | [required] |
**uri** | **String** | The unique identifier (URI) for which the rights origins are being queried. | [required] |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ticket_for_user

> models::GetTicketForUser200Response get_ticket_for_user(ticket, login)
Get Ticket for Another User

This endpoint retrieves a ticket for another user. It requires both  the ticket of the requesting user and the login of the user for whom  the ticket is being requested. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket** | **String** | The unique identifier of the requesting user's ticket. | [required] |
**login** | **String** | The login identifier of the user for whom the ticket is being requested. | [required] |

### Return type

[**models::GetTicketForUser200Response**](getTicketForUser_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_individual

> models::PutIndividual200Response put_individual(ticket, put_individual_request)
Put One Individual

This endpoint updates or inserts information about an individual.  It requires a user ticket and the individual object, with optional  parameters for event handling and transaction control. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket** | **String** | The unique identifier of the user's ticket. | [required] |
**put_individual_request** | [**PutIndividualRequest**](PutIndividualRequest.md) |  | [required] |

### Return type

[**models::PutIndividual200Response**](putIndividual_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_multiple_individuals

> models::PutIndividual200Response put_multiple_individuals(put_multiple_individuals_request)
Put More Individuals

This endpoint updates or inserts information about multiple individuals.  It requires a user ticket and an array of individual objects, with optional  parameters for event handling and transaction control. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**put_multiple_individuals_request** | [**PutMultipleIndividualsRequest**](PutMultipleIndividualsRequest.md) |  | [required] |

### Return type

[**models::PutIndividual200Response**](putIndividual_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_field_from_individual

> models::RemoveIndividual200Response remove_field_from_individual(remove_field_from_individual_request)
Remove Field from Individual

This endpoint removes a specific field from the information of an individual. It requires a user ticket and the individual's URI, along with the details of the field to be removed,  and supports optional parameters for event handling and transaction control. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remove_field_from_individual_request** | [**RemoveFieldFromIndividualRequest**](RemoveFieldFromIndividualRequest.md) |  | [required] |

### Return type

[**models::RemoveIndividual200Response**](removeIndividual_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_individual

> models::RemoveIndividual200Response remove_individual(remove_individual_request)
Remove One Individual

This endpoint removes information about a specific individual.  It requires a user ticket and the URI of the individual, with optional  parameters for event handling and transaction control. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remove_individual_request** | [**RemoveIndividualRequest**](RemoveIndividualRequest.md) |  | [required] |

### Return type

[**models::RemoveIndividual200Response**](removeIndividual_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_field_in_individual

> models::SetFieldInIndividual200Response set_field_in_individual(set_field_in_individual_request)
Set Field into Individual

This endpoint sets or updates a specific field in the information of an individual. It requires a user ticket and the individual's URI, along with the details of the field to be set or updated, and supports optional parameters for event handling and transaction control. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_field_in_individual_request** | [**SetFieldInIndividualRequest**](SetFieldInIndividualRequest.md) |  | [required] |

### Return type

[**models::SetFieldInIndividual200Response**](setFieldInIndividual_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

