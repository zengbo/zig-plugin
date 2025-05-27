use crate::zig_dist::ZigDist;
use extism_pdk::*;
use proto_pdk::*;
use std::collections::HashMap;

static NAME: &str = "Zig";

#[plugin_fn]
pub fn register_tool(Json(_): Json<RegisterToolInput>) -> FnResult<Json<RegisterToolOutput>> {
    Ok(Json(RegisterToolOutput {
        name: NAME.into(),
        type_of: PluginType::Language,
        minimum_proto_version: Some(Version::new(0, 47, 4)),
        plugin_version: Version::parse(env!("CARGO_PKG_VERSION")).ok(),
        ..RegisterToolOutput::default()
    }))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_host_environment()?;

    check_supported_os_and_arch(
        NAME,
        &env,
        permutations![
            HostOS::Linux => [HostArch::X86, HostArch::X64, HostArch::Arm64],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
            HostOS::Windows => [HostArch::X86, HostArch::X64, HostArch::Arm64],
        ],
    )?;

    // Always fetch the JSON response to get the actual download URLs
    let response: ZigDist = fetch_json("https://ziglang.org/download/index.json")?;

    let mut version = input.context.version;
    let is_canary = version.is_canary();
    if is_canary {
        version = VersionSpec::parse(&response.master.version)?;
    }

    // Get platform-specific information from the JSON response
    let platform_info = if is_canary {
        response.master.get_platform_info(env.os, env.arch)
    } else {
        // Look up the version in the response
        let version_str = version.to_string();
        response
            .versions
            .get(&version_str)
            .and_then(|v| v.get_platform_info(env.os, env.arch))
    };

    let platform_info = platform_info.ok_or_else(|| {
        Error::msg(format!(
            "No prebuilt binary available for {} on {}-{}",
            version, env.os, env.arch
        ))
    })?;

    // Extract filename from the tarball URL
    let download_url = &platform_info.tarball;
    let filename = download_url.split('/').last().unwrap_or("").to_string();

    // Generate archive prefix by removing the file extension
    let archive_prefix = if filename.ends_with(".zip") {
        filename.trim_end_matches(".zip").to_string()
    } else if filename.ends_with(".tar.xz") {
        filename.trim_end_matches(".tar.xz").to_string()
    } else {
        filename.clone()
    };

    Ok(Json(DownloadPrebuiltOutput {
        archive_prefix: Some(archive_prefix),
        checksum_url: Some(format!("{}.minisig", download_url)),
        checksum_public_key: Some(
            "RWSGOq2NVecA2UPNdBUZykf1CCb147pkmdtYxgb3Ti+JO/wCYvhbAb/U".into(),
        ),
        download_url: download_url.clone(),
        download_name: Some(filename),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;

    Ok(Json(LocateExecutablesOutput {
        exes: HashMap::from_iter([(
            "zig".into(),
            ExecutableConfig::new_primary(env.os.get_exe_name("zig")),
        )]),
        ..LocateExecutablesOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let response: ZigDist = fetch_json("https://ziglang.org/download/index.json")?;
    let mut versions: Vec<String> = response.versions.keys().map(|t| t.to_owned()).collect();
    versions.push(response.master.version.clone());

    let mut output = LoadVersionsOutput::from(versions)?;
    output.aliases.insert(
        "master".into(),
        UnresolvedVersionSpec::parse(&response.master.version)?,
    );

    Ok(Json(output))
}
