// automatically generated by rust-bindgen
#[allow(dead_code)]
pub const VPSDK_VERSION: i32 = 4;

/**
 *	Events can be registered using #vp_event_set
 */
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum VpEvent {
    VpEventChat = 0,
    VpEventAvatarAdd = 1,
    VpEventAvatarChange = 2,
    VpEventAvatarDelete = 3,
    VpEventObject = 4,
    VpEventObjectChange = 5,
    VpEventObjectDelete = 6,
    VpEventObjectClick = 7,
    VpEventWorldList = 8,
    VpEventWorldSetting = 9,
    VpEventWorldSettingsChanged = 10,
    VpEventFriend = 11,
    VpEventWorldDisconnect = 12,
    VpEventUniverseDisconnect = 13,
    VpEventUserAttributes = 14,
    VpEventCellEnd = 15,
    VpEventTerrainNode = 16,
    VpEventAvatarClick = 17,
    VpEventTeleport = 18,
    VpEventUrl = 19,
    VpEventObjectBumpBegin = 20,
    VpEventObjectBumpEnd = 21,
    VpEventTerrainNodeChanged = 22,
    VpEventJoin = 23,
    VpHighestEvent = 24,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum VpCallback {
    VpCallbackObjectAdd = 0,
    VpCallbackObjectChange = 1,
    VpCallbackObjectDelete = 2,
    VpCallbackGetFriends = 3,
    VpCallbackFriendAdd = 4,
    VpCallbackFriendDelete = 5,
    VpCallbackTerrainQuery = 6,
    VpCallbackTerrainNodeSet = 7,
    VpCallbackObjectGet = 8,
    VpCallbackObjectLoad = 9,
    VpCallbackLogin = 10,
    VpCallbackEnter = 11,
    VpCallbackJoin = 12,
    VpCallbackConnectUniverse = 13,
    VpCallbackWorldPermissionUserSet = 14,
    VpCallbackWorldPermissionSessionSet = 15,
    VpCallbackWorldSettingSet = 16,
    VpHighestCallback = 17,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum VpIntAttributes {
    VpAvatarSession = 0,
    VpAvatarType = 1,
    VpMyType = 2,
    VpObjectId = 3,
    VpObjectType = 4,
    VpObjectTime = 5,
    VpObjectUserId = 6,
    VpWorldState = 7,
    VpWorldUsers = 8,
    VpReferenceNumber = 9,
    VpCallback = 10,
    VpUserId = 11,
    VpUserRegistrationTime = 12,
    VpUserOnlineTime = 13,
    VpUserLastLogin = 14,
    VpFriendId = 15,
    VpFriendUserId = 16,
    VpFriendOnline = 17,
    VpMyUserId = 18,
    VpProxyType = 19,
    VpProxyPort = 20,
    VpCellX = 21,
    VpCellZ = 22,
    VpTerrainTileX = 23,
    VpTerrainTileZ = 24,
    VpTerrainNodeX = 25,
    VpTerrainNodeZ = 26,
    VpTerrainNodeRevision = 27,
    VpClickedSession = 28,
    VpChatType = 29,
    VpChatColorRed = 30,
    VpChatColorGreen = 31,
    VpChatColorBlue = 32,
    VpChatEffects = 33,
    VpDisconnectErrorCode = 34,
    VpUrlTarget = 35,
    VpCurrentEvent = 36,
    VpCurrentCallback = 37,
    VpCellRevision = 38,
    VpCellStatus = 39,
    VpJoinId = 40,
    VpHighestInt = 41,
}

#[allow(dead_code)]
pub const VP_OBJECT_YAW: VpFloatAttribute =
    VpFloatAttribute::VpObjectRotationX;

#[allow(dead_code)]
pub const VP_OBJECT_PITCH: VpFloatAttribute =
    VpFloatAttribute::VpObjectRotationY;

#[allow(dead_code)]
pub const VP_OBJECT_ROLL: VpFloatAttribute =
    VpFloatAttribute::VpObjectRotationZ;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum VpFloatAttribute {
    VpAvatarX = 0,
    VpAvatarY = 1,
    VpAvatarZ = 2,
    VpAvatarYaw = 3,
    VpAvatarPitch = 4,
    VpMyX = 5,
    VpMyY = 6,
    VpMyZ = 7,
    VpMyYaw = 8,
    VpMyPitch = 9,
    VpObjectX = 10,
    VpObjectY = 11,
    VpObjectZ = 12,
    VpObjectRotationX = 13,
    VpObjectRotationY = 14,
    VpObjectRotationZ = 15,
    VpObjectRotationAngle = 16,
    VpTeleportX = 17,
    VpTeleportY = 18,
    VpTeleportZ = 19,
    VpTeleportYaw = 20,
    VpTeleportPitch = 21,
    VpClickHitX = 22,
    VpClickHitY = 23,
    VpClickHitZ = 24,
    VpJoinX = 25,
    VpJoinY = 26,
    VpJoinZ = 27,
    VpJoinYaw = 28,
    VpJoinPitch = 29,
    VpHighestFloat = 30,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum VpStringAttribute {
    VpAvatarName = 0,
    VpChatMessage = 1,
    VpObjectModel = 2,
    VpObjectAction = 3,
    VpObjectDescription = 4,
    VpWorldName = 5,
    VpUserName = 6,
    VpUserEmail = 7,
    VpWorldSettingKey = 8,
    VpWorldSettingValue = 9,
    VpFriendName = 10,
    VpProxyHost = 11,
    VpTeleportWorld = 12,
    VpUrl = 13,
    VpJoinWorld = 14,
    VpJoinName = 15,
    VpStartWorld = 16,
    VpHighestString = 17,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum VpDataAttribute {
    VpObjectData = 0,
    VpTerrainNodeData = 1,
    VpHighestData = 2,
}

/**
 *	Proxy types
 */
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum VpProxyType {
    VpProxyTypeNone = 0,
    VpProxyTypeSocks4a = 1,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum VpUrlTarget {
    VpUrlTargetBrowser = 0,
    VpUrlTargetOverlay = 1,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum VpCellStatus {
    VpCellStatusModified = 0,
    VpCellStatusNotModified = 1,
    VpCellStatusError = 2,
}

/**
 *  Chat message types. 
 */
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum VpChatType {
    VpChatNormal = 0,
    VpChatConsoleMessage = 1,
    VpChatPrivate = 2,
}

/**
 *  Text effect flags. Can be combined with bitwise OR operator.
 */
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum VpTextEffect {
    VpTextEffectBold = 1,
    VpTextEffectItalic = 2,
}

pub type VpInstance = *mut ::std::os::raw::c_void;
pub type VpEventHandler = ::std::option::Option<unsafe extern "C" fn(arg1: VpInstance)>;
pub type VpCallbackHandler =
    ::std::option::Option<unsafe extern "C" fn(arg1: VpInstance,
                                                 arg2: ::std::os::raw::c_int,
                                                 arg3: ::std::os::raw::c_int)>;

#[repr(C)]
#[derive(Debug, Copy)]
#[allow(dead_code)]
pub struct VpTerrainCell {
    pub height: f32,
    pub attributes: ::std::os::raw::c_ushort,
}

#[test]
fn bindgen_test_layout_VpTerrainCell() {
    assert_eq!(::std::mem::size_of::<VpTerrainCell>(), 8usize);
    assert_eq!(::std::mem::align_of::<VpTerrainCell>(), 4usize);
}

impl Clone for VpTerrainCell {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
#[derive(Debug, Copy)]
pub struct VpNetConfig {
    pub _address: u8,
}
impl Clone for VpNetConfig {
    fn clone(&self) -> Self { *self }
}

extern "C" {
    pub fn vp_init(version: ::std::os::raw::c_int) -> ::std::os::raw::c_int; 

    /**
    *  Create a new instance.
    *  \param net_config network configuration for the instance or NULL to use 
    *                    the default select-based implementation. The contents of 
    *                    the structure will be copied.
    *  \return New instance or NULL on failure.
    */
    pub fn vp_create(net_config: *const VpNetConfig) -> VpInstance;

    ///  Destroy a Virtual Paradise SDK instance.
    ///
    pub fn vp_destroy(instance: VpInstance) -> ::std::os::raw::c_int;

    ///  Connect to a universe server
    ///  \param instance
    ///  \param host Host address of server to connect to.
    ///  \param port TCP port of remote server.
    ///  \returns #VpRcSuccess
    ///  \returns #VpRcConnectionError
    ///
    pub fn vp_connect_universe(instance: VpInstance,
                               host: *const ::std::os::raw::c_char,
                               port: ::std::os::raw::c_int)
                               -> ::std::os::raw::c_int;

    ///  Login to the universe server
    ///  \param instance
    ///  \param username
    ///  \param password
    ///  \param botname
    ///  \returns #VpRcSuccess
    ///  \returns #VpRcStringTooLong
    ///  \returns #VpRcInvalidLogin
    ///  \returns #VpRcTimeout
    ///  \returns #VpRcNotInUniverse
    ///
    pub fn vp_login(instance: VpInstance,
                    username: *const ::std::os::raw::c_char,
                    password: *const ::std::os::raw::c_char,
                    botname: *const ::std::os::raw::c_char)
                    -> ::std::os::raw::c_int;

    ///  Process incoming and outgoing data. Waits for connection to be ready for
    ///  sending or receiving. This function needs to be called for events to fire.
    ///  \param milliseconds The maximum time to wait in milliseconds.
    ///  \warning Not reentrant when used with the same instance, may not be called from event handlers unless it is for a different instance.
    ///  \return Zero when successful, otherwise nonzero. See RC.h
    ///
    pub fn vp_wait(instance: VpInstance,
                   milliseconds: ::std::os::raw::c_int)
                   -> ::std::os::raw::c_int;

    ///  Enter a world, the current world will be left.
    ///  \warning This function uses #vp_wait internally, the same warnings apply.
    ///  \returns #VpRcSuccess if successful
    ///  \returns #VpRcStringTooLong
    ///  \returns #VpRcConnectionError
    ///  \returns #VpRcWorldNotFound
    ///  \returns #VpRcWorldLoginError
    ///  \returns #VpRcTimeout
    ///  \returns #VpRcNotInUniverse
    ///
    pub fn vp_enter(instance: VpInstance,
                    worldname: *const ::std::os::raw::c_char)
                    -> ::std::os::raw::c_int;

    /// 	Leave the current world.
    ///  \returns #VpRcSuccess
    ///  \returns #VpRcNotInWorld
    ///
    pub fn vp_leave(instance: VpInstance) -> ::std::os::raw::c_int;

    ///  Send a simple message to everyone in the current world.
    ///  \param message The message to send.
    ///  \returns #VpRcSuccess
    ///  \returns #VpRcNotInWorld
    ///  \returns #VpRcStringTooLong
    ///
    pub fn vp_say(instance: VpInstance,
                  message: *const ::std::os::raw::c_char)
                  -> ::std::os::raw::c_int;

    ///  Send a console message.
    ///  \param session The session ID to send the message to. Zero to send to everyone
    ///  \param name The name to use for the chat message. Empty string to hide name.
    ///  \param message Chat message contents
    ///  \param effects Text effects (combination of #VPTextEffect flags)
    ///  \param red Red component of the text color(0-255)
    ///  \param green Green component of the text color(0-255)
    ///  \param blue Blue component of the text color(0-255)
    ///  \returns #VpRcSuccess
    ///  \returns #VpRcNotInWorld
    ///  \returns #VpRcStringTooLong
    ///
    pub fn _vp_console_message(instance: VpInstance,
                              session: ::std::os::raw::c_int,
                              name: *const ::std::os::raw::c_char,
                              message: *const ::std::os::raw::c_char,
                              effects: ::std::os::raw::c_int,
                              red: ::std::os::raw::c_uchar,
                              green: ::std::os::raw::c_uchar,
                              blue: ::std::os::raw::c_uchar)
                              -> ::std::os::raw::c_int;

    ///  Register an event handler.
    ///  \return Zero when successful, otherwise nonzero. See RC.h
    ///
    pub fn _vp_event_set(instance: VpInstance,
                        eventname: VpEvent,
                        event: VpEventHandler)
                        -> ::std::os::raw::c_int;

    ///  Register a callback function.
    ///  \return Zero when successful, otherwise nonzero. See RC.h
    ///
    pub fn _vp_callback_set(instance: VpInstance,
                           callbackname: VpCallback,
                           callback: VpCallbackHandler)
                           -> ::std::os::raw::c_int;

    ///  Retrieve the pointer to user-defined data for this instance.
    ///  \return Pointer to user-defined data.
    ///
    pub fn _vp_user_data(instance: VpInstance) -> *mut ::std::os::raw::c_void;

    ///  Sets a pointer to user-defined data for this instance.
    ///  This pointer is not accessed in any way, allocating and freeing it is the responsibility of the application programmer.
    ///  \param data The pointer to your user-defined data.
    ///
    pub fn _vp_user_data_set(instance: VpInstance, data: *mut ::std::os::raw::c_void);

    ///  Update avatar
    ///
    ///  Attributes:
    ///  - #VpMyX
    ///  - #VpMyY
    ///  - #VpMyZ
    ///  - #VpMyYaw
    ///  - #VpMyPitch
    ///  - #VpMyType
    ///  \return #VpRcSuccess
    ///  \return #VpRcNotInWorld
    ///
    pub fn vp_state_change(instance: VpInstance) -> ::std::os::raw::c_int;

    pub fn _vp_int(instance: VpInstance, attr: VpIntAttributes) -> ::std::os::raw::c_int;

    pub fn _vp_float(instance: VpInstance, attr: VpFloatAttribute) -> f32;

    pub fn _vp_double(instance: VpInstance, attr: VpFloatAttribute) -> f64;

    pub fn _vp_string(instance: VpInstance,
                     attr: VpStringAttribute)
                     -> *const ::std::os::raw::c_char;

    pub fn _vp_data(instance: VpInstance,
                   attr: VpDataAttribute,
                   length: *mut ::std::os::raw::c_int)
                   -> *const ::std::os::raw::c_char;

    pub fn _vp_int_get(instance: VpInstance,
                      attr: VpIntAttributes,
                      value: *mut ::std::os::raw::c_int)
                      -> ::std::os::raw::c_int;

    pub fn _vp_float_get(instance: VpInstance,
                        attr: VpFloatAttribute,
                        value: *mut f32)
                        -> ::std::os::raw::c_int;

    pub fn _vp_double_get(instance: VpInstance,
                         attr: VpFloatAttribute,
                         value: *mut f64)
                         -> ::std::os::raw::c_int;

    pub fn _vp_string_get(instance: VpInstance,
                         attr: VpStringAttribute,
                         value: *mut *mut ::std::os::raw::c_char)
                         -> ::std::os::raw::c_int;

    pub fn _vp_int_set(instance: VpInstance,
                      name: VpIntAttributes,
                      value: ::std::os::raw::c_int)
                      -> ::std::os::raw::c_int;

    pub fn _vp_float_set(instance: VpInstance,
                        name: VpFloatAttribute,
                        value: f32)
                        -> ::std::os::raw::c_int;

    pub fn _vp_double_set(instance: VpInstance,
                         attr: VpFloatAttribute,
                         value: f64)
                         -> ::std::os::raw::c_int;

    pub fn _vp_string_set(instance: VpInstance,
                         name: VpStringAttribute,
                         str: *const ::std::os::raw::c_char);

    pub fn _vp_data_set(instance: VpInstance,
                       name: VpDataAttribute,
                       length: ::std::os::raw::c_int,
                       data: *const ::std::os::raw::c_char)
                       -> ::std::os::raw::c_int;

    /// 	Query the objects in a single cell
    /// 	Each object will be sent in a #VpEventObject event. After all the objects
    ///  for the cell have been sent, a #VpEventCellEnd event will be raised.
    ///
    pub fn _vp_query_cell(instance: VpInstance,
                         x: ::std::os::raw::c_int,
                         z: ::std::os::raw::c_int)
                         -> ::std::os::raw::c_int;

    pub fn _vp_query_cell_revision(instance: VpInstance,
                                  x: ::std::os::raw::c_int,
                                  z: ::std::os::raw::c_int,
                                  revision: ::std::os::raw::c_int)
                                  -> ::std::os::raw::c_int;

    ///  Builds a new object
    ///
    ///  Uses attributes:
    ///  - #VpObjectType
    ///  - #VpObjectX
    ///  - #VpObjectY
    ///  - #VpObjectZ
    ///  - #VpObjectRotationX
    ///  - #VpObjectRotationY
    ///  - #VpObjectRotationZ
    ///  - #VpObjectRotationAngle
    ///  - #VpObjectModel
    ///  - #VpObjectAction
    ///  - #VpObjectDescription
    ///  - #VpObjectData
    ///
    pub fn _vp_object_add(instance: VpInstance) -> ::std::os::raw::c_int;

    pub fn _vp_object_load(instance: VpInstance) -> ::std::os::raw::c_int;

    ///  Send a object contact begin event to other users in the world
    ///  \param object_id
    ///  \param session_to Session ID to send a bump event to, or 0 to send to everyone
    ///
    pub fn _vp_object_bump_begin(instance: VpInstance,
                                object_id: ::std::os::raw::c_int,
                                session_to: ::std::os::raw::c_int)
                                -> ::std::os::raw::c_int;

    ///  Send a object contact end event to other users in the world
    ///  \param object_id
    ///  \param session_to Session ID to send a bump event to, or 0 to send to everyone
    ///
    pub fn _vp_object_bump_end(instance: VpInstance,
                              object_id: ::std::os::raw::c_int,
                              session_to: ::std::os::raw::c_int)
                              -> ::std::os::raw::c_int;

    ///  Changes an existing object
    ///
    ///  Uses attributes:
    ///  - #VpObjectId
    ///  - #VpObjectType
    ///  - #VpObjectX
    ///  - #VpObjectY
    ///  - #VpObjectZ
    ///  - #VpObjectRotationX
    ///  - #VpObjectRotationY
    ///  - #VpObjectRotationZ
    ///  - #VpObjectRotationAngle
    ///  - #VpObjectModel
    ///  - #VpObjectAction
    ///  - #VpObjectDescription
    ///  - #VpObjectData
    ///
    pub fn _vp_object_change(instance: VpInstance) -> ::std::os::raw::c_int;

    ///  Sends an object click event to other users in the world.
    ///  \param object_id            Object ID of the clicked object
    ///  \param session_to           Target session, or 0 to send to everyone
    ///  \param hit_x,hit_y,hit_z    Position where the object was hit
    ///  \returns #VpRcSuccess
    ///  \returns #VpRcNotInWorld
    ///
    pub fn _vp_object_click(instance: VpInstance,
                           object_id: ::std::os::raw::c_int,
                           session_to: ::std::os::raw::c_int,
                           hit_x: f32,
                           hit_y: f32,
                           hit_z: f32)
                           -> ::std::os::raw::c_int;

    ///  Delete an object
    ///  \param object_id ID of the object to be deleted
    ///
    pub fn _vp_object_delete(instance: VpInstance,
                            object_id: ::std::os::raw::c_int)
                            -> ::std::os::raw::c_int;

    ///  Request the attributes of a single object. The result will be returned in
    ///  the #VpCallbackObjectGet callback.
    ///
    pub fn _vp_object_get(instance: VpInstance,
                         object_id: ::std::os::raw::c_int)
                         -> ::std::os::raw::c_int;

    ///  Request the world list.
    ///  The worlds will be listed in the #VpEventWorldList event. See vp_event_set().
    ///  \param time Time since your last update. This is not used yet, the whole list will be requested.
    ///
    pub fn _vp_world_list(instance: VpInstance,
                         time: ::std::os::raw::c_int)
                         -> ::std::os::raw::c_int;

    ///  Request user attributes by user ID.
    ///  The user attributes will be returned in the #VpEventUserAttributes event.
    ///  \return Zero when successful, otherwise nonzero
    ///
    pub fn _vp_user_attributes_by_id(instance: VpInstance,
                                    user_id: ::std::os::raw::c_int)
                                    -> ::std::os::raw::c_int;

    ///  Get user attributes by user name. Not implemented.
    ///  \returns #VpRcNotImplemented
    ///
    pub fn _vp_user_attributes_by_name(instance: VpInstance,
                                      name: *const ::std::os::raw::c_char)
                                      -> ::std::os::raw::c_int;

    pub fn _vp_friends_get(instance: VpInstance) -> ::std::os::raw::c_int;

    pub fn _vp_friend_add_by_name(instance: VpInstance,
                                 name: *const ::std::os::raw::c_char)
                                 -> ::std::os::raw::c_int;

    pub fn _vp_friend_delete(instance: VpInstance,
                            friend_user_id: ::std::os::raw::c_int)
                            -> ::std::os::raw::c_int;

    ///  Query a terrain tile
    ///  \param  tile_x
    ///  \param  tile_z
    ///  \param  revision 16 node revision numbers (4x4: revision[z][x])
    ///  \return #VpRcSuccess
    ///  \return #VpRcNotInWorld
    ///
    pub fn _vp_terrain_query(instance: VpInstance,
                            tile_x: ::std::os::raw::c_int,
                            tile_z: ::std::os::raw::c_int,
                            revision: *mut [::std::os::raw::c_int; 4usize])
                            -> ::std::os::raw::c_int;

    ///  Replace terrain node data
    ///  \param  tile_x
    ///  \param  tile_z
    ///  \param  node_x
    ///  \param  node_z
    ///  \param  cells Pointer to 64 cells (8x8: cells[z*8+x])
    ///  \return #VpRcSuccess
    ///  \return #VpRcNotInWorld
    ///
    pub fn _vp_terrain_node_set(instance: VpInstance,
                               tile_x: ::std::os::raw::c_int,
                               tile_z: ::std::os::raw::c_int,
                               node_x: ::std::os::raw::c_int,
                               node_z: ::std::os::raw::c_int,
                               cells: *mut VpTerrainCell)
                               -> ::std::os::raw::c_int;

    ///  Send an avatar click event to other users in the world.
    ///  Uses the following attributes:
    ///  - #VpClickHitX
    ///  - #VpClickHitY
    ///  - #VpClickHitZ
    ///
    ///  \param      avatar_session The session id of the clicked avatar
    ///  \returns    #VpRcSuccess
    ///  \returns    #VpRcNotInWorld
    ///
    pub fn _vp_avatar_click(instance: VpInstance,
                           avatar_session: ::std::os::raw::c_int)
                           -> ::std::os::raw::c_int;

    ///  Request that another avatar teleports to a new location.
    ///  \note this is only a request and receiving client can choose to ignore it
    ///  \param target_session   session ID of the avatar to teleport
    ///  \param world            destination world, empty string to teleport avatar in current world
    ///  \param x,y,z            avatar position
    ///  \param yaw,pitch        avatar rotation
    ///  \returns                #VpRcSuccess
    ///  \returns                #VpRcNotInWorld (this is about the bot instance, not the avatar to be teleported)
    ///
    pub fn _vp_teleport_avatar(instance: VpInstance,
                              target_session: ::std::os::raw::c_int,
                              world: *const ::std::os::raw::c_char,
                              x: f32,
                              y: f32,
                              z: f32,
                              yaw: f32,
                              pitch: f32)
                              -> ::std::os::raw::c_int;

    ///  Send a URL to a user
    ///  \param session_id   target session id
    ///  \param url          the URL to send to the target session ID
    ///  \param url_target   Where to open the URL, see #VpUrlTarget
    ///
    pub fn _vp_url_send(instance: VpInstance,
                       session_id: ::std::os::raw::c_int,
                       url: *const ::std::os::raw::c_char,
                       url_target: VpUrlTarget)
                       -> ::std::os::raw::c_int;

    pub fn _vp_join(instance: VpInstance,
                   user_id: ::std::os::raw::c_int)
                   -> ::std::os::raw::c_int;

    pub fn _vp_join_accept(instance: VpInstance,
                          requestId: ::std::os::raw::c_int,
                          world: *const ::std::os::raw::c_char,
                          x: f64,
                          y: f64,
                          z: f64,
                          yaw: f32,
                          pitch: f32)
                          -> ::std::os::raw::c_int;

    pub fn _vp_join_decline(instance: VpInstance,
                           requestId: ::std::os::raw::c_int)
                           -> ::std::os::raw::c_int;

    pub fn _vp_world_permission_user_set(instance: VpInstance,
                                        permission: *const ::std::os::raw::c_char,
                                        user_id: ::std::os::raw::c_int,
                                        enable: ::std::os::raw::c_int)
                                        -> ::std::os::raw::c_int;

    pub fn _vp_world_permission_session_set(instance: VpInstance,
                                           permission: *const ::std::os::raw::c_char,
                                           session_id: ::std::os::raw::c_int,
                                           enable: ::std::os::raw::c_int)
                                           -> ::std::os::raw::c_int;

    pub fn _vp_world_setting_set(instance: VpInstance,
                                setting: *const ::std::os::raw::c_char,
                                value: *const ::std::os::raw::c_char,
                                session_to: ::std::os::raw::c_int)
                                -> ::std::os::raw::c_int;
}
