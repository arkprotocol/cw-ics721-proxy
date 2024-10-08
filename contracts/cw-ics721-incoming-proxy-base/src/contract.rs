use cosmwasm_schema::cw_serde;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use cw_ics721_incoming_proxy::IncomingProxyQuery;
use cw_ics721_incoming_proxy::{IncomingProxyError, IncomingProxyExecute};

use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

const CONTRACT_NAME: &str = "crates.io:cw721-incoming-proxy";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cw_serde]
#[derive(Default)]
pub struct IncomingProxyContract {}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    IncomingProxyContract::default().initialize(
        deps.storage,
        deps.api,
        msg.origin.clone(),
        msg.channels.clone(),
    )?;

    Ok(Response::default()
        .add_attribute("method", "instantiate")
        .add_attribute(
            "origin",
            msg.origin.map_or("empty".to_string(), |o| o.to_string()),
        )
        .add_attribute(
            "channels",
            msg.channels.map_or("epmty".to_string(), |sc| {
                if sc.is_empty() {
                    "empty".to_string()
                } else {
                    sc.join(",")
                }
            }),
        ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, IncomingProxyError> {
    match msg {
        ExecuteMsg::Ics721ReceivePacketMsg { data, packet } => IncomingProxyContract::default()
            .execute_ics721_receive_packet_msg(deps.storage, &info, packet, data),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOrigin {} => {
            to_json_binary(&IncomingProxyContract::default().get_origin(deps.storage)?)
        }
        QueryMsg::GetChannels { start_after, limit } => to_json_binary(
            &IncomingProxyContract::default().get_channels(deps, start_after, limit)?,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> StdResult<Response> {
    match msg {
        MigrateMsg::WithUpdate { origin, channels } => {
            IncomingProxyContract::default().migrate(deps.storage, deps.api, origin, channels)
        }
    }
}
