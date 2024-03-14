# PutMultipleIndividualsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ticket** | **String** | The unique identifier of the user's ticket. | 
**individuals** | [**Vec<serde_json::Value>**](serde_json::Value.md) | An array of individual objects to be updated or inserted. | 
**prepare_events** | Option<**bool**> | Optional flag to prepare events. | [optional]
**assigned_subsystems** | Option<**i32**> | Optional byte value for assigned subsystems. | [optional]
**event_id** | Option<**String**> | Optional event identifier. | [optional]
**transaction_id** | Option<**String**> | Optional transaction identifier. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


