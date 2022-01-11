// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_describe_job_log_items_output_next_token(
    input: &crate::output::DescribeJobLogItemsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_jobs_output_next_token(
    input: &crate::output::DescribeJobsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_recovery_instances_output_next_token(
    input: &crate::output::DescribeRecoveryInstancesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_recovery_snapshots_output_next_token(
    input: &crate::output::DescribeRecoverySnapshotsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_replication_configuration_templates_output_next_token(
    input: &crate::output::DescribeReplicationConfigurationTemplatesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_source_servers_output_next_token(
    input: &crate::output::DescribeSourceServersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_job_log_items_output_items(
    input: crate::output::DescribeJobLogItemsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::JobLog>> {
    let input = match input.items {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_jobs_output_items(
    input: crate::output::DescribeJobsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Job>> {
    let input = match input.items {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_recovery_instances_output_items(
    input: crate::output::DescribeRecoveryInstancesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::RecoveryInstance>> {
    let input = match input.items {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_recovery_snapshots_output_items(
    input: crate::output::DescribeRecoverySnapshotsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::RecoverySnapshot>> {
    let input = match input.items {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_replication_configuration_templates_output_items(
    input: crate::output::DescribeReplicationConfigurationTemplatesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ReplicationConfigurationTemplate>> {
    let input = match input.items {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_source_servers_output_items(
    input: crate::output::DescribeSourceServersOutput,
) -> std::option::Option<std::vec::Vec<crate::model::SourceServer>> {
    let input = match input.items {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}