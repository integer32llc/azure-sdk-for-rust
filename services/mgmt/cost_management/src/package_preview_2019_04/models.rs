#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorBase {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorBase>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(rename = "eTag", skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigDefinition {
    #[serde(rename = "type")]
    pub type_: report_config_definition::Type,
    pub timeframe: report_config_definition::Timeframe,
    #[serde(rename = "timePeriod", skip_serializing_if = "Option::is_none")]
    pub time_period: Option<ReportConfigTimePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<ReportConfigDataset>,
}
pub mod report_config_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Usage,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Timeframe {
        WeekToDate,
        MonthToDate,
        YearToDate,
        Custom,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigTimePeriod {
    pub from: String,
    pub to: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigDataset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<report_config_dataset::Granularity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ReportConfigDatasetConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub grouping: Vec<ReportConfigGrouping>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sorting: Vec<ReportConfigSorting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ReportConfigFilter>,
}
pub mod report_config_dataset {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Granularity {
        Daily,
        Monthly,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigDatasetConfiguration {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigAggregation {
    pub name: String,
    pub function: report_config_aggregation::Function,
}
pub mod report_config_aggregation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Function {
        Sum,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigSorting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<report_config_sorting::Direction>,
    pub name: String,
}
pub mod report_config_sorting {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Direction {
        Ascending,
        Descending,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigGrouping {
    #[serde(rename = "type")]
    pub type_: ReportConfigColumnType,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigFilter {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub and: Vec<ReportConfigFilter>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub or: Vec<ReportConfigFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<ReportConfigFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension: Option<ReportConfigComparisonExpression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<ReportConfigComparisonExpression>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReportConfigColumnType {
    Tag,
    Dimension,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigComparisonExpression {
    pub name: String,
    pub operator: report_config_comparison_expression::Operator,
    pub values: Vec<String>,
}
pub mod report_config_comparison_expression {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        In,
        Contains,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing)]
        pub provider: Option<String>,
        #[serde(skip_serializing)]
        pub resource: Option<String>,
        #[serde(skip_serializing)]
        pub operation: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Scope {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "childScope", skip_serializing_if = "Option::is_none")]
    pub child_scope: Option<Scope>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ViewListResult {
    #[serde(skip_serializing)]
    pub value: Vec<View>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct View {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ViewProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ViewProperties {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "createdOn", skip_serializing)]
    pub created_on: Option<String>,
    #[serde(rename = "modifiedOn", skip_serializing)]
    pub modified_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<ReportConfigDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart: Option<view_properties::Chart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulated: Option<view_properties::Accumulated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<view_properties::Metric>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub kpis: Vec<KpiProperties>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pivots: Vec<PivotProperties>,
}
pub mod view_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Chart {
        Area,
        Line,
        StackedColumn,
        GroupedColumn,
        Table,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Accumulated {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Metric {
        ActualCost,
        AmortizedCost,
        #[serde(rename = "AHUB")]
        Ahub,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KpiProperties {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<kpi_properties::Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
pub mod kpi_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Forecast,
        Budget,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PivotProperties {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<pivot_properties::Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
pub mod pivot_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Dimension,
        TagKey,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetsListResult {
    #[serde(skip_serializing)]
    pub value: Vec<BudgetModel>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetModel {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BudgetProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetProperties {
    pub category: budget_properties::Category,
    pub amount: f64,
    #[serde(rename = "timeGrain")]
    pub time_grain: budget_properties::TimeGrain,
    #[serde(rename = "timePeriod")]
    pub time_period: BudgetTimePeriod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ReportConfigFilter>,
    #[serde(rename = "currentSpend", skip_serializing_if = "Option::is_none")]
    pub current_spend: Option<CurrentSpend>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<serde_json::Value>,
}
pub mod budget_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Category {
        Cost,
        Usage,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TimeGrain {
        Monthly,
        Quarterly,
        Annually,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetTimePeriod {
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentSpend {
    #[serde(skip_serializing)]
    pub amount: Option<f64>,
    #[serde(skip_serializing)]
    pub unit: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    pub enabled: bool,
    pub operator: notification::Operator,
    pub threshold: f64,
    #[serde(rename = "contactEmails")]
    pub contact_emails: Vec<String>,
    #[serde(rename = "contactRoles", skip_serializing_if = "Vec::is_empty")]
    pub contact_roles: Vec<String>,
    #[serde(rename = "contactGroups", skip_serializing_if = "Vec::is_empty")]
    pub contact_groups: Vec<String>,
}
pub mod notification {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        EqualTo,
        GreaterThan,
        GreaterThanOrEqualTo,
    }
}