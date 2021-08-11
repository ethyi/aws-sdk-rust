// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_associate_configuration_items_to_application_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateConfigurationItemsToApplicationInput,
) {
    if let Some(var_1) = &input.application_configuration_id {
        object.key("applicationConfigurationId").string(var_1);
    }
    if let Some(var_2) = &input.configuration_ids {
        let mut array_3 = object.key("configurationIds").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4);
            }
        }
        array_3.finish();
    }
}

pub fn serialize_structure_batch_delete_import_data_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchDeleteImportDataInput,
) {
    if let Some(var_5) = &input.import_task_ids {
        let mut array_6 = object.key("importTaskIds").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7);
            }
        }
        array_6.finish();
    }
}

pub fn serialize_structure_create_application_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateApplicationInput,
) {
    if let Some(var_8) = &input.name {
        object.key("name").string(var_8);
    }
    if let Some(var_9) = &input.description {
        object.key("description").string(var_9);
    }
}

pub fn serialize_structure_create_tags_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateTagsInput,
) {
    if let Some(var_10) = &input.configuration_ids {
        let mut array_11 = object.key("configurationIds").start_array();
        for item_12 in var_10 {
            {
                array_11.value().string(item_12);
            }
        }
        array_11.finish();
    }
    if let Some(var_13) = &input.tags {
        let mut array_14 = object.key("tags").start_array();
        for item_15 in var_13 {
            {
                let mut object_16 = array_14.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_16, item_15);
                object_16.finish();
            }
        }
        array_14.finish();
    }
}

pub fn serialize_structure_delete_applications_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteApplicationsInput,
) {
    if let Some(var_17) = &input.configuration_ids {
        let mut array_18 = object.key("configurationIds").start_array();
        for item_19 in var_17 {
            {
                array_18.value().string(item_19);
            }
        }
        array_18.finish();
    }
}

pub fn serialize_structure_delete_tags_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteTagsInput,
) {
    if let Some(var_20) = &input.configuration_ids {
        let mut array_21 = object.key("configurationIds").start_array();
        for item_22 in var_20 {
            {
                array_21.value().string(item_22);
            }
        }
        array_21.finish();
    }
    if let Some(var_23) = &input.tags {
        let mut array_24 = object.key("tags").start_array();
        for item_25 in var_23 {
            {
                let mut object_26 = array_24.value().start_object();
                crate::json_ser::serialize_structure_tag(&mut object_26, item_25);
                object_26.finish();
            }
        }
        array_24.finish();
    }
}

pub fn serialize_structure_describe_agents_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAgentsInput,
) {
    if let Some(var_27) = &input.agent_ids {
        let mut array_28 = object.key("agentIds").start_array();
        for item_29 in var_27 {
            {
                array_28.value().string(item_29);
            }
        }
        array_28.finish();
    }
    if let Some(var_30) = &input.filters {
        let mut array_31 = object.key("filters").start_array();
        for item_32 in var_30 {
            {
                let mut object_33 = array_31.value().start_object();
                crate::json_ser::serialize_structure_filter(&mut object_33, item_32);
                object_33.finish();
            }
        }
        array_31.finish();
    }
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_34) = &input.next_token {
        object.key("nextToken").string(var_34);
    }
}

pub fn serialize_structure_describe_configurations_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeConfigurationsInput,
) {
    if let Some(var_35) = &input.configuration_ids {
        let mut array_36 = object.key("configurationIds").start_array();
        for item_37 in var_35 {
            {
                array_36.value().string(item_37);
            }
        }
        array_36.finish();
    }
}

pub fn serialize_structure_describe_continuous_exports_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeContinuousExportsInput,
) {
    if let Some(var_38) = &input.export_ids {
        let mut array_39 = object.key("exportIds").start_array();
        for item_40 in var_38 {
            {
                array_39.value().string(item_40);
            }
        }
        array_39.finish();
    }
    if let Some(var_41) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_41).into()),
        );
    }
    if let Some(var_42) = &input.next_token {
        object.key("nextToken").string(var_42);
    }
}

pub fn serialize_structure_describe_export_configurations_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeExportConfigurationsInput,
) {
    if let Some(var_43) = &input.export_ids {
        let mut array_44 = object.key("exportIds").start_array();
        for item_45 in var_43 {
            {
                array_44.value().string(item_45);
            }
        }
        array_44.finish();
    }
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_46) = &input.next_token {
        object.key("nextToken").string(var_46);
    }
}

pub fn serialize_structure_describe_export_tasks_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeExportTasksInput,
) {
    if let Some(var_47) = &input.export_ids {
        let mut array_48 = object.key("exportIds").start_array();
        for item_49 in var_47 {
            {
                array_48.value().string(item_49);
            }
        }
        array_48.finish();
    }
    if let Some(var_50) = &input.filters {
        let mut array_51 = object.key("filters").start_array();
        for item_52 in var_50 {
            {
                let mut object_53 = array_51.value().start_object();
                crate::json_ser::serialize_structure_export_filter(&mut object_53, item_52);
                object_53.finish();
            }
        }
        array_51.finish();
    }
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_54) = &input.next_token {
        object.key("nextToken").string(var_54);
    }
}

pub fn serialize_structure_describe_import_tasks_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeImportTasksInput,
) {
    if let Some(var_55) = &input.filters {
        let mut array_56 = object.key("filters").start_array();
        for item_57 in var_55 {
            {
                let mut object_58 = array_56.value().start_object();
                crate::json_ser::serialize_structure_import_task_filter(&mut object_58, item_57);
                object_58.finish();
            }
        }
        array_56.finish();
    }
    if let Some(var_59) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_59).into()),
        );
    }
    if let Some(var_60) = &input.next_token {
        object.key("nextToken").string(var_60);
    }
}

pub fn serialize_structure_describe_tags_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeTagsInput,
) {
    if let Some(var_61) = &input.filters {
        let mut array_62 = object.key("filters").start_array();
        for item_63 in var_61 {
            {
                let mut object_64 = array_62.value().start_object();
                crate::json_ser::serialize_structure_tag_filter(&mut object_64, item_63);
                object_64.finish();
            }
        }
        array_62.finish();
    }
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_65) = &input.next_token {
        object.key("nextToken").string(var_65);
    }
}

pub fn serialize_structure_disassociate_configuration_items_from_application_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisassociateConfigurationItemsFromApplicationInput,
) {
    if let Some(var_66) = &input.application_configuration_id {
        object.key("applicationConfigurationId").string(var_66);
    }
    if let Some(var_67) = &input.configuration_ids {
        let mut array_68 = object.key("configurationIds").start_array();
        for item_69 in var_67 {
            {
                array_68.value().string(item_69);
            }
        }
        array_68.finish();
    }
}

pub fn serialize_structure_list_configurations_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListConfigurationsInput,
) {
    if let Some(var_70) = &input.configuration_type {
        object.key("configurationType").string(var_70.as_str());
    }
    if let Some(var_71) = &input.filters {
        let mut array_72 = object.key("filters").start_array();
        for item_73 in var_71 {
            {
                let mut object_74 = array_72.value().start_object();
                crate::json_ser::serialize_structure_filter(&mut object_74, item_73);
                object_74.finish();
            }
        }
        array_72.finish();
    }
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_75) = &input.next_token {
        object.key("nextToken").string(var_75);
    }
    if let Some(var_76) = &input.order_by {
        let mut array_77 = object.key("orderBy").start_array();
        for item_78 in var_76 {
            {
                let mut object_79 = array_77.value().start_object();
                crate::json_ser::serialize_structure_order_by_element(&mut object_79, item_78);
                object_79.finish();
            }
        }
        array_77.finish();
    }
}

pub fn serialize_structure_list_server_neighbors_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListServerNeighborsInput,
) {
    if let Some(var_80) = &input.configuration_id {
        object.key("configurationId").string(var_80);
    }
    if input.port_information_needed {
        object
            .key("portInformationNeeded")
            .boolean(input.port_information_needed);
    }
    if let Some(var_81) = &input.neighbor_configuration_ids {
        let mut array_82 = object.key("neighborConfigurationIds").start_array();
        for item_83 in var_81 {
            {
                array_82.value().string(item_83);
            }
        }
        array_82.finish();
    }
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_84) = &input.next_token {
        object.key("nextToken").string(var_84);
    }
}

pub fn serialize_structure_start_data_collection_by_agent_ids_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartDataCollectionByAgentIdsInput,
) {
    if let Some(var_85) = &input.agent_ids {
        let mut array_86 = object.key("agentIds").start_array();
        for item_87 in var_85 {
            {
                array_86.value().string(item_87);
            }
        }
        array_86.finish();
    }
}

pub fn serialize_structure_start_export_task_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartExportTaskInput,
) {
    if let Some(var_88) = &input.export_data_format {
        let mut array_89 = object.key("exportDataFormat").start_array();
        for item_90 in var_88 {
            {
                array_89.value().string(item_90.as_str());
            }
        }
        array_89.finish();
    }
    if let Some(var_91) = &input.filters {
        let mut array_92 = object.key("filters").start_array();
        for item_93 in var_91 {
            {
                let mut object_94 = array_92.value().start_object();
                crate::json_ser::serialize_structure_export_filter(&mut object_94, item_93);
                object_94.finish();
            }
        }
        array_92.finish();
    }
    if let Some(var_95) = &input.start_time {
        object
            .key("startTime")
            .instant(var_95, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_96) = &input.end_time {
        object
            .key("endTime")
            .instant(var_96, smithy_types::instant::Format::EpochSeconds);
    }
}

pub fn serialize_structure_start_import_task_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartImportTaskInput,
) {
    if let Some(var_97) = &input.client_request_token {
        object.key("clientRequestToken").string(var_97);
    }
    if let Some(var_98) = &input.name {
        object.key("name").string(var_98);
    }
    if let Some(var_99) = &input.import_url {
        object.key("importUrl").string(var_99);
    }
}

pub fn serialize_structure_stop_continuous_export_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopContinuousExportInput,
) {
    if let Some(var_100) = &input.export_id {
        object.key("exportId").string(var_100);
    }
}

pub fn serialize_structure_stop_data_collection_by_agent_ids_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopDataCollectionByAgentIdsInput,
) {
    if let Some(var_101) = &input.agent_ids {
        let mut array_102 = object.key("agentIds").start_array();
        for item_103 in var_101 {
            {
                array_102.value().string(item_103);
            }
        }
        array_102.finish();
    }
}

pub fn serialize_structure_update_application_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateApplicationInput,
) {
    if let Some(var_104) = &input.configuration_id {
        object.key("configurationId").string(var_104);
    }
    if let Some(var_105) = &input.name {
        object.key("name").string(var_105);
    }
    if let Some(var_106) = &input.description {
        object.key("description").string(var_106);
    }
}

pub fn serialize_structure_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_107) = &input.key {
        object.key("key").string(var_107);
    }
    if let Some(var_108) = &input.value {
        object.key("value").string(var_108);
    }
}

pub fn serialize_structure_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Filter,
) {
    if let Some(var_109) = &input.name {
        object.key("name").string(var_109);
    }
    if let Some(var_110) = &input.values {
        let mut array_111 = object.key("values").start_array();
        for item_112 in var_110 {
            {
                array_111.value().string(item_112);
            }
        }
        array_111.finish();
    }
    if let Some(var_113) = &input.condition {
        object.key("condition").string(var_113);
    }
}

pub fn serialize_structure_export_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ExportFilter,
) {
    if let Some(var_114) = &input.name {
        object.key("name").string(var_114);
    }
    if let Some(var_115) = &input.values {
        let mut array_116 = object.key("values").start_array();
        for item_117 in var_115 {
            {
                array_116.value().string(item_117);
            }
        }
        array_116.finish();
    }
    if let Some(var_118) = &input.condition {
        object.key("condition").string(var_118);
    }
}

pub fn serialize_structure_import_task_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ImportTaskFilter,
) {
    if let Some(var_119) = &input.name {
        object.key("name").string(var_119.as_str());
    }
    if let Some(var_120) = &input.values {
        let mut array_121 = object.key("values").start_array();
        for item_122 in var_120 {
            {
                array_121.value().string(item_122);
            }
        }
        array_121.finish();
    }
}

pub fn serialize_structure_tag_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TagFilter,
) {
    if let Some(var_123) = &input.name {
        object.key("name").string(var_123);
    }
    if let Some(var_124) = &input.values {
        let mut array_125 = object.key("values").start_array();
        for item_126 in var_124 {
            {
                array_125.value().string(item_126);
            }
        }
        array_125.finish();
    }
}

pub fn serialize_structure_order_by_element(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OrderByElement,
) {
    if let Some(var_127) = &input.field_name {
        object.key("fieldName").string(var_127);
    }
    if let Some(var_128) = &input.sort_order {
        object.key("sortOrder").string(var_128.as_str());
    }
}