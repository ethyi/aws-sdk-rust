// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_describe_addon_versions_output_next_token(
    input: &crate::output::DescribeAddonVersionsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_addons_output_next_token(
    input: &crate::output::ListAddonsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_clusters_output_next_token(
    input: &crate::output::ListClustersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_fargate_profiles_output_next_token(
    input: &crate::output::ListFargateProfilesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_identity_provider_configs_output_next_token(
    input: &crate::output::ListIdentityProviderConfigsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_nodegroups_output_next_token(
    input: &crate::output::ListNodegroupsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_updates_output_next_token(
    input: &crate::output::ListUpdatesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_addon_versions_output_addons(
    input: crate::output::DescribeAddonVersionsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::AddonInfo>> {
    let input = match input.addons {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_addons_output_addons(
    input: crate::output::ListAddonsOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.addons {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_clusters_output_clusters(
    input: crate::output::ListClustersOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.clusters {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_fargate_profiles_output_fargate_profile_names(
    input: crate::output::ListFargateProfilesOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.fargate_profile_names {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_identity_provider_configs_output_identity_provider_configs(
    input: crate::output::ListIdentityProviderConfigsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::IdentityProviderConfig>> {
    let input = match input.identity_provider_configs {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_nodegroups_output_nodegroups(
    input: crate::output::ListNodegroupsOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.nodegroups {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_updates_output_update_ids(
    input: crate::output::ListUpdatesOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.update_ids {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}