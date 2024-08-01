use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-search=libsrc/lib");
    println!("cargo:rustc-link-lib=static=SimConnect");

    let bindings = bindgen::Builder::default()
        .header("libsrc/include/SimConnect.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_var("SIMCONNECT_UNUSED")
        .allowlist_var("SIMCONNECT_OBJECT_ID_USER")
        .allowlist_var("SIMCONNECT_CAMERA_IGNORE_FIELD")
        .allowlist_var("SIMCONNECT_CLIENTDATA_MAX_SIZE")
        .allowlist_var("SIMCONNECT_GROUP_PRIORITY_HIGHEST")
        .allowlist_var("SIMCONNECT_GROUP_PRIORITY_HIGHEST_MASKABLE")
        .allowlist_var("SIMCONNECT_GROUP_PRIORITY_STANDARD")
        .allowlist_var("SIMCONNECT_GROUP_PRIORITY_DEFAULT")
        .allowlist_var("SIMCONNECT_GROUP_PRIORITY_LOWEST")
        .allowlist_var("MAX_METAR_LENGTH")
        .allowlist_var("MAX_THERMAL_SIZE")
        .allowlist_var("MAX_THERMAL_RATE")
        .allowlist_var("INITPOSITION_AIRSPEED_CRUISE")
        .allowlist_var("INITPOSITION_AIRSPEED_KEEP")
        .allowlist_var("SIMCONNECT_CLIENTDATATYPE_INT8")
        .allowlist_var("SIMCONNECT_CLIENTDATATYPE_INT16")
        .allowlist_var("SIMCONNECT_CLIENTDATATYPE_INT32")
        .allowlist_var("SIMCONNECT_CLIENTDATATYPE_INT64")
        .allowlist_var("SIMCONNECT_CLIENTDATATYPE_FLOAT32")
        .allowlist_var("SIMCONNECT_CLIENTDATATYPE_FLOAT64")
        .allowlist_var("SIMCONNECT_CLIENTDATAOFFSET_AUTO")
        .allowlist_var("SIMCONNECT_OPEN_CONFIGINDEX_LOCAL")
        .allowlist_var("SIMCONNECT_RECV_ID_VOR_LIST_HAS_NAV_SIGNAL")
        .allowlist_var("SIMCONNECT_RECV_ID_VOR_LIST_HAS_LOCALIZER")
        .allowlist_var("SIMCONNECT_RECV_ID_VOR_LIST_HAS_GLIDE_SLOPE")
        .allowlist_var("SIMCONNECT_RECV_ID_VOR_LIST_HAS_DME")
        .allowlist_var("SIMCONNECT_WAYPOINT_NONE")
        .allowlist_var("SIMCONNECT_WAYPOINT_SPEED_REQUESTED")
        .allowlist_var("SIMCONNECT_WAYPOINT_THROTTLE_REQUESTED")
        .allowlist_var("SIMCONNECT_WAYPOINT_COMPUTE_VERTICAL_SPEED")
        .allowlist_var("SIMCONNECT_WAYPOINT_ALTITUDE_IS_AGL")
        .allowlist_var("SIMCONNECT_WAYPOINT_ON_GROUND")
        .allowlist_var("SIMCONNECT_WAYPOINT_REVERSE")
        .allowlist_var("SIMCONNECT_WAYPOINT_WRAP_TO_FIRST")
        .allowlist_var("SIMCONNECT_EVENT_FLAG_DEFAULT")
        .allowlist_var("SIMCONNECT_EVENT_FLAG_FAST_REPEAT_TIMER")
        .allowlist_var("SIMCONNECT_EVENT_FLAG_SLOW_REPEAT_TIMER")
        .allowlist_var("SIMCONNECT_EVENT_FLAG_GROUPID_IS_PRIORITY")
        .allowlist_var("SIMCONNECT_DATA_REQUEST_FLAG_DEFAULT")
        .allowlist_var("SIMCONNECT_DATA_REQUEST_FLAG_CHANGED")
        .allowlist_var("SIMCONNECT_DATA_REQUEST_FLAG_TAGGED")
        .allowlist_var("SIMCONNECT_DATA_SET_FLAG_DEFAULT")
        .allowlist_var("SIMCONNECT_DATA_SET_FLAG_TAGGED")
        .allowlist_var("SIMCONNECT_CREATE_CLIENT_DATA_FLAG_DEFAULT")
        .allowlist_var("SIMCONNECT_CREATE_CLIENT_DATA_FLAG_READ_ONLY")
        .allowlist_var("SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_DEFAULT")
        .allowlist_var("SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_CHANGED")
        .allowlist_var("SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_TAGGED")
        .allowlist_var("SIMCONNECT_CLIENT_DATA_SET_FLAG_DEFAULT")
        .allowlist_var("SIMCONNECT_CLIENT_DATA_SET_FLAG_TAGGED")
        .allowlist_var("SIMCONNECT_VIEW_SYSTEM_EVENT_DATA_COCKPIT_2D")
        .allowlist_var("SIMCONNECT_VIEW_SYSTEM_EVENT_DATA_COCKPIT_VIRTUAL")
        .allowlist_var("SIMCONNECT_VIEW_SYSTEM_EVENT_DATA_ORTHOGONAL")
        .allowlist_var("SIMCONNECT_SOUND_SYSTEM_EVENT_DATA_MASTER")
        .allowlist_var("UNKNOWN_SENDID")
        .allowlist_var("UNKNOWN_INDEX")
        .allowlist_var("UNKNOWN_GROUP")
        .allowlist_var("SIMCONNECT_CLOUD_STATE_ARRAY_WIDTH")
        .allowlist_var("SIMCONNECT_CLOUD_STATE_ARRAY_SIZE")
        .allowlist_type("HANDLE")
        .allowlist_type("SIMCONNECT_RECV_ID")
        .allowlist_type("SIMCONNECT_DATATYPE")
        .allowlist_type("SIMCONNECT_EXCEPTION")
        .allowlist_type("SIMCONNECT_SIMOBJECT_TYPE")
        .allowlist_type("SIMCONNECT_STATE")
        .allowlist_type("SIMCONNECT_PERIOD")
        .allowlist_type("SIMCONNECT_MISSION_END")
        .allowlist_type("SIMCONNECT_CLIENT_DATA_PERIOD")
        .allowlist_type("SIMCONNECT_TEXT_TYPE")
        .allowlist_type("SIMCONNECT_TEXT_RESULT")
        .allowlist_type("SIMCONNECT_WEATHER_MODE")
        .allowlist_type("SIMCONNECT_FACILITY_LIST_TYPE")
        .allowlist_type("SIMCONNECT_RECV")
        .allowlist_type("SIMCONNECT_RECV_EXCEPTION")
        .allowlist_type("SIMCONNECT_RECV_OPEN")
        .allowlist_type("SIMCONNECT_RECV_QUIT")
        .allowlist_type("SIMCONNECT_RECV_EVENT")
        .allowlist_type("SIMCONNECT_RECV_EVENT_FILENAME")
        .allowlist_type("SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE")
        .allowlist_type("SIMCONNECT_RECV_EVENT_FRAME")
        .allowlist_type("SIMCONNECT_RECV_EVENT_MULTIPLAYER_SERVER_STARTED")
        .allowlist_type("SIMCONNECT_RECV_EVENT_MULTIPLAYER_CLIENT_STARTED")
        .allowlist_type("SIMCONNECT_RECV_EVENT_MULTIPLAYER_SESSION_ENDED")
        .allowlist_type("SIMCONNECT_RECV_EVENT_RACE_END")
        .allowlist_type("SIMCONNECT_RECV_EVENT_RACE_LAP")
        .allowlist_type("SIMCONNECT_RECV_SIMOBJECT_DATA")
        .allowlist_type("SIMCONNECT_RECV_SIMOBJECT_DATA_BYTYPE")
        .allowlist_type("SIMCONNECT_RECV_CLIENT_DATA")
        .allowlist_type("SIMCONNECT_RECV_WEATHER_OBSERVATION")
        .allowlist_type("SIMCONNECT_RECV_CLOUD_STATE")
        .allowlist_type("SIMCONNECT_RECV_ASSIGNED_OBJECT_ID")
        .allowlist_type("SIMCONNECT_RECV_RESERVED_KEY")
        .allowlist_type("SIMCONNECT_RECV_SYSTEM_STATE")
        .allowlist_type("SIMCONNECT_RECV_CUSTOM_ACTION")
        .allowlist_type("SIMCONNECT_RECV_EVENT_WEATHER_MODE")
        .allowlist_type("SIMCONNECT_RECV_FACILITIES_LIST")
        .allowlist_type("SIMCONNECT_DATA_FACILITY_AIRPORT")
        .allowlist_type("SIMCONNECT_RECV_AIRPORT_LIST")
        .allowlist_type("SIMCONNECT_DATA_FACILITY_WAYPOINT")
        .allowlist_type("SIMCONNECT_RECV_WAYPOINT_LIST")
        .allowlist_type("SIMCONNECT_DATA_FACILITY_NDB")
        .allowlist_type("SIMCONNECT_RECV_NDB_LIST")
        .allowlist_type("SIMCONNECT_DATA_FACILITY_VOR")
        .allowlist_type("SIMCONNECT_RECV_VOR_LIST")
        .allowlist_type("SIMCONNECT_RECV_PICK")
        .allowlist_function("SimConnect_MapClientEventToSimEvent")
        .allowlist_function("SimConnect_TransmitClientEvent")
        .allowlist_function("SimConnect_SetSystemEventState")
        .allowlist_function("SimConnect_AddClientEventToNotificationGroup")
        .allowlist_function("SimConnect_RemoveClientEvent")
        .allowlist_function("SimConnect_SetNotificationGroupPriority")
        .allowlist_function("SimConnect_ClearNotificationGroup")
        .allowlist_function("SimConnect_RequestNotificationGroup")
        .allowlist_function("SimConnect_AddToDataDefinition")
        .allowlist_function("SimConnect_ClearDataDefinition")
        .allowlist_function("SimConnect_RequestDataOnSimObject")
        .allowlist_function("SimConnect_RequestDataOnSimObjectType")
        .allowlist_function("SimConnect_SetDataOnSimObject")
        .allowlist_function("SimConnect_MapInputEventToClientEvent")
        .allowlist_function("SimConnect_SetInputGroupPriority")
        .allowlist_function("SimConnect_RemoveInputEvent")
        .allowlist_function("SimConnect_ClearInputGroup")
        .allowlist_function("SimConnect_SetInputGroupState")
        .allowlist_function("SimConnect_RequestReservedKey")
        .allowlist_function("SimConnect_SubscribeToSystemEvent")
        .allowlist_function("SimConnect_UnsubscribeFromSystemEvent")
        .allowlist_function("SimConnect_WeatherRequestInterpolatedObservation")
        .allowlist_function("SimConnect_WeatherRequestObservationAtStation")
        .allowlist_function("SimConnect_WeatherRequestObservationAtNearestStation")
        .allowlist_function("SimConnect_WeatherCreateStation")
        .allowlist_function("SimConnect_WeatherRemoveStation")
        .allowlist_function("SimConnect_WeatherSetObservation")
        .allowlist_function("SimConnect_WeatherSetModeServer")
        .allowlist_function("SimConnect_WeatherSetModeTheme")
        .allowlist_function("SimConnect_WeatherSetModeGlobal")
        .allowlist_function("SimConnect_WeatherSetModeCustom")
        .allowlist_function("SimConnect_WeatherSetDynamicUpdateRate")
        .allowlist_function("SimConnect_WeatherRequestCloudState")
        .allowlist_function("SimConnect_WeatherCreateThermal")
        .allowlist_function("SimConnect_WeatherRemoveThermal")
        .allowlist_function("SimConnect_AICreateParkedATCAircraft")
        .allowlist_function("SimConnect_AICreateEnrouteATCAircraft")
        .allowlist_function("SimConnect_AICreateNonATCAircraft")
        .allowlist_function("SimConnect_AICreateSimulatedObject")
        .allowlist_function("SimConnect_AIReleaseControl")
        .allowlist_function("SimConnect_AIRemoveObject")
        .allowlist_function("SimConnect_AISetAircraftFlightPlan")
        .allowlist_function("SimConnect_ExecuteMissionAction")
        .allowlist_function("SimConnect_CompleteCustomMissionAction")
        .allowlist_function("SimConnect_Close")
        .allowlist_function("SimConnect_RetrieveString")
        .allowlist_function("SimConnect_GetLastSentPacketID")
        .allowlist_function("SimConnect_Open")
        .allowlist_function("SimConnect_CallDispatch")
        .allowlist_function("SimConnect_GetNextDispatch")
        .allowlist_function("SimConnect_RequestResponseTimes")
        .allowlist_function("SimConnect_InsertString")
        .allowlist_function("SimConnect_CameraSetRelative6DOF")
        .allowlist_function("SimConnect_MenuAddItem")
        .allowlist_function("SimConnect_MenuDeleteItem")
        .allowlist_function("SimConnect_MenuAddSubItem")
        .allowlist_function("SimConnect_MenuDeleteSubItem")
        .allowlist_function("SimConnect_RequestSystemState")
        .allowlist_function("SimConnect_SetSystemState")
        .allowlist_function("SimConnect_MapClientDataNameToID")
        .allowlist_function("SimConnect_CreateClientData")
        .allowlist_function("SimConnect_AddToClientDataDefinition")
        .allowlist_function("SimConnect_ClearClientDataDefinition")
        .allowlist_function("SimConnect_RequestClientData")
        .allowlist_function("SimConnect_SetClientData")
        .allowlist_function("SimConnect_FlightLoad")
        .allowlist_function("SimConnect_FlightSave")
        .allowlist_function("SimConnect_FlightPlanLoad")
        .allowlist_function("SimConnect_Text")
        .allowlist_function("SimConnect_SubscribeToFacilities")
        .allowlist_function("SimConnect_UnsubscribeToFacilities")
        .allowlist_function("SimConnect_RequestFacilitiesList")
        .allowlist_type("SIMCONNECT_DATA_RACE_RESULT")
        .allowlist_type("SIMCONNECT_DATA_INITPOSITION")
        .allowlist_type("SIMCONNECT_DATA_MARKERSTATE")
        .allowlist_type("SIMCONNECT_DATA_WAYPOINT")
        .allowlist_type("SIMCONNECT_DATA_LATLONALT")
        .allowlist_type("SIMCONNECT_DATA_XYZ")
        .impl_debug(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}