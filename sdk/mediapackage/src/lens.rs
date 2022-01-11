// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_channels_output_next_token(
    input: &crate::output::ListChannelsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_harvest_jobs_output_next_token(
    input: &crate::output::ListHarvestJobsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_origin_endpoints_output_next_token(
    input: &crate::output::ListOriginEndpointsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_channels_output_channels(
    input: crate::output::ListChannelsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Channel>> {
    let input = match input.channels {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_harvest_jobs_output_harvest_jobs(
    input: crate::output::ListHarvestJobsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::HarvestJob>> {
    let input = match input.harvest_jobs {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_origin_endpoints_output_origin_endpoints(
    input: crate::output::ListOriginEndpointsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::OriginEndpoint>> {
    let input = match input.origin_endpoints {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}