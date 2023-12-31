use crate::availability::Availability;
use crate::ast::config::Config;
use crate::ast::schema::Schema;
use crate::ast::source::Source;
use crate::completion::completion_item::CompletionItem;
use crate::completion::completion_item_from_top::completion_item_from_field;
use crate::traits::has_availability::HasAvailability;
use crate::traits::node_trait::NodeTrait;

pub(super) fn find_completion_in_config(schema: &Schema, _source: &Source, config: &Config, line_col: (usize, usize)) -> Vec<CompletionItem> {
    // let mut used: Vec<&str> = vec![];
    // used.extend(config.items().map(|i| i.identifier().name()));
    // used.extend(config.unattached_identifiers.iter().map(|i| i.name()));
    // for item in config.items() {
    //     if item.span().contains_line_col(line_col) {
    //         return find_completion_in_config_item(schema, item, line_col, &used);
    //     }
    // }
    // for unattached_identifier in &config.unattached_identifiers {
    //     if unattached_identifier.span.contains_line_col(line_col) {
    //         return collect_config_declaration_item_names(schema, config.keyword().name(), config.availability(), &used)
    //     }
    // }
    vec![]
}

fn collect_config_declaration_item_names(schema: &Schema, config_name: &str, availability: Availability, used: &Vec<&str>) -> Vec<CompletionItem> {
    let Some(config_declaration) = schema.find_config_declaration_by_name(config_name, availability) else {
        return vec![];
    };
    config_declaration.fields().filter(|f| !used.contains(&f.identifier().name())).map(|f| completion_item_from_field(f)).collect()
}